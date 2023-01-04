#![allow(clippy::enum_variant_names)]

use dozer_api::errors::GRPCError;
use dozer_cache::errors::CacheError;
use dozer_core::dag::errors::ExecutionError;
use dozer_ingestion::errors::ConnectorError;
use dozer_sql::pipeline::errors::PipelineError;
use dozer_types::crossbeam::channel::RecvError;
use dozer_types::errors::internal::BoxedError;
use dozer_types::thiserror::Error;
use dozer_types::{serde_yaml, thiserror};

#[derive(Error, Debug)]
pub enum OrchestrationError {
    #[error("Failed to write config yaml: {0:?}")]
    FailedToWriteConfigYaml(#[source] serde_yaml::Error),
    #[error("Failed to initialize. {0}[/api/generated,/cache] are not empty. Use -f to clean the directory and overwrite. Warning! there will be data loss.")]
    InitializationFailed(String),
    #[error("Failed to initialize pipeline_dir. Is the path {0:?} accessible?: {1}")]
    PipelineDirectoryInitFailed(String, #[source] std::io::Error),
    #[error("Can't locate pipeline_dir. Has dozer been initialized(dozer init) ?")]
    PipelineDirectoryNotFound(String),
    #[error("Failed to generate token: {0:?}")]
    GenerateTokenFailed(String),
    #[error("Failed to initialize api server: {0}")]
    ApiServerFailed(#[source] std::io::Error),
    #[error("Failed to initialize grpc server: {0}")]
    GrpcServerFailed(#[source] GRPCError),
    #[error(
        "{0}: Failed to initialize read only cache. Has dozer been initialized (`dozer init`)?"
    )]
    CacheInitFailed(#[source] CacheError),
    #[error(transparent)]
    InternalError(#[from] BoxedError),
    #[error(transparent)]
    ExecutionError(#[from] ExecutionError),
    #[error(transparent)]
    ConnectorError(#[from] ConnectorError),
    #[error(transparent)]
    PipelineError(#[from] PipelineError),
    #[error(transparent)]
    CliError(#[from] CliError),
    #[error("Failed to receive server handle from grpc server: {0}")]
    GrpcServerHandleError(#[source] RecvError),
}

#[derive(Error, Debug)]
pub enum CliError {
    #[error("Can't find the configuration file at: {0:?}")]
    FailedToLoadFile(String),
    #[error("Failed to parse dozer config: {0:?}")]
    FailedToParseYaml(#[source] BoxedError),
    #[error("Failed to validate dozer config: {0:?}")]
    FailedToParseValidateYaml(#[source] BoxedError),
}
