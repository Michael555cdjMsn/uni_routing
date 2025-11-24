//! 中间件模块的单元测试

use uni_routing::middleware::{LoggingMiddleware, CorsMiddleware, AuthMiddleware, MiddlewareContext, Middleware};
use uni_routing::auth::{AuthPolicy, JwtToken};
use uni_routing::Error;
use serde_json::json;

#[test]
fn test_logging_middleware() {
    let middleware = LoggingMiddleware;
    let mut context = MiddlewareContext {
        token: None,
        path: "/api/users".to_string(),
        method: "GET".to_string(),
    };
    
    let result = middleware.handle(&mut context);
    assert!(result.is_ok());
}

#[test]
fn test_cors_middleware() {
    let middleware = CorsMiddleware::default();
    let mut context = MiddlewareContext {
        token: None,
        path: "/api/users".to_string(),
        method: "GET".to_string(),
    };
    
    let result = middleware.handle(&mut context);
    assert!(result.is_ok());
}

#[test]
fn test_auth_middleware_without_token() {
    let policy = AuthPolicy::new("roles:admin");
    let middleware = AuthMiddleware::new(policy);
    let mut context = MiddlewareContext {
        token: None,
        path: "/api/users".to_string(),
        method: "GET".to_string(),
    };
    
    let result = middleware.handle(&mut context);
    assert!(matches!(result, Err(Error::Unauthorized)));
}

#[test]
fn test_auth_middleware_with_token() {
    let policy = AuthPolicy::new("roles:admin");
    let middleware = AuthMiddleware::new(policy);
    
    let mut payload = std::collections::HashMap::new();
    payload.insert("sub".to_string(), json!("123"));
    let token = JwtToken { payload };
    
    let mut context = MiddlewareContext {
        token: Some(token),
        path: "/api/users".to_string(),
        method: "GET".to_string(),
    };
    
    let result = middleware.handle(&mut context);
    assert!(result.is_ok());
}

#[test]
fn test_auth_middleware_empty_policy() {
    let policy = AuthPolicy::new("");
    let middleware = AuthMiddleware::new(policy);
    let mut context = MiddlewareContext {
        token: None,
        path: "/api/users".to_string(),
        method: "GET".to_string(),
    };
    
    let result = middleware.handle(&mut context);
    assert!(result.is_ok());
}