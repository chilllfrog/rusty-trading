use thiserror::Error;

#[derive(Error, Debug, Clone, PartialEq)]
pub enum LimiterError {
    #[error("acquire timeout:{0}")]
    TIMEOUT(u64),
    #[error("insuffient token left:{0}")]
    INSUFFICIENTOKEN(u64),
}

pub type Result<T> = std::result::Result<T, LimiterError>;
