extern crate core;

mod c;
mod error;
mod json;
mod status;
mod utils;

pub use error::Error;
pub use etsi014_client::ETSI014Client;
pub use secrets::SecretVec;
pub use status::Status;

pub mod etsi014_client {
    use crate::error::ErrorType::{
        ConnectionError, InvalidArgument, InvalidHost, InvalidResponse,
    };
    use crate::json::key_container::KeyContainer;
    use crate::json::key_id::KeyId;
    use crate::json::key_request::KeyRequest;
    use crate::json::keys_by_ids_request::KeysByIdsRequest;
    use crate::json::status_response::StatusResponse;
    use crate::status::Status;
    use crate::utils::read_file;
    use crate::Error;
    use base64ct::{Base64, Encoding};
    use reqwest::header::CONTENT_TYPE;
    use reqwest::{Client, Identity, Url};
    pub use secrets::Secret;
    pub use secrets::SecretBox;
    pub use secrets::SecretVec;
    use serde::de;
    use std::path::PathBuf;

    #[derive(Debug)]
    pub struct ETSI014Client {
        http_client: Client,
        base_url: Url,
    }

    impl ETSI014Client {
        const PATH_PREFIX: &'static str = "api/v1/keys";

        pub fn new(
            host: &str,
            port: u16,
            cert_path: &PathBuf,
            key_path: &PathBuf,
            server_ca_path: &PathBuf,
        ) -> Result<Self, Error> {
            // Can not set host and port without parsing something first
            let mut base_url =
                Url::parse("https://localhost").expect("Error parsing hardcoded URL");
            base_url
                .set_scheme("https")
                .expect("Error setting https as scheme");
            base_url.set_host(Some(host)).map_err(|e| {
                Error::new(
                    format!("Invalid host: {host}"),
                    InvalidHost,
                    Some(Box::new(e)),
                )
            })?;
            base_url
                .set_port(Some(port))
                // Might fail if host invalid
                .map_err(|_| {
                    Error::new(
                        format!("Error setting port for host: '{host}"),
                        InvalidHost,
                        None,
                    )
                })?;
            let server_ca = reqwest::Certificate::from_pem(&read_file(server_ca_path)?)
                .map_err(|e| {
                Error::new(
                    format!("Error parsing {server_ca_path:?}"),
                    InvalidArgument,
                    Some(Box::new(e)),
                )
            })?;
            let identity =
                Identity::from_pkcs8_pem(&read_file(cert_path)?, &read_file(key_path)?)
                    .map_err(|e| {
                    Error::new(
                        format!("Error parsing {cert_path:?} or {key_path:?}"),
                        InvalidArgument,
                        Some(Box::new(e)),
                    )
                })?;
            let http_client = Client::builder()
                .use_native_tls()
                .tls_built_in_root_certs(false)
                .add_root_certificate(server_ca)
                .identity(identity)
                .build()
                .map_err(|e| {
                    Error::new(
                        "Error creating http client".to_string(),
                        InvalidArgument,
                        Some(Box::new(e)),
                    )
                })?;
            Ok(ETSI014Client {
                http_client,
                base_url,
            })
        }

        async fn send_request<T>(
            &self,
            target_sae_id: &str,
            endpoint: &str,
            body: Option<&str>,
        ) -> Result<T, Error>
        where
            T: de::DeserializeOwned,
        {
            let mut url = self.base_url.clone();
            let path_prefix = Self::PATH_PREFIX;
            url.set_path(&format!("{path_prefix}/{target_sae_id}/{endpoint}"));
            let request = match body {
                None => self
                    .http_client
                    .get(url.clone())
                    .build()
                    .map_err(|e| Error::new(
                        format!("Error building request for url: {url}"),
                        InvalidArgument,
                        Some(Box::new(e)),
                    )),
                Some(body) =>
                    self.http_client
                        .post(url.clone())
                        .header(CONTENT_TYPE, "application/json")
                        .body(body.to_owned())
                        .build()
                        .map_err(|e| Error::new(
                            format!("Error building request for url: {url}\n\nRequest body: {body}"),
                            InvalidArgument,
                            Some(Box::new(e))
                        )),
            }?;

            let response = self
                .http_client
                .execute(request.try_clone().unwrap())
                .await
                .map_err(|e| {
                    Error::new(
                        format!("Error sending request: {request:#?}"),
                        ConnectionError,
                        Some(Box::new(e)),
                    )
                })?;
            let http_code = response.status();
            let response_string = response.text().await.map_err(|e| {
                Error::new(
                    "Response not UTF-8".to_string(),
                    InvalidResponse,
                    Some(Box::new(e)),
                )
            })?;
            let error_info = |s: String| {
                let body_info = match body {
                    None => "".to_owned(),
                    Some(body) => format!("\nUsing POST body: {body}"),
                };
                format!(
                    "{s}\n\n\
                         HTTP Code: {http_code}\n\
                         Response:\n{response_string}\n\
                         Using request: {request:#?}{body_info}"
                )
            };
            if !http_code.is_success() {
                return Err(Error::new(
                    error_info("Unsuccessful HTTP code".to_string()),
                    InvalidResponse,
                    None,
                ));
            }
            serde_json::from_str::<T>(&response_string).map_err(|e| {
                Error::new(
                    error_info("Unable to deserialize JSON from response.".to_string()),
                    InvalidResponse,
                    Some(Box::new(e)),
                )
            })
        }

        pub async fn get_status(&self, target_sae_id: &str) -> Result<Status, Error> {
            let sr: StatusResponse =
                self.send_request(target_sae_id, "status", None).await?;
            Ok(Status {
                source_kme_id: sr.source_kme_id,
                target_kme_id: sr.target_kme_id,
                source_sae_id: sr.source_sae_id,
                target_sae_id: sr.target_sae_id,
                key_size: sr.key_size,
                stored_key_count: sr.stored_key_count,
                max_key_count: sr.max_key_count,
                max_key_per_request: sr.max_key_per_request,
                max_key_size: sr.max_key_size,
                min_key_size: sr.min_key_size,
                max_sae_id_count: sr.max_sae_id_count,
            })
        }

        fn key_container_to_vector(
            kc: KeyContainer,
        ) -> Result<Vec<(String, SecretVec<u8>)>, Error> {
            let amount_of_keys = kc.keys.len();
            kc.keys.into_iter().try_fold(
                Vec::with_capacity(amount_of_keys),
                |mut l, key_and_id| {
                    let uuid = &key_and_id.key_id;
                    let base64_string = key_and_id.key;
                    let mut base64_vec = base64_string.into_bytes();
                    let base64_slice = base64_vec.as_mut();
                    let mut secret_base64 = SecretVec::from(base64_slice);
                    let mut secret_base64_ref_mut = secret_base64.borrow_mut();
                    let secret_slice = Base64::decode_in_place(
                        secret_base64_ref_mut.as_mut(),
                    )
                    .map_err(|_| {
                        Error::new(
                            format!("Error decoding base64 for uuid {uuid}"),
                            InvalidResponse,
                            None,
                        )
                    })?;
                    // Cannot resize SecretVec, so create a new shorter one.
                    let secret = SecretVec::new(secret_slice.len(), |sv| {
                        sv.copy_from_slice(secret_slice);
                    });
                    l.push((key_and_id.key_id, secret));
                    Ok(l)
                },
            )
        }

        pub async fn get_keys(
            &self,
            key_size_bits: u32,
            target_sae_id: &str,
            additional_target_sae_ids: &[&str],
            amount_of_keys: u32,
        ) -> Result<Vec<(String, SecretVec<u8>)>, Error> {
            let post_body = serde_json::to_string(&KeyRequest {
                number: amount_of_keys,
                size: Some(key_size_bits),
                additional_target_sae_ids,
                extension_mandatory: None,
            })
            .expect("Error serializing key request.");
            let key_container = self
                .send_request::<KeyContainer>(target_sae_id, "enc_keys", Some(&post_body))
                .await?;
            Self::key_container_to_vector(key_container)
        }

        pub async fn get_keys_by_ids(
            &self,
            target_sae_id: &str,
            key_ids: &[&str],
        ) -> Result<Vec<(String, SecretVec<u8>)>, Error> {
            let post_body = serde_json::to_string(&KeysByIdsRequest {
                key_ids: key_ids.iter().map(|key_id| KeyId { key_id }).collect(),
            })
            .expect("Error serializing keys by ids reqeust");
            let key_container = self
                .send_request::<KeyContainer>(target_sae_id, "dec_keys", Some(&post_body))
                .await?;
            Self::key_container_to_vector(key_container)
        }
    }
}
