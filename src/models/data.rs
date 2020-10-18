use crate::models::{ExceptionData, MessageData, RemoteDependencyData, RequestData};
use serde::Serialize;

/// Data struct to contain both B and C sections.
#[derive(Debug, Serialize)]
#[serde(tag = "baseType", content = "baseData")]
pub(crate) enum Data {
    #[serde(rename = "ExceptionData")]
    Exception(ExceptionData),
    #[serde(rename = "MessageData")]
    Message(MessageData),
    #[serde(rename = "RemoteDependencyData")]
    RemoteDependency(RemoteDependencyData),
    #[serde(rename = "RequestData")]
    Request(RequestData),
}
