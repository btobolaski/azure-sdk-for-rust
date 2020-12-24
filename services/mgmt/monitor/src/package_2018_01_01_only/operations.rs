#![doc = "generated by AutoRust 0.1.0"]
#![allow(unused_mut)]
#![allow(unused_variables)]
#![allow(unused_imports)]
use crate::models::*;
use reqwest::StatusCode;
use snafu::{ResultExt, Snafu};
pub mod metric_definitions {
    use crate::models::*;
    use reqwest::StatusCode;
    use snafu::{ResultExt, Snafu};
    pub async fn list(
        operation_config: &crate::OperationConfig,
        resource_uri: &str,
        metricnamespace: Option<&str>,
    ) -> std::result::Result<MetricDefinitionCollection, list::Error> {
        let client = &operation_config.client;
        let uri_str = &format!(
            "{}/{}/providers/microsoft.insights/metricDefinitions",
            &operation_config.base_path, resource_uri
        );
        let mut req_builder = client.get(uri_str);
        if let Some(token_credential) = &operation_config.token_credential {
            let token_response = token_credential
                .get_token(&operation_config.token_credential_resource)
                .await
                .context(list::GetTokenError)?;
            req_builder = req_builder.bearer_auth(token_response.token.secret());
        }
        req_builder = req_builder.query(&[("api-version", &operation_config.api_version)]);
        if let Some(metricnamespace) = metricnamespace {
            req_builder = req_builder.query(&[("metricnamespace", metricnamespace)]);
        }
        let req = req_builder.build().context(list::BuildRequestError)?;
        let rsp = client.execute(req).await.context(list::ExecuteRequestError)?;
        match rsp.status() {
            StatusCode::OK => {
                let body: bytes::Bytes = rsp.bytes().await.context(list::ResponseBytesError)?;
                let rsp_value: MetricDefinitionCollection = serde_json::from_slice(&body).context(list::DeserializeError { body })?;
                Ok(rsp_value)
            }
            status_code => {
                let body: bytes::Bytes = rsp.bytes().await.context(list::ResponseBytesError)?;
                let rsp_value: ErrorResponse = serde_json::from_slice(&body).context(list::DeserializeError { body })?;
                list::DefaultResponse {
                    status_code,
                    value: rsp_value,
                }
                .fail()
            }
        }
    }
    pub mod list {
        use crate::{models, models::*};
        use reqwest::StatusCode;
        use snafu::Snafu;
        #[derive(Debug, Snafu)]
        #[snafu(visibility(pub(crate)))]
        pub enum Error {
            DefaultResponse {
                status_code: StatusCode,
                value: models::ErrorResponse,
            },
            BuildRequestError {
                source: reqwest::Error,
            },
            ExecuteRequestError {
                source: reqwest::Error,
            },
            ResponseBytesError {
                source: reqwest::Error,
            },
            DeserializeError {
                source: serde_json::Error,
                body: bytes::Bytes,
            },
            GetTokenError {
                source: azure_core::errors::AzureError,
            },
        }
    }
}
pub mod metrics {
    use crate::models::*;
    use reqwest::StatusCode;
    use snafu::{ResultExt, Snafu};
    pub async fn list(
        operation_config: &crate::OperationConfig,
        resource_uri: &str,
        timespan: Option<&str>,
        interval: Option<&str>,
        metricnames: Option<&str>,
        aggregation: Option<&str>,
        top: Option<i32>,
        orderby: Option<&str>,
        filter: Option<&str>,
        result_type: Option<&str>,
        metricnamespace: Option<&str>,
    ) -> std::result::Result<Response, list::Error> {
        let client = &operation_config.client;
        let uri_str = &format!(
            "{}/{}/providers/microsoft.insights/metrics",
            &operation_config.base_path, resource_uri
        );
        let mut req_builder = client.get(uri_str);
        if let Some(token_credential) = &operation_config.token_credential {
            let token_response = token_credential
                .get_token(&operation_config.token_credential_resource)
                .await
                .context(list::GetTokenError)?;
            req_builder = req_builder.bearer_auth(token_response.token.secret());
        }
        req_builder = req_builder.query(&[("api-version", &operation_config.api_version)]);
        if let Some(timespan) = timespan {
            req_builder = req_builder.query(&[("timespan", timespan)]);
        }
        if let Some(interval) = interval {
            req_builder = req_builder.query(&[("interval", interval)]);
        }
        if let Some(metricnames) = metricnames {
            req_builder = req_builder.query(&[("metricnames", metricnames)]);
        }
        if let Some(aggregation) = aggregation {
            req_builder = req_builder.query(&[("aggregation", aggregation)]);
        }
        if let Some(top) = top {
            req_builder = req_builder.query(&[("top", top)]);
        }
        if let Some(orderby) = orderby {
            req_builder = req_builder.query(&[("orderby", orderby)]);
        }
        if let Some(filter) = filter {
            req_builder = req_builder.query(&[("$filter", filter)]);
        }
        if let Some(result_type) = result_type {
            req_builder = req_builder.query(&[("resultType", result_type)]);
        }
        if let Some(metricnamespace) = metricnamespace {
            req_builder = req_builder.query(&[("metricnamespace", metricnamespace)]);
        }
        let req = req_builder.build().context(list::BuildRequestError)?;
        let rsp = client.execute(req).await.context(list::ExecuteRequestError)?;
        match rsp.status() {
            StatusCode::OK => {
                let body: bytes::Bytes = rsp.bytes().await.context(list::ResponseBytesError)?;
                let rsp_value: Response = serde_json::from_slice(&body).context(list::DeserializeError { body })?;
                Ok(rsp_value)
            }
            status_code => {
                let body: bytes::Bytes = rsp.bytes().await.context(list::ResponseBytesError)?;
                let rsp_value: ErrorResponse = serde_json::from_slice(&body).context(list::DeserializeError { body })?;
                list::DefaultResponse {
                    status_code,
                    value: rsp_value,
                }
                .fail()
            }
        }
    }
    pub mod list {
        use crate::{models, models::*};
        use reqwest::StatusCode;
        use snafu::Snafu;
        #[derive(Debug, Snafu)]
        #[snafu(visibility(pub(crate)))]
        pub enum Error {
            DefaultResponse {
                status_code: StatusCode,
                value: models::ErrorResponse,
            },
            BuildRequestError {
                source: reqwest::Error,
            },
            ExecuteRequestError {
                source: reqwest::Error,
            },
            ResponseBytesError {
                source: reqwest::Error,
            },
            DeserializeError {
                source: serde_json::Error,
                body: bytes::Bytes,
            },
            GetTokenError {
                source: azure_core::errors::AzureError,
            },
        }
    }
}