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
pub struct QueryRequest {
    pub id: String,
    pub sql_statement_id: String,
    pub user_id: String,
    pub tags: HashMap<String, Option<String>>,
    pub max_age_minutes: u64,
    #[serde(rename = "resultTTLHours")]
    pub result_ttl_hours: u64,
    pub user_skip_cache: bool,
    pub triggered_query_run: bool,
    pub query_run_id: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FileNames {
    Single(String),
    Multiple(Vec<String>),
}

impl Into<Vec<String>> for FileNames {
    fn into(self) -> Vec<String> {
        match self {
            FileNames::Single(s) => vec![s],
            FileNames::Multiple(v) => v,
        }
    }
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
    pub file_names: Option<FileNames>,
    pub error_name: Option<String>,
    pub error_message: Option<String>,
    pub error_data: Option<String>,
    pub external_query_id: Option<String>,
    pub data_source_query_id: Option<String>,
    pub data_source_session_id: Option<String>,
    pub started_at: Option<String>,
    pub query_running_ended_at: Option<String>,
    pub query_streaming_ended_at: Option<String>,
    pub ended_at: Option<String>,
    pub row_count: Option<usize>,
    pub total_size: Option<String>,
    pub tags: HashMap<String, Option<String>>,
    pub data_source_id: String,
    pub user_id: String,
    pub created_at: String,
    pub updated_at: String,
    pub archived_at: Option<String>,
    pub rows_per_result_set: usize,
    pub statement_timeout_seconds: u64,
    pub abort_detached_query: bool,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ColumnMetadata {
    types: Vec<String>,
    columns: Vec<String>,
    col_type_map: HashMap<String, String>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SqlStatement {
    pub id: String,
    pub statement_hash: String,
    pub sql: String,
    pub column_metadata: ColumnMetadata,
    pub user_id: String,
    pub tags: HashMap<String, Option<String>>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CreateQueryRunResult {
    pub query_request: QueryRequest,
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
    Array,
    Boolean,
    Unknown,
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

#[derive(Clone, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetQueryRunResultsParams {
    pub query_run_id: String,
    pub format: QueryFormat,
    pub sort_by: Vec<SortBy>,
    pub filters: Vec<HashMap<FilterKey, String>>,
    pub page: Option<Pagination>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CreateQueryRunParams {
    #[serde(rename = "resultTTLHours")]
    pub result_ttl_hours: u64,
    pub max_age_minutes: u64,
    pub sql: String,
    pub tags: HashMap<String, Option<String>>,
    pub data_source: String,
    pub data_provider: String,
}

#[derive(Clone, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct QueryRunIdParams {
    pub query_run_id: String,
}

#[rpc(client)]
pub trait Rpc {
    #[method(name = "getQueryRunResults")]
    async fn get_query_run_results(
        &self,
        params: GetQueryRunResultsParams,
    ) -> RpcResult<GetQueryRunResultsResult>;

    #[method(name = "createQueryRun")]
    async fn create_query_run(
        &self,
        params: CreateQueryRunParams,
    ) -> RpcResult<CreateQueryRunResult>;

    #[method(name = "getQueryRun")]
    async fn get_query_run(&self, params: QueryRunIdParams) -> RpcResult<GetQueryRunResult>;

    #[method(name = "cancelQueryRun")]
    async fn cancel_query_run(&self, params: QueryRunIdParams) -> RpcResult<CancelQueryRunResult>;
}
