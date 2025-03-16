use std::collections::HashMap;

use jsonrpsee::proc_macros::rpc;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum QueryState {
    QueryStateReady,
    QueryStateRunning,
    QueryStateSuccess,
    QueryStateFailed,
    QueryStateStreamingResults,
    QueryStateCancelled,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct QueryResult {
    pub id: String,
    pub sql_statement_id: String,
    pub user_id: String,
    pub tags: HashMap<String, String>,
    pub max_age_minutes: u64,
    pub result_ttl_hours: u64,
    pub user_skip_cache: bool,
    pub triggered_query_run: bool,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct QueryRun {
    pub id: String,
    pub sql_statement_id: String,
    pub state: QueryState,
    pub path: String,
    pub file_count: Option<usize>,
    pub last_file_number: Option<usize>,
    pub file_names: Option<Vec<String>>,
    pub error_name: Option<String>,
    pub error_message: Option<String>,
    pub error_data: Option<String>,
    pub data_source_query_id: Option<String>,
    pub data_source_session_id: Option<String>,
    pub started_at: Option<String>,
    pub query_running_ended_at: Option<String>,
    pub query_streaming_ended_at: Option<String>,
    pub ended_at: Option<String>,
    pub row_count: Option<usize>,
    pub total_size: Option<usize>,
    pub tags: HashMap<String, String>,
    pub data_source_id: String,
    pub user_id: String,
    pub created_at: String,
    pub updated_at: String,
    pub archived_at: Option<String>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SqlStatement {
    pub id: String,
    pub statement_hash: String,
    pub sql: String,
    pub column_metadata: Option<String>,
    pub user_id: String,
    pub tags: HashMap<String, String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CreateQueryRunResult {
    pub query_result: QueryResult,
    pub query_run: QueryRun,
    pub sql_statement: SqlStatement,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SortBy {
    pub column: String,
    pub direction: String,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum FilterKey {
    Column,
    Eq,
    Neq,
    Gt,
    Gte,
    Lt,
    Lte,
    Like,
    In,
    NotIn,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Pagination {
    pub number: usize,
    pub size: usize,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum ColumnType {
    String,
    Number,
    Date,
    Object,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PaginationDetails {
    pub current_page_number: usize,
    pub current_page_size: usize,
    pub total_rows: usize,
    pub total_pages: usize,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetQueryRunResultsResult {
    pub column_names: Vec<String>,
    pub column_types: Vec<ColumnType>,
    pub rows: Vec<Value>,
    pub page: PaginationDetails,
    pub original_query_run: QueryRun,
    pub redirected_to_query_run: Option<QueryRun>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetQueryRunResult {
    pub query_run: QueryRun,
    pub redirected_to_query_run: Option<QueryRun>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CancelQueryRunResult {
    pub canceled_query_run: QueryRun,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum QueryFormat {
    Json,
    Csv,
}

#[rpc(client)]
pub trait Rpc {
    #[method(name = "getQueryRunResults")]
    async fn get_query_run_results(
        &self,
        query_run_id: String,
        format: QueryFormat,
        sort_by: Option<Vec<SortBy>>,
        filters: Option<Vec<HashMap<FilterKey, String>>>,
        page: Option<Pagination>,
    ) -> RpcResult<GetQueryRunResultsResult>;

    #[method(name = "createQueryRun")]
    async fn create_query_run(
        &self,
        result_ttl_hours: u64,
        max_age_minutes: u64,
        sql: String,
        tags: HashMap<String, String>,
        data_source: String,
        data_provider: String,
    ) -> RpcResult<CreateQueryRunResult>;

    #[method(name = "getQueryRun")]
    async fn get_query_run(&self, query_run_id: String) -> RpcResult<GetQueryRunResult>;

    #[method(name = "cancelQueryRun")]
    async fn cancel_query_run(&self, query_run_id: String) -> RpcResult<CancelQueryRunResult>;
}
