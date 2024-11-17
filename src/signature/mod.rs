use base64::engine::general_purpose::STANDARD;
use base64::Engine;
use hmac::{Hmac, Mac};
use http::header::{CONTENT_TYPE, DATE};
use http::HeaderValue;
use log::debug;
use sha1::Sha1;
use std::collections::BTreeMap;
use std::sync::Arc;
use tower_http::set_header::MakeHeaderValue;

#[derive(Clone)]
pub struct Signer {
    access_id: Arc<String>,
    access_key: Arc<String>,
}

impl Signer {
    pub fn new(access_id: &str, access_key: &str) -> Self {
        Self {
            access_id: Arc::new(access_id.to_string()),
            access_key: Arc::new(access_key.to_string()),
        }
    }

    fn calc_signature(access_key: &str, auth_input: &str) -> anyhow::Result<String> {
        let mut mac = Hmac::<Sha1>::new_from_slice(access_key.as_bytes())?;
        mac.update(auth_input.as_ref());
        let result = mac.finalize();
        let code_bytes = result.into_bytes();
        let sig = STANDARD.encode(code_bytes);
        Ok(sig)
    }
}

impl MakeHeaderValue<reqwest::Request> for Signer {
    fn make_header_value(&mut self, request: &reqwest::Request) -> Option<HeaderValue> {
        let http_method = request.method().as_str().to_uppercase();
        let header_map = request.headers();
        let content_type = match header_map.get(CONTENT_TYPE) {
            None => "",
            Some(v) => v.to_str().unwrap(),
        };
        let date = header_map.get(DATE).unwrap().to_str().unwrap();

        let canonicalized_datahub_headers: BTreeMap<&str, &HeaderValue> = header_map
            .iter()
            .filter(|(k, _)| {
                debug!("found header:{}", k);
                k.as_str().starts_with("x-datahub-")
            })
            .map(|(k, v)| (k.as_str(), v))
            .collect();

        let canonicalized_datahub_headers_str = canonicalized_datahub_headers
            .iter()
            .map(|(k, v)| format!("{}:{}", k, v.to_str().unwrap()))
            .reduce(|acc, x| format!("{acc}\n{x}"))
            .unwrap_or(String::new());

        let path = request.url().path();

        debug!("request path:{path}");
        let query = request.url().query();

        let query = match query {
            Some(query) => {
                let mut query_pair: Vec<&str> = query.split("&").collect();
                query_pair.sort_by(|a, b| {
                    a.split("=")
                        .next()
                        .unwrap()
                        .cmp(b.split("=").next().unwrap())
                });

                query_pair
                    .iter_mut()
                    .map(|kv| {
                        if !kv.contains("=") {
                            format!("{kv}=")
                        } else {
                            kv.to_string()
                        }
                    })
                    .fold(String::new(), |acc, x| format!("{acc}&{x}"))
            }
            None => String::new(),
        };

        let canonicalized_resource_str = if query.is_empty() {
            path.to_string()
        } else {
            format!("{path}?{query}")
        };

        let str_to_sig = &[
            &http_method,
            content_type,
            date,
            &canonicalized_datahub_headers_str,
            &canonicalized_resource_str,
        ]
        .join("\n");

        let signature = Self::calc_signature(&self.access_key, str_to_sig).unwrap();

        let authorization = format!("DATAHUB {}:{signature}", &self.access_id);

        Some(HeaderValue::from_str(&authorization).unwrap())
    }
}
