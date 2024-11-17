#[macro_export]
macro_rules! parse_json_response {
    ($res:expr,$ty:ty) => {
        if $res.status().is_success() {
            let request_id_header = $res.headers().get("x-datahub-request-id");
            let request_id = request_id_header
                .as_ref()
                .map_or("", |h| h.to_str().unwrap_or(""))
                .to_string();
            let mut list_project_response: $ty = $res.json().await?;
            list_project_response.set_request_id(Some(request_id));
            Ok(list_project_response)
        } else {
            let err_info: $crate::models::err_msg::ErrorInfo = $res.json().await?;
            Err(err_info.into())
        }
    };
}

#[macro_export]
macro_rules! parse_empty_response {
    ($res:expr) => {
        if $res.status().is_success() {
            let request_id_header = $res.headers().get("x-datahub-request-id");
            let request_id = request_id_header
                .as_ref()
                .map_or("", |h| h.to_str().unwrap_or(""))
                .to_string();
            let mut list_project_response: $crate::models::EmptyResponse = Default::default();
            list_project_response.set_request_id(Some(request_id));
            Ok(list_project_response)
        } else {
            let err_info: $crate::models::err_msg::ErrorInfo = $res.json().await?;
            Err(err_info.into())
        }
    };
}
