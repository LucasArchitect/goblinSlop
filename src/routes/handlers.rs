use serde::Serialize;

/// Shared API response type used by all JSON endpoints.
#[derive(Serialize)]
pub struct ApiResponse<T: Serialize> {
    pub success: bool,
    pub data: T,
}