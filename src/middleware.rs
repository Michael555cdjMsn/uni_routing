//! 中间件模块，提供统一的中间件处理机制

use crate::auth::{AuthPolicy, JwtToken};
use crate::Error;
use std::sync::Arc;

/// 中间件特征
pub trait Middleware: Send + Sync {
    /// 处理请求
    fn handle(&self, context: &mut MiddlewareContext) -> Result<(), Error>;
}

/// 中间件上下文
pub struct MiddlewareContext {
    /// JWT令牌（如果已认证）
    pub token: Option<JwtToken>,
    /// 请求路径
    pub path: String,
    /// HTTP方法
    pub method: String,
}

/// 中间件链
pub struct MiddlewareChain {
    pub middlewares: Vec<Arc<dyn Middleware>>,
}

impl MiddlewareChain {
    /// 创建新的中间件链
    pub fn new() -> Self {
        Self {
            middlewares: Vec::new(),
        }
    }
    
    /// 添加中间件到链中
    pub fn add_middleware(mut self, middleware: Arc<dyn Middleware>) -> Self {
        self.middlewares.push(middleware);
        self
    }
    
    /// 执行中间件链
    pub fn execute(&self, context: &mut MiddlewareContext) -> Result<(), Error> {
        for middleware in &self.middlewares {
            middleware.handle(context)?;
        }
        Ok(())
    }
}

impl Default for MiddlewareChain {
    fn default() -> Self {
        Self::new()
    }
}

/// 日志中间件
#[derive(Clone)]
pub struct LoggingMiddleware;

impl Middleware for LoggingMiddleware {
    fn handle(&self, context: &mut MiddlewareContext) -> Result<(), Error> {
        println!("Request: {} {}", context.method, context.path);
        Ok(())
    }
}

/// CORS中间件
#[derive(Clone)]
pub struct CorsMiddleware;

impl CorsMiddleware {
    pub fn default() -> Self {
        Self
    }
}

impl Middleware for CorsMiddleware {
    fn handle(&self, _context: &mut MiddlewareContext) -> Result<(), Error> {
        // 在实际实现中会处理CORS头
        Ok(())
    }
}

/// 认证中间件
#[derive(Clone)]
pub struct AuthMiddleware {
    policy: AuthPolicy,
}

impl AuthMiddleware {
    pub fn new(policy: AuthPolicy) -> Self {
        Self { policy }
    }
}

impl Middleware for AuthMiddleware {
    fn handle(&self, context: &mut MiddlewareContext) -> Result<(), Error> {
        // 如果有认证策略且没有令牌，则返回未授权错误
        if !self.policy.policy_expr.is_empty() && context.token.is_none() {
            return Err(Error::Unauthorized);
        }
        Ok(())
    }
}