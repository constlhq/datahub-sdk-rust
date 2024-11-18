use crate::errors::DHResult;
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
use crate::payload::cursor::CursorType;
use crate::payload::data::{ReadDataPayload, WriteDataPayload};
use crate::payload::topics::CreateTopicPayload;
use async_trait::async_trait;
use std::collections::HashMap;

#[async_trait]
pub trait DatahubClientTrait {
    /// 查询Project列表
    async fn list_project(&mut self) -> DHResult<ListProjectResponse>;

    /// 创建Project
    async fn create_project(
        &mut self,
        project_name: &str,
        comment: &str,
    ) -> DHResult<EmptyResponse>;
    /// 查询Project
    async fn get_project(&mut self, project_name: &str) -> DHResult<GetProjectResponse>;

    /// 更新Project
    async fn update_project(
        &mut self,
        project_name: &str,
        comment: &str,
    ) -> DHResult<EmptyResponse>;
    /// 删除Project
    async fn delete_project(&mut self, project_name: &str) -> DHResult<EmptyResponse>;

    /// 创建Topic
    async fn create_topic(
        &mut self,
        project_name: &str,
        topic_name: &str,
        create_topic_payload: &CreateTopicPayload,
    ) -> DHResult<EmptyResponse>;

    /// 查询Topic
    async fn get_topic(
        &mut self,
        project_name: &str,
        topic_name: &str,
    ) -> DHResult<GetTopicResponse>;

    /// 查询Topic列表
    async fn list_topic(&mut self, project_name: &str) -> DHResult<ListTopicResponse>;

    ///更新Topic
    async fn update_topic(
        &mut self,
        project_name: &str,
        topic_name: &str,
        comment: &str,
    ) -> DHResult<EmptyResponse>;

    /// 删除Topic
    async fn delete_topic(
        &mut self,
        project_name: &str,
        topic_name: &str,
    ) -> DHResult<EmptyResponse>;

    ///新增Field
    async fn append_filed(
        &mut self,
        project_name: &str,
        topic_name: &str,
        field_name: &str,
        field_type: FieldType,
    ) -> DHResult<EmptyResponse>;

    ///获取Shard列表
    async fn list_shard(
        &mut self,
        project_name: &str,
        topic_name: &str,
    ) -> DHResult<ListShardResponse>;

    /// 分裂Shard
    async fn split_shard(
        &mut self,
        project_name: &str,
        topic_name: &str,
        shard_id: &str,
        split_key: &str,
    ) -> DHResult<SplitShardResponse>;

    ///合并Shard
    async fn merge_shard(
        &mut self,
        project_name: &str,
        topic_name: &str,
        shard_id: &str,
        adjacent_shard_id: &str,
    ) -> DHResult<MergeShardResponse>;

    /// 查询数据Cursor
    async fn get_cursor(
        &mut self,
        project_name: &str,
        topic_name: &str,
        shard_id: &str,
        cursor_type: CursorType,
        parameter: i64,
    ) -> DHResult<QueryCursorResponse>;

    /// 写入数据
    async fn write_data(
        &mut self,
        project_name: &str,
        topic_name: &str,
        write_data_payload: &WriteDataPayload,
    ) -> DHResult<WriteDataResponse>;
    /// 读取数据
    async fn read_data(
        &mut self,
        project_name: &str,
        topic_name: &str,
        shard_id: &str,
        read_data_payload: &ReadDataPayload,
    ) -> DHResult<ReadDataResponse>;

    /// 创建订阅
    async fn create_subscriptions(
        &mut self,
        project_name: &str,
        topic_name: &str,
        comment: &str,
    ) -> DHResult<CreateSubscriptionRes>;

    /// 查询订阅
    async fn get_subscription(
        &mut self,
        project_name: &str,
        topic_name: &str,
        sub_id: &str,
    ) -> DHResult<GetSubscriptionRes>;

    /// 查询订阅列表
    async fn list_subscriptions(
        &mut self,
        project_name: &str,
        topic_name: &str,
        page_index: u32,
        page_size: u32,
    ) -> DHResult<ListSubscriptionRes>;

    /// 删除订阅
    async fn delete_subscription(
        &mut self,
        project_name: &str,
        topic_name: &str,
        sub_id: &str,
    ) -> DHResult<EmptyResponse>;

    /// 更新订阅状态
    async fn set_subscription_state(
        &mut self,
        project_name: &str,
        topic_name: &str,
        sub_id: &str,
        state: i32,
    ) -> DHResult<EmptyResponse>;
    /// open点位session
    async fn open_subscription_session(
        &mut self,
        project_name: &str,
        topic_name: &str,
        sub_id: &str,
        shard_ids: &[&str],
    ) -> DHResult<SubscriptionSessionOptRes>;

    /// 查询点位
    async fn get_subscription_offset(
        &mut self,
        project_name: &str,
        topic_name: &str,
        sub_id: &str,
        shard_ids: &[&str],
    ) -> DHResult<SubscriptionSessionOptRes>;

    /// 提交点位
    async fn commit_subscription_offset(
        &mut self,
        project_name: &str,
        topic_name: &str,
        sub_id: &str,
        offset_map: &HashMap<String, SubscriptionOffset>,
    ) -> DHResult<EmptyResponse>;
}
