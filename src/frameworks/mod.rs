//! 框架特定的路由注册器

#[cfg(feature = "actix")]
pub mod actix;

#[cfg(feature = "axum")]
pub mod axum;

#[cfg(feature = "rocket")]
pub mod rocket;