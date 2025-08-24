use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub status: i32,
    pub msg: String,
    pub results: Option<T>,
}
