use serde::Serialize;

#[derive(Serialize)]
pub struct ApiResponse<T> {
    pub status: i32,
    pub msg: String,
    pub results: Option<T>,
}
