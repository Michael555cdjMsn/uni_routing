//! 路由模块，用于处理不同Web框架的统一API路由

use crate::auth::AuthPolicy;
use crate::Error;

/// 路由信息结构体
#[derive(Debug, Clone)]
pub struct RouteInfo {
    /// 路由路径
    pub path: String,
    /// HTTP方法
    pub method: HttpMethod,
    /// 认证策略
    pub auth_policy: Option<AuthPolicy>,
    /// 路由描述
    pub description: Option<String>,
}

/// HTTP方法枚举
#[derive(Debug, Clone, PartialEq)]
pub enum HttpMethod {
    GET,
    POST,
    PUT,
    DELETE,
    PATCH,
}

impl HttpMethod {
    /// 从字符串解析HTTP方法
    pub fn from_str(method: &str) -> Self {
        match method.to_uppercase().as_str() {
            "GET" => HttpMethod::GET,
            "POST" => HttpMethod::POST,
            "PUT" => HttpMethod::PUT,
            "DELETE" => HttpMethod::DELETE,
            "PATCH" => HttpMethod::PATCH,
            _ => HttpMethod::GET, // 默认为GET
        }
    }
}

impl std::fmt::Display for HttpMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HttpMethod::GET => write!(f, "GET"),
            HttpMethod::POST => write!(f, "POST"),
            HttpMethod::PUT => write!(f, "PUT"),
            HttpMethod::DELETE => write!(f, "DELETE"),
            HttpMethod::PATCH => write!(f, "PATCH"),
        }
    }
}

/// 统一路由特征，用于适配不同的Web框架
pub trait UnifiedRouter {
    /// 添加路由
    fn add_route(&mut self, route_info: RouteInfo, handler: Box<dyn Fn()>);
    
    /// 启动服务
    fn serve(self) -> Result<(), Error>;
}