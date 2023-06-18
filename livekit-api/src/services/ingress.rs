use super::{ServiceBase, ServiceResult, LIVEKIT_PACKAGE};
use crate::services::twirp_client::TwirpClient;
use crate::{access_token::VideoGrants, get_env_keys};
use livekit_protocol as proto;

#[derive(Default, Clone, Debug)]
pub struct IngressOptions {
    pub name: String,
    pub room_name: String,
    pub participant_identity: String,
    pub participant_name: String,
    pub audio: proto::IngressAudioOptions,
    pub video: proto::IngressVideoOptions,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IngressListFilter {
    All,
    Room(String),
}

const SVC: &'static str = "Ingress";

#[derive(Debug)]
pub struct IngressClient {
    base: ServiceBase,
    client: TwirpClient,
}

impl IngressClient {
    pub fn with_api_key(host: &str, api_key: &str, api_secret: &str) -> Self {
        Self {
            base: ServiceBase::with_api_key(api_key, api_secret),
            client: TwirpClient::new(host, LIVEKIT_PACKAGE, None),
        }
    }

    pub fn new(host: &str) -> ServiceResult<Self> {
        let (api_key, api_secret) = get_env_keys()?;
        Ok(Self::with_api_key(host, &api_key, &api_secret))
    }

    pub async fn create_ingress(
        &self,
        input_type: proto::IngressInput,
        options: IngressOptions,
    ) -> ServiceResult<proto::IngressInfo> {
        self.client
            .request(
                SVC,
                "CreateIngress",
                proto::CreateIngressRequest {
                    input_type: input_type as i32,
                    name: options.name,
                    room_name: options.room_name,
                    participant_identity: options.participant_identity,
                    participant_name: options.participant_name,
                    audio: Some(options.audio),
                    video: Some(options.video),
                    bypass_transcoding: false, // TODO Expose
                },
                self.base.auth_header(VideoGrants {
                    ingress_admin: true,
                    ..Default::default()
                })?,
            )
            .await
            .map_err(Into::into)
    }

    pub async fn update_ingress(
        &self,
        ingress_id: &str,
        options: IngressOptions,
    ) -> ServiceResult<proto::IngressInfo> {
        self.client
            .request(
                SVC,
                "UpdateIngress",
                proto::UpdateIngressRequest {
                    ingress_id: ingress_id.to_owned(),
                    name: options.name,
                    room_name: options.room_name,
                    participant_identity: options.participant_identity,
                    participant_name: options.participant_name,
                    audio: Some(options.audio),
                    video: Some(options.video),
                    bypass_transcoding: None, // TODO Expose
                },
                self.base.auth_header(VideoGrants {
                    ingress_admin: true,
                    ..Default::default()
                })?,
            )
            .await
            .map_err(Into::into)
    }

    pub async fn list_ingress(
        &self,
        filter: IngressListFilter,
    ) -> ServiceResult<Vec<proto::IngressInfo>> {
        let resp: proto::ListIngressResponse = self
            .client
            .request(
                SVC,
                "ListIngress",
                proto::ListIngressRequest {
                    room_name: match filter {
                        IngressListFilter::All => Default::default(),
                        IngressListFilter::Room(room) => room,
                    },
                },
                self.base.auth_header(VideoGrants {
                    ingress_admin: true,
                    ..Default::default()
                })?,
            )
            .await?;

        Ok(resp.items)
    }

    pub async fn delete_ingress(&self, ingress_id: &str) -> ServiceResult<proto::IngressInfo> {
        self.client
            .request(
                SVC,
                "DeleteIngress",
                proto::DeleteIngressRequest {
                    ingress_id: ingress_id.to_owned(),
                },
                self.base.auth_header(VideoGrants {
                    ingress_admin: true,
                    ..Default::default()
                })?,
            )
            .await
            .map_err(Into::into)
    }
}
