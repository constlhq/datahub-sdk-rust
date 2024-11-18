use crate::client::datahub_client::DatahubClientTrait;
use crate::compressor::CompressMethod;
use crate::config::DatahubConfig;
use crate::errors::DHResult;
use crate::middleware::set_header::{SetRequestHeader, SetRequestHeaderLayer};
use crate::models::cursor::QueryCursorResponse;
use crate::models::project::{GetProjectResponse, ListProjectResponse};
use crate::models::record::{FieldType, ReadDataResponse, WriteDataResponse};
use crate::models::shard::{ListShardResponse, MergeShardResponse, SplitShardResponse};
use crate::models::subscription::{
    CreateSubscriptionRes, GetSubscriptionRes, ListSubscriptionRes, SubscriptionOffset,
    SubscriptionSessionOptRes,
};
use crate::models::topic::{GetTopicResponse, ListTopicResponse};
use crate::models::EmptyResponse;
use crate::payload::comment::UpdateCommentPayload;
use crate::payload::cursor::{CursorType, QueryCursorPayload};
use crate::payload::data::{ReadDataPayload, WriteDataPayload};
use crate::payload::projects::CreateProjectPayload;
use crate::payload::shards::{MergeShardPayload, SplitShardPayload};
use crate::payload::subscriptions::{
    CreateSubscriptionPayload, ListSubscriptionsPayload, SetSubscriptionStatePayload,
    SubscriptionSessionOptPayload,
};
use crate::payload::topics::{AppendFieldPayload, CreateTopicPayload};
use crate::signature::Signer;
use crate::version::API_VERSION;
use crate::{parse_empty_response, parse_json_response};
use async_trait::async_trait;
use chrono::Utc;
use http::header::{AUTHORIZATION, DATE, USER_AGENT};
use http::{HeaderMap, HeaderName, HeaderValue, Method};
use reqwest::{Client, Request, Url};
use std::collections::HashMap;
use tower::{Service, ServiceBuilder};

type SetRequestHeaderTowerService = SetRequestHeader<
    SetRequestHeader<
        SetRequestHeader<SetRequestHeader<Client, Signer>, Option<HeaderValue>>,
        HeaderValue,
    >,
    HeaderValue,
>;
#[derive(Clone)]
pub struct DatahubJsonClient {
    client: Client,
    service: SetRequestHeaderTowerService,
    endpoint: Url,
}
impl DatahubJsonClient {
    pub(crate) fn new(conf: &DatahubConfig) -> Self {
        let endpoint = conf.endpoint();
        let account = conf.account();
        let endpoint = Url::parse(endpoint).expect("invalid endpoint");
        let http_config = conf.http_config();

        let mut default_headers = HeaderMap::new();
        default_headers.insert(USER_AGENT, HeaderValue::from_static("datahub-sdk-rust"));

        let client_builder = Client::builder()
            .default_headers(default_headers)
            .connect_timeout(http_config.conn_timeout())
            .read_timeout(http_config.read_timeout());

        let client_builder = match http_config.compress_type() {
            CompressMethod::ZLIB => client_builder.zstd(true),
            CompressMethod::DEFLATE => client_builder.deflate(true),
        };

        let client = client_builder.build().unwrap();

        let signer = Signer::new(account.id(), account.key());

        let service = ServiceBuilder::new()
            .layer(SetRequestHeaderLayer::new(
                DATE,
                HeaderValue::from_str(&Utc::now().format("%a, %d %b %Y %H:%M:%S GMT").to_string())
                    .unwrap(),
            ))
            .layer(SetRequestHeaderLayer::new(
                HeaderName::from_static("x-datahub-client-version"),
                HeaderValue::from_static(API_VERSION),
            ))
            .layer(SetRequestHeaderLayer::new(
                HeaderName::from_static("x-datahub-security-token"),
                match account.token() {
                    None => None,
                    Some(token) => HeaderValue::from_str(token).ok(),
                },
            ))
            .layer(SetRequestHeaderLayer::new(AUTHORIZATION, signer))
            .service(client.clone());
        Self {
            service,
            endpoint,
            client,
        }
    }
}
#[async_trait]
impl DatahubClientTrait for DatahubJsonClient {
    /// 查询Project列表
    async fn list_project(&mut self) -> DHResult<ListProjectResponse> {
        let url = self.endpoint.join("/projects")?;
        let request = Request::new(Method::GET, url);
        let res = self.service.call(request).await?;
        parse_json_response!(res, ListProjectResponse)
    }

    /// 创建Project
    async fn create_project(
        &mut self,
        project_name: &str,
        comment: &str,
    ) -> DHResult<EmptyResponse> {
        let path = format!("/projects/{project_name}");
        let url = self.endpoint.join(&path)?;
        let mut request_builder = self.client.post(url);
        let create_project_payload = CreateProjectPayload::new(comment);
        request_builder = request_builder.json(&create_project_payload);
        let request = request_builder.build()?;
        let res = self.service.call(request).await?;
        parse_empty_response!(res)
    }
    /// 查询Project
    async fn get_project(&mut self, project_name: &str) -> DHResult<GetProjectResponse> {
        let path = format!("/projects/{project_name}");
        let url = self.endpoint.join(&path)?;
        let request = Request::new(Method::GET, url);
        let res = self.service.call(request).await?;
        parse_json_response!(res, GetProjectResponse)
    }

    /// 更新Project
    async fn update_project(
        &mut self,
        project_name: &str,
        comment: &str,
    ) -> DHResult<EmptyResponse> {
        let path = format!("/projects/{project_name}");
        let url = self.endpoint.join(&path)?;

        let mut request_builder = self.client.put(url);
        let create_project_payload: CreateProjectPayload = CreateProjectPayload::new(comment);
        request_builder = request_builder.json(&create_project_payload);
        let request = request_builder.build()?;
        let res = self.service.call(request).await?;
        parse_empty_response!(res)
    }
    /// 删除Project
    async fn delete_project(&mut self, project_name: &str) -> DHResult<EmptyResponse> {
        let path = format!("/projects/{project_name}");
        let url = self.endpoint.join(&path)?;
        let request = Request::new(Method::DELETE, url);
        let res = self.service.call(request).await?;
        parse_empty_response!(res)
    }

    /// 创建Topic
    async fn create_topic(
        &mut self,
        project_name: &str,
        topic_name: &str,
        create_topic_payload: &CreateTopicPayload,
    ) -> DHResult<EmptyResponse> {
        let path = format!("/projects/{project_name}/topics/{topic_name}");
        let url = self.endpoint.join(&path)?;
        let mut request_builder = self.client.post(url);
        request_builder = request_builder.json(&create_topic_payload);
        let request = request_builder.build()?;
        let res = self.service.call(request).await?;
        parse_empty_response!(res)
    }

    /// 查询Topic
    async fn get_topic(
        &mut self,
        project_name: &str,
        topic_name: &str,
    ) -> DHResult<GetTopicResponse> {
        let path = format!("/projects/{project_name}/topics/{topic_name}");
        let url = self.endpoint.join(&path)?;
        let request = Request::new(Method::GET, url);
        let res = self.service.call(request).await?;
        parse_json_response!(res, GetTopicResponse)
    }

    /// 查询Topic列表
    async fn list_topic(&mut self, project_name: &str) -> DHResult<ListTopicResponse> {
        let path = format!("/projects/{project_name}/topics");
        let url = self.endpoint.join(&path)?;
        let request = Request::new(Method::GET, url);
        let res = self.service.call(request).await?;
        parse_json_response!(res, ListTopicResponse)
    }

    ///更新Topic
    async fn update_topic(
        &mut self,
        project_name: &str,
        topic_name: &str,
        comment: &str,
    ) -> DHResult<EmptyResponse> {
        let path = format!("/projects/{project_name}/topics/{topic_name}");
        let url = self.endpoint.join(&path)?;
        let mut request_builder = self.client.put(url);
        let update_comment_payload = UpdateCommentPayload { comment };
        request_builder = request_builder.json(&update_comment_payload);
        let request = request_builder.build()?;
        let res = self.service.call(request).await?;
        parse_empty_response!(res)
    }

    /// 删除Topic
    async fn delete_topic(
        &mut self,
        project_name: &str,
        topic_name: &str,
    ) -> DHResult<EmptyResponse> {
        let path = format!("/projects/{project_name}/topics/{topic_name}");
        let url = self.endpoint.join(&path)?;
        let request = Request::new(Method::GET, url);
        let res = self.service.call(request).await?;
        parse_empty_response!(res)
    }

    ///新增Field
    async fn append_filed(
        &mut self,
        project_name: &str,
        topic_name: &str,
        field_name: &str,
        field_type: FieldType,
    ) -> DHResult<EmptyResponse> {
        let path = format!("/projects/{project_name}/topics/{topic_name}");
        let url = self.endpoint.join(&path)?;
        let mut request_builder = self.client.post(url);
        let append_field_payload: AppendFieldPayload =
            AppendFieldPayload::new(field_name, field_type);
        request_builder = request_builder.json(&append_field_payload);
        let request = request_builder.build()?;
        let res = self.service.call(request).await?;
        parse_empty_response!(res)
    }

    ///获取Shard列表
    async fn list_shard(
        &mut self,
        project_name: &str,
        topic_name: &str,
    ) -> DHResult<ListShardResponse> {
        let path = format!("/projects/{project_name}/topics/{topic_name}/shards");
        let url = self.endpoint.join(&path)?;
        let request = Request::new(Method::GET, url);
        let res = self.service.call(request).await?;
        parse_json_response!(res, ListShardResponse)
    }

    /// 分裂Shard
    async fn split_shard(
        &mut self,
        project_name: &str,
        topic_name: &str,
        shard_id: &str,
        split_key: &str,
    ) -> DHResult<SplitShardResponse> {
        let path = format!("/projects/{project_name}/topics/{topic_name}/shards");
        let url = self.endpoint.join(&path)?;
        let mut request_builder = self.client.put(url);
        let split_shard_payload = SplitShardPayload::new(shard_id, split_key);
        request_builder = request_builder.json(&split_shard_payload);
        let request = request_builder.build()?;
        let res = self.service.call(request).await?;
        parse_json_response!(res, SplitShardResponse)
    }

    ///合并Shard
    async fn merge_shard(
        &mut self,
        project_name: &str,
        topic_name: &str,
        shard_id: &str,
        adjacent_shard_id: &str,
    ) -> DHResult<MergeShardResponse> {
        let path = format!("/projects/{project_name}/topics/{topic_name}/shards");
        let url = self.endpoint.join(&path)?;
        let mut request_builder = self.client.put(url);
        let merge_shard_payload = MergeShardPayload::new(shard_id, adjacent_shard_id);
        request_builder = request_builder.json(&merge_shard_payload);
        let request = request_builder.build()?;
        let res = self.service.call(request).await?;
        parse_json_response!(res, MergeShardResponse)
    }

    /// 查询数据Cursor
    async fn get_cursor(
        &mut self,
        project_name: &str,
        topic_name: &str,
        shard_id: &str,
        cursor_type: CursorType,
        parameter: i64,
    ) -> DHResult<QueryCursorResponse> {
        let path = format!("/projects/{project_name}/topics/{topic_name}/shards/{shard_id}");
        let url = self.endpoint.join(&path)?;
        let mut request_builder = self.client.post(url);
        let query_cursor_payload = QueryCursorPayload::new(cursor_type, parameter);
        request_builder = request_builder.json(&query_cursor_payload);
        let request = request_builder.build()?;
        let res = self.service.call(request).await?;
        parse_json_response!(res, QueryCursorResponse)
    }

    /// 写入数据
    async fn write_data(
        &mut self,
        project_name: &str,
        topic_name: &str,
        write_data_payload: &WriteDataPayload,
    ) -> DHResult<WriteDataResponse> {
        let path = format!("/projects/{project_name}/topics/{topic_name}/shards");
        let url = self.endpoint.join(&path)?;
        let mut request_builder = self.client.post(url);
        request_builder = request_builder.json(write_data_payload);
        let request = request_builder.build()?;
        let res = self.service.call(request).await?;
        parse_json_response!(res, WriteDataResponse)
    }
    /// 读取数据
    async fn read_data(
        &mut self,
        project_name: &str,
        topic_name: &str,
        shard_id: &str,
        read_data_payload: &ReadDataPayload,
    ) -> DHResult<ReadDataResponse> {
        let path = format!("/projects/{project_name}/topics/{topic_name}/shards/{shard_id}");
        let url = self.endpoint.join(&path)?;
        let mut request_builder = self.client.post(url);
        request_builder = request_builder.json(read_data_payload);
        let request = request_builder.build()?;
        let res = self.service.call(request).await?;
        parse_json_response!(res, ReadDataResponse)
    }

    /// 创建订阅
    async fn create_subscriptions(
        &mut self,
        project_name: &str,
        topic_name: &str,
        comment: &str,
    ) -> DHResult<CreateSubscriptionRes> {
        let path = format!("/projects/{project_name}/topics/{topic_name}/subscriptions");
        let url = self.endpoint.join(&path)?;
        let mut request_builder = self.client.post(url);

        let create_subscription_payload: CreateSubscriptionPayload =
            CreateSubscriptionPayload::new(comment);

        request_builder = request_builder.json(&create_subscription_payload);
        let request = request_builder.build()?;
        let res = self.service.call(request).await?;
        parse_json_response!(res, CreateSubscriptionRes)
    }

    /// 查询订阅
    async fn get_subscription(
        &mut self,
        project_name: &str,
        topic_name: &str,
        sub_id: &str,
    ) -> DHResult<GetSubscriptionRes> {
        let path = format!("/projects/{project_name}/topics/{topic_name}/subscriptions/{sub_id}");
        let url = self.endpoint.join(&path)?;
        let request = Request::new(Method::GET, url);
        let res = self.service.call(request).await?;
        parse_json_response!(res, GetSubscriptionRes)
    }

    /// 查询订阅列表
    async fn list_subscriptions(
        &mut self,
        project_name: &str,
        topic_name: &str,
        page_index: u32,
        page_size: u32,
    ) -> DHResult<ListSubscriptionRes> {
        let path = format!("/projects/{project_name}/topics/{topic_name}/subscriptions");
        let url = self.endpoint.join(&path)?;
        let mut request_builder = self.client.post(url);

        let list_subscription_payload: ListSubscriptionsPayload =
            ListSubscriptionsPayload::new(page_index, page_size);

        request_builder = request_builder.json(&list_subscription_payload);
        let request = request_builder.build()?;
        let res = self.service.call(request).await?;
        parse_json_response!(res, ListSubscriptionRes)
    }

    /// 删除订阅
    async fn delete_subscription(
        &mut self,
        project_name: &str,
        topic_name: &str,
        sub_id: &str,
    ) -> DHResult<EmptyResponse> {
        let path = format!("/projects/{project_name}/topics/{topic_name}/subscriptions/{sub_id}");
        let url = self.endpoint.join(&path)?;
        let request = Request::new(Method::DELETE, url);
        let res = self.service.call(request).await?;
        parse_empty_response!(res)
    }

    /// 更新订阅状态
    async fn set_subscription_state(
        &mut self,
        project_name: &str,
        topic_name: &str,
        sub_id: &str,
        state: i32,
    ) -> DHResult<EmptyResponse> {
        let path = format!("/projects/{project_name}/topics/{topic_name}/subscriptions/{sub_id}");
        let url = self.endpoint.join(&path)?;
        let mut request_builder = self.client.post(url);

        let set_subscription_state_payload = SetSubscriptionStatePayload::new(state);

        request_builder = request_builder.json(&set_subscription_state_payload);
        let request = request_builder.build()?;
        let res = self.service.call(request).await?;
        parse_json_response!(res, EmptyResponse)
    }
    /// open点位session
    async fn open_subscription_session(
        &mut self,
        project_name: &str,
        topic_name: &str,
        sub_id: &str,
        shard_ids: &[&str],
    ) -> DHResult<SubscriptionSessionOptRes> {
        let path =
            format!("/projects/{project_name}/topics/{topic_name}/subscriptions/{sub_id}/offsets");
        let url = self.endpoint.join(&path)?;
        let mut request_builder = self.client.post(url);

        let open_subscription_session_payload = SubscriptionSessionOptPayload::open(shard_ids);

        request_builder = request_builder.json(&open_subscription_session_payload);
        let request = request_builder.build()?;
        let res = self.service.call(request).await?;
        parse_json_response!(res, SubscriptionSessionOptRes)
    }

    /// 查询点位
    async fn get_subscription_offset(
        &mut self,
        project_name: &str,
        topic_name: &str,
        sub_id: &str,
        shard_ids: &[&str],
    ) -> DHResult<SubscriptionSessionOptRes> {
        let path =
            format!("/projects/{project_name}/topics/{topic_name}/subscriptions/{sub_id}/offsets");
        let url = self.endpoint.join(&path)?;
        let mut request_builder = self.client.post(url);

        let open_subscription_session_payload = SubscriptionSessionOptPayload::get(shard_ids);

        request_builder = request_builder.json(&open_subscription_session_payload);
        let request = request_builder.build()?;
        let res = self.service.call(request).await?;
        parse_json_response!(res, SubscriptionSessionOptRes)
    }

    /// 提交点位
    async fn commit_subscription_offset(
        &mut self,
        project_name: &str,
        topic_name: &str,
        sub_id: &str,
        offset_map: &HashMap<String, SubscriptionOffset>,
    ) -> DHResult<EmptyResponse> {
        let path =
            format!("/projects/{project_name}/topics/{topic_name}/subscriptions/{sub_id}/offsets");
        let url = self.endpoint.join(&path)?;
        let mut request_builder = self.client.post(url);

        request_builder = request_builder.json(&offset_map);
        let request = request_builder.build()?;
        let res = self.service.call(request).await?;
        parse_empty_response!(res)
    }
}
