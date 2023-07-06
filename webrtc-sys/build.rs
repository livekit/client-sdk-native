use regex::Regex;
use reqwest::StatusCode;
use std::env;
use std::fs;
use std::io::BufRead;
use std::io::{self, Write};
use std::path;
use std::process::Command;

const WEBRTC_TAG: &str = "webrtc-4a9b827";
const IGNORE_DEFINES: [&str; 2] = ["CR_CLANG_REVISION", "CR_XCODE_VERSION"];

fn target_os() -> String {
    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap();
    let target = env::var("TARGET").unwrap();

    let is_simulator = target.ends_with("-sim");

    match target_os.as_str() {
        "windows" => "win",
        "macos" => "mac",
        "ios" => {
            if is_simulator {
                "ios-simulator"
            } else {
                "ios-device"
            }
        }
        _ => &target_os,
    }
    .to_string()
}

fn target_arch() -> String {
    let target_arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap();
    match target_arch.as_str() {
        "aarch64" => "arm64",
        "x86_64" => "x64",
        _ => &target_arch,
    }
    .to_owned()
}

fn download_prebuilt_webrtc(
    out_path: path::PathBuf,
) -> Result<path::PathBuf, Box<dyn std::error::Error>> {
    // This is not yet supported on all platforms.
    // On Windows, we need Rust to link against libcmtd.
    let use_debug = {
        let var = env::var("LK_DEBUG_WEBRTC");
        var.is_ok() && var.unwrap() == "true"
    };

    let profile = if use_debug { "debug" } else { "release" };

    let file_name = format!("webrtc-{}-{}-{}.zip", &target_os(), &target_arch(), profile);
    let file_url = format!(
        "https://github.com/livekit/client-sdk-rust/releases/download/{}/{}",
        WEBRTC_TAG, file_name
    );
    let file_path = out_path.join(&file_name);

    if !out_path.exists() {
        fs::create_dir(&out_path)?;
    }

    // Download the release archive
    if !file_path.exists() {
        let file = fs::File::create(&file_path)?;
        {
            // Download WebRTC-SDK
            let res = reqwest::blocking::get(&file_url);
            if let Ok(mut res) = res {
                if res.status() != StatusCode::OK {
                    return Err(format!("Failed to download {}: {}", file_url, res.status()).into());
                }
                let mut writer = io::BufWriter::new(file);
                io::copy(&mut res, &mut writer)?;
            } else {
                fs::remove_file(&file_path)?;
                res?;
            }
        }

        // Extract the archive
        let file = fs::File::open(&file_path)?;
        let res = zip::ZipArchive::new(file);
        if res.is_err() {
            fs::remove_file(&file_path)?;
        }
        let mut archive = res?;
        for i in 0..archive.len() {
            let mut inner_file = archive.by_index(i)?;
            let relative_path = inner_file.mangled_name();
            if relative_path.to_string_lossy().is_empty() {
                continue; // Ignore root
            }

            let extracted_file = out_path.join(relative_path);
            if inner_file.name().ends_with('/') {
                // Directory
                fs::create_dir_all(&extracted_file)?;
            } else {
                // File
                if let Some(p) = extracted_file.parent() {
                    if !p.exists() {
                        fs::create_dir_all(&p)?;
                    }
                }
                let mut outfile = fs::File::create(&extracted_file)?;
                io::copy(&mut inner_file, &mut outfile)?;
            }
        }
    }

    Ok(out_path.join(file_name.replace(".zip", "")))
}

fn configure_darwin_sysroot(builder: &mut cc::Build) {
    let target_os = target_os();

    let sdk = match target_os.as_str() {
        "mac" => "macosx",
        "ios-device" => "iphoneos",
        "ios-simulator" => "iphonesimulator",
        _ => panic!("Unsupported target_os: {}", target_os),
    };

    let clang_rt = match target_os.as_str() {
        "mac" => "clang_rt.osx",
        "ios-device" => "clang_rt.ios",
        "ios-simulator" => "clang_rt.iossim",
        _ => panic!("Unsupported target_os: {}", target_os),
    };

    println!("cargo:rustc-link-lib={}", clang_rt);
    println!("cargo:rustc-link-arg=-ObjC");

    let sysroot = Command::new("xcrun")
        .args(["--sdk", sdk, "--show-sdk-path"])
        .output()
        .unwrap();

    let sysroot = String::from_utf8_lossy(&sysroot.stdout);
    let sysroot = sysroot.trim();

    let search_dirs = Command::new("clang")
        .arg("--print-search-dirs")
        .output()
        .unwrap();

    let search_dirs = String::from_utf8_lossy(&search_dirs.stdout);
    for line in search_dirs.lines() {
        if line.contains("libraries: =") {
            let path = line.split('=').nth(1).unwrap();
            let path = format!("{}/lib/darwin", path);
            println!("cargo:rustc-link-search={}", path);
        }
    }

    builder.flag(format!("-isysroot{}", sysroot).as_str());
}

fn main() {
    let use_custom_webrtc = {
        let var = env::var("LK_CUSTOM_WEBRTC");
        var.is_ok() && var.unwrap() == "true"
    };
    println!("cargo:rerun-if-env-changed=LK_CUSTOM_WEBRTC");

    let webrtc_dir = if use_custom_webrtc {
        path::PathBuf::from("./libwebrtc").canonicalize().unwrap()
    } else {
        let download_dir = env::var("OUT_DIR").unwrap() + "/webrtc-sdk";
        download_prebuilt_webrtc(path::PathBuf::from(download_dir))
            .unwrap()
            .canonicalize()
            .unwrap()
    };

    let (webrtc_dir, webrtc_include, webrtc_lib) = (
        webrtc_dir.clone(),
        webrtc_dir.join("include"),
        webrtc_dir.join("lib"),
    );

    let mut builder = cxx_build::bridges([
        "src/peer_connection.rs",
        "src/peer_connection_factory.rs",
        "src/media_stream.rs",
        "src/media_stream_track.rs",
        "src/audio_track.rs",
        "src/video_track.rs",
        "src/data_channel.rs",
        "src/jsep.rs",
        "src/candidate.rs",
        "src/rtp_parameters.rs",
        "src/rtp_sender.rs",
        "src/rtp_receiver.rs",
        "src/rtp_transceiver.rs",
        "src/rtc_error.rs",
        "src/webrtc.rs",
        "src/video_frame.rs",
        "src/video_frame_buffer.rs",
        "src/yuv_helper.rs",
        "src/helper.rs",
        "src/audio_resampler.rs",
        "src/android.rs",
    ]);

    builder.includes(&[
        path::PathBuf::from("./include"),
        webrtc_include.clone(),
        webrtc_include.join("third_party/abseil-cpp/"),
        webrtc_include.join("third_party/libyuv/include/"),
        webrtc_include.join("third_party/libc++/"),
        // For mac & ios
        webrtc_include.join("sdk/objc"),
        webrtc_include.join("sdk/objc/base"),
    ]);

    builder.files(&[
        "src/peer_connection.cpp",
        "src/peer_connection_factory.cpp",
        "src/media_stream.cpp",
        "src/media_stream_track.cpp",
        "src/audio_track.cpp",
        "src/video_track.cpp",
        "src/data_channel.cpp",
        "src/jsep.cpp",
        "src/candidate.cpp",
        "src/rtp_receiver.cpp",
        "src/rtp_sender.cpp",
        "src/rtp_transceiver.cpp",
        "src/rtp_parameters.cpp",
        "src/rtc_error.cpp",
        "src/webrtc.cpp",
        "src/video_frame.cpp",
        "src/video_frame_buffer.cpp",
        "src/video_encoder_factory.cpp",
        "src/video_decoder_factory.cpp",
        "src/audio_device.cpp",
        "src/audio_resampler.cpp",
        "src/android.cpp",
    ]);

    println!(
        "cargo:rustc-link-search=native={}",
        webrtc_lib.to_str().unwrap()
    );

    // Read preprocessor definitions from webrtc.ninja
    let webrtc_gni = fs::File::open(webrtc_dir.join("webrtc.ninja")).unwrap();
    let mut reader = io::BufReader::new(webrtc_gni).lines();
    let defines_line = reader.next().unwrap().unwrap(); // The first line contains the defines
    let defines_re = Regex::new(r"-D(\w+)(?:=([^\s]+))?").unwrap();
    for cap in defines_re.captures_iter(&defines_line) {
        let define_name = &cap[1];
        let define_value = cap.get(2).map(|m| m.as_str());
        if IGNORE_DEFINES.contains(&define_name) {
            continue;
        }
        builder.define(define_name, define_value);
    }

    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap();
    match target_os.as_str() {
        "windows" => {
            println!("cargo:rustc-link-lib=dylib=msdmo");
            println!("cargo:rustc-link-lib=dylib=wmcodecdspuuid");
            println!("cargo:rustc-link-lib=dylib=dmoguids");
            println!("cargo:rustc-link-lib=dylib=crypt32");
            println!("cargo:rustc-link-lib=dylib=iphlpapi");
            println!("cargo:rustc-link-lib=dylib=ole32");
            println!("cargo:rustc-link-lib=dylib=secur32");
            println!("cargo:rustc-link-lib=dylib=winmm");
            println!("cargo:rustc-link-lib=dylib=ws2_32");
            println!("cargo:rustc-link-lib=dylib=strmiids");
            println!("cargo:rustc-link-lib=dylib=d3d11");
            println!("cargo:rustc-link-lib=dylib=gdi32");
            println!("cargo:rustc-link-lib=dylib=dxgi");
            println!("cargo:rustc-link-lib=dylib=dwmapi");
            println!("cargo:rustc-link-lib=static=webrtc");

            builder.flag("/std:c++17").flag("/EHsc");
        }
        "linux" => {
            println!("cargo:rustc-link-lib=dylib=Xext");
            println!("cargo:rustc-link-lib=dylib=X11");
            println!("cargo:rustc-link-lib=dylib=GL");
            println!("cargo:rustc-link-lib=dylib=rt");
            println!("cargo:rustc-link-lib=dylib=dl");
            println!("cargo:rustc-link-lib=dylib=pthread");
            println!("cargo:rustc-link-lib=dylib=m");
            println!("cargo:rustc-link-lib=static=webrtc");

            builder.flag("-std=c++17");
        }
        "macos" => {
            println!("cargo:rustc-link-lib=framework=Foundation");
            println!("cargo:rustc-link-lib=framework=AVFoundation");
            println!("cargo:rustc-link-lib=framework=CoreAudio");
            println!("cargo:rustc-link-lib=framework=AudioToolbox");
            println!("cargo:rustc-link-lib=framework=Appkit");
            println!("cargo:rustc-link-lib=framework=CoreMedia");
            println!("cargo:rustc-link-lib=framework=CoreGraphics");
            println!("cargo:rustc-link-lib=framework=VideoToolbox");
            println!("cargo:rustc-link-lib=framework=CoreVideo");
            println!("cargo:rustc-link-lib=framework=OpenGL");
            println!("cargo:rustc-link-lib=framework=Metal");
            println!("cargo:rustc-link-lib=framework=QuartzCore");
            println!("cargo:rustc-link-lib=framework=IOKit");
            println!("cargo:rustc-link-lib=framework=IOSurface");
            println!("cargo:rustc-link-lib=static=webrtc");

            configure_darwin_sysroot(&mut builder);

            builder
                .file("src/objc_video_factory.mm")
                .flag("-stdlib=libc++")
                .flag("-std=c++17");
        }
        "ios" => {
            println!("cargo:rustc-link-lib=static=webrtc");
            println!("cargo:rustc-link-lib=framework=CoreFoundation");
            println!("cargo:rustc-link-lib=framework=AVFoundation");
            println!("cargo:rustc-link-lib=framework=CoreAudio");
            println!("cargo:rustc-link-lib=framework=UIKit");
            println!("cargo:rustc-link-lib=framework=CoreVideo");
            println!("cargo:rustc-link-lib=framework=CoreGraphics");
            println!("cargo:rustc-link-lib=framework=CoreMedia");
            println!("cargo:rustc-link-lib=framework=VideoToolbox");
            println!("cargo:rustc-link-lib=framework=AudioToolbox");
            println!("cargo:rustc-link-lib=framework=OpenGLES");
            println!("cargo:rustc-link-lib=framework=GLKit");
            println!("cargo:rustc-link-lib=framework=Metal");
            println!("cargo:rustc-link-lib=framework=Network");
            println!("cargo:rustc-link-lib=framework=QuartzCore");
            println!("cargo:rustc-link-lib=objc");

            configure_darwin_sysroot(&mut builder);

            builder.file("src/objc_video_factory.mm").flag("-std=c++17");
        }
        "android" => {
            let ndk_env = env::var("ANDROID_NDK_HOME").expect(
                "ANDROID_NDK_HOME is not set, please set it to the path of your Android NDK",
            );
            let android_ndk = path::PathBuf::from(ndk_env);

            let host_os = if cfg!(linux) {
                "linux-x86_64"
            } else if cfg!(target_os = "macos") {
                "darwin-x86_64"
            } else if cfg!(windows) {
                "windows-x86_64"
            } else {
                panic!("Unsupported host OS");
            };

            let toolchain = android_ndk.join(std::format!("toolchains/llvm/prebuilt/{}", host_os));

            // libgcc (redirects to libunwind)
            println!(
                "cargo:rustc-link-search={}",
                toolchain.join("lib").to_str().unwrap()
            );

            // Needed when loading the library in the JVM (System.loadLibrary)
            println!("cargo:rustc-link-lib=egl");
            println!("cargo:rustc-link-lib=OpenSLES");

            // Find JNI symbols
            let readelf_output = Command::new(toolchain.join("bin/llvm-readelf"))
                .arg("-Ws")
                .arg(webrtc_lib.join("/libwebrtc.a"))
                .output()
                .expect("failed to run llvm-readelf");

            // Get all JNI symbols
            let jni_regex = Regex::new(r"(Java_org_webrtc.*)").unwrap();
            let content = &String::from_utf8_lossy(&readelf_output.stdout);
            let mut jni_symbols = Vec::new();
            jni_regex.captures_iter(content).for_each(|cap| {
                jni_symbols.push(cap.get(1).unwrap().as_str());
            });

            // JNI Version Script & Keep JNI symbols
            let vs_path = path::PathBuf::from(env::var("OUT_DIR").unwrap()).join("webrtc_jni.map");
            let mut vs_file = fs::File::create(&vs_path).unwrap();

            write!(vs_file, "JNI_WEBRTC {{\n\tglobal: ").unwrap();
            for x in jni_symbols {
                println!("cargo:rustc-link-arg=-Wl,--undefined={}", x);
                write!(vs_file, "{}; ", x).unwrap();
            }
            write!(vs_file, "\n}};").unwrap();

            println!(
                "cargo:rustc-link-arg=-Wl,--version-script={}",
                vs_path.to_str().unwrap()
            );

            println!("cargo:rustc-link-lib=static=webrtc");

            // Set sysroot
            let sysroot = toolchain.join("sysroot").canonicalize().unwrap();
            let sysroot_include = sysroot.join("usr/include");

            println!(
                "cargo:rustc-link-search={}",
                sysroot_include.to_str().unwrap()
            );

            builder.flag(format!("-isysroot{}", &sysroot.to_string_lossy().to_string()).as_str());
            builder.flag("-std=c++17");
        }
        _ => {
            panic!("Unsupported target, {}", target_os);
        }
    }

    // TODO(theomonnom) Only add this define when building tests
    builder.define("LIVEKIT_TEST", None);

    builder.warnings(false).compile("webrtcsys-cxx");

    for entry in glob::glob("./src/**/*.cpp").unwrap() {
        println!("cargo:rerun-if-changed={}", entry.unwrap().display());
    }

    for entry in glob::glob("./src/**/*.mm").unwrap() {
        println!("cargo:rerun-if-changed={}", entry.unwrap().display());
    }

    for entry in glob::glob("./include/**/*.h").unwrap() {
        println!("cargo:rerun-if-changed={}", entry.unwrap().display());
    }
}
