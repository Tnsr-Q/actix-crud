use actix_web::http::{header, Method};

pub const METHODS: &[Method] = &[
    Method::GET,
    Method::POST,
    Method::PUT,
    Method::DELETE,
    Method::PATCH,
];

pub const HEADERS: &[header::HeaderName] =
    &[header::AUTHORIZATION, header::ACCEPT, header::CONTENT_TYPE];
