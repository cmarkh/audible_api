use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

pub type Result<T, E = AppError> = core::result::Result<T, E>;

#[derive(Debug)]
pub struct AppError(Box<dyn std::error::Error + Send + Sync>);

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Error: {}", self.0),
        )
            .into_response()
    }
}

impl<E> From<E> for AppError
where
    E: Into<Box<dyn std::error::Error + Send + Sync>>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}
