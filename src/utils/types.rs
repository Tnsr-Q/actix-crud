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

#[derive(Debug, Deserialize, Serialize)]
pub struct RegisterUser {
    pub user_name: String,
    pub sec: String,
    pub user_login: String,
    pub address: String,
}

#[derive(Serialize, Debug, Deserialize)]
pub struct Claims {
    pub sub: i32,
    pub exp: i64,
    pub aud: String,
    pub iss: String,
    pub iat: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserLogin {
    pub user_login: String,
    pub sec: String,
}

#[derive(sqlx::FromRow, Serialize, Deserialize, Debug)]
pub struct UserDetails {
    pub id: i32,
    pub sec: String,
}
