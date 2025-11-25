//! # Uni Routing Framework
//! 
//! 一个为 Rust 设计的统一 API 框架，支持多种 web 服务器，
//! 提供统一的路由、认证和 OpenAPI 文档生成功能。

/// 核心模块
pub mod routing;
/// 认证模块
pub mod auth;
/// 中间件模块
pub mod middleware;
/// 路由注册模块
pub mod registry;

#[cfg(feature = "swagger")]
/// OpenAPI 文档生成模块
pub mod swagger;

/// 框架特定的路由注册器
#[cfg(any(feature = "actix", feature = "axum", feature = "rocket"))]
pub mod frameworks;

/// 重新导出常用的类型
pub use serde::{Deserialize, Serialize};

/// 错误类型
#[derive(Debug)]
pub enum Error {
    /// 权限错误
    Unauthorized,
    /// 其他错误
    Other(String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Unauthorized => write!(f, "Unauthorized"),
            Error::Other(msg) => write!(f, "{}", msg),
        }
    }
}

impl std::error::Error for Error {}