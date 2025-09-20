use color_eyre::eyre::Report;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum LoadBalancerError {
    #[error("Unexpected error")]
    UnexpectedError(#[source] Report),
}
