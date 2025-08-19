use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct UserInfo {
    pub user_id: usize,
}

#[derive(Deserialize, Serialize)]
pub struct UserPayload {
    pub user_name: String,
    pub user_email: String,
    pub user_address: String,
}

#[derive(Serialize, Deserialize)]
pub struct UserDetail {
    pub user_info: UserInfo,
    pub user_payload: UserPayload,
}
