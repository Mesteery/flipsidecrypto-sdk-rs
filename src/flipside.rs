use crate::defaults::{
    API_BASE_URL, DATA_PROVIDER, DATA_SOURCE, MAX_AGE_MINUTES, RETRY_INTERVAL, TIMEOUT, TTL_MINUTES,
};
use crate::rpc::{
    FilterKey, GetQueryRunResultsResult, Pagination, QueryFormat, QueryRun, QueryState, RpcClient,
    SortBy,
};
use jsonrpsee::core::ClientError;
use jsonrpsee::http_client::{HeaderMap, HttpClient, HttpClientBuilder};
use std::collections::HashMap;
use std::time::{Duration, Instant};

pub struct Query {
    /// SQL query to execute
    sql: String,
    /// the maximum age of the query results in minutes you will accept, defaults to zero
    max_age_minutes: Option<u64>,
    /// An override on the cache. A value of true will reexecute the query.
    cached: Option<bool>,
    /// The number of minutes until your query times out
    timeout: Option<Duration>,
    /// The number of seconds to use between retries
    retry_interval_seconds: Option<Duration>,
    /// The data source to execute the query against
    data_source: Option<String>,
    /// The owner of the data source
    data_provider: Option<String>,
}

pub struct ExecutionError {
    pub name: String,
    pub message: String,
    pub data: String,
}

pub enum QueryRunError {
    RpcError(ClientError),
    Timeout(Duration),
    ExecutionError(ExecutionError),
}

#[derive(Clone)]
pub struct Flipside(HttpClient);

impl Flipside {
    pub fn new(api_key: String, base_url: Option<String>) -> Result<Self, ClientError> {
        let mut headers = HeaderMap::new();
        headers.insert("x-api-key", api_key.parse().unwrap());

        Ok(Self(
            HttpClientBuilder::default()
                .set_headers(headers)
                .build(base_url.unwrap_or(API_BASE_URL.to_string()))?,
        ))
    }

    fn get_timeout(&self, query: &Query) -> Duration {
        query.timeout.unwrap_or(TIMEOUT)
    }

    fn get_retry_interval_seconds(&self, query: &Query) -> Duration {
        query.retry_interval_seconds.unwrap_or(RETRY_INTERVAL)
    }

    #[inline]
    fn get_max_age_minutes(&self, query: &Query) -> u64 {
        if query.cached == Some(false) {
            return 0;
        }
        query.max_age_minutes.unwrap_or(MAX_AGE_MINUTES)
    }

    #[inline]
    fn get_ttl_hours(&self, query: &Query) -> u64 {
        let max_age_minutes = self.get_max_age_minutes(query);
        (if max_age_minutes > 60 {
            max_age_minutes
        } else {
            TTL_MINUTES
        }) / 60
    }

    pub async fn run(&self, query: Query) -> Result<QueryRun, QueryRunError> {
        let ttl_hours = self.get_ttl_hours(&query);
        let max_age_minutes = self.get_max_age_minutes(&query);
        let retry_interval = self.get_retry_interval_seconds(&query);
        let timeout = self.get_timeout(&query);

        let mut query_run = self
            .0
            .create_query_run(
                ttl_hours,
                max_age_minutes,
                query.sql,
                HashMap::new(),
                query.data_source.unwrap_or(DATA_SOURCE.to_string()),
                query.data_provider.unwrap_or(DATA_PROVIDER.to_string()),
            )
            .await
            .map_err(QueryRunError::RpcError)?
            .query_run;

        let query_run_id = query_run.id;

        let mut retry_duration = retry_interval;
        let start = Instant::now();

        loop {
            let res = self
                .0
                .get_query_run(query_run_id.clone())
                .await
                .map_err(QueryRunError::RpcError)?;

            query_run = res.redirected_to_query_run.unwrap_or(res.query_run);

            match query_run.state {
                QueryState::QueryStateSuccess => break,

                QueryState::QueryStateFailed | QueryState::QueryStateCancelled => {
                    return Err(QueryRunError::ExecutionError(ExecutionError {
                        name: query_run.error_name.unwrap(),
                        message: query_run.error_message.unwrap(),
                        data: query_run.error_data.unwrap(),
                    }));
                }

                _ => {}
            };

            tokio::time::sleep(retry_duration).await;
            retry_duration += retry_interval;

            let elapsed = start.elapsed();
            if elapsed > timeout {
                return Err(QueryRunError::Timeout(elapsed));
            }
        }

        Ok(query_run)
    }

    pub async fn create_query_run(&self, query: Query) -> Result<QueryRun, ClientError> {
        Ok(self
            .0
            .create_query_run(
                self.get_ttl_hours(&query),
                self.get_max_age_minutes(&query),
                query.sql,
                HashMap::new(),
                query.data_source.unwrap_or(DATA_SOURCE.to_string()),
                query.data_provider.unwrap_or(DATA_PROVIDER.to_string()),
            )
            .await?
            .query_run)
    }

    pub async fn get_query_run(&self, query_run_id: String) -> Result<QueryRun, ClientError> {
        let res = self.0.get_query_run(query_run_id).await?;
        Ok(res.redirected_to_query_run.unwrap_or(res.query_run))
    }

    pub async fn cancel_query_run(&self, query_run_id: &str) -> Result<QueryRun, ClientError> {
        Ok(self
            .0
            .cancel_query_run(query_run_id.to_string())
            .await?
            .canceled_query_run)
    }

    pub async fn get_query_results(
        &self,
        query_run_id: String,
        page: Option<Pagination>,
        filters: Vec<HashMap<FilterKey, String>>,
        sort_by: Vec<SortBy>,
    ) -> Result<GetQueryRunResultsResult, ClientError> {
        let res = self.0.get_query_run(query_run_id).await?;

        let query_run = res.redirected_to_query_run.unwrap_or(res.query_run);

        self.0
            .get_query_run_results(
                query_run.id,
                QueryFormat::Csv,
                Some(sort_by),
                Some(filters),
                Some(page.unwrap_or(Pagination {
                    number: 1,
                    size: 100000,
                })),
            )
            .await
    }
}
