//! 集成测试，验证整个项目的功能

use uni_routing::auth::{AuthPolicy, AuthCondition, JwtToken};
use uni_routing::routing::{RouteInfo, HttpMethod};
use uni_routing::middleware::{AuthMiddleware, MiddlewareContext, Middleware};
use uni_routing::Error;
use serde_json::json;

#[test]
fn test_complete_auth_flow() {
    // 创建认证策略
    let policy = AuthPolicy::new("role:admin,permission:read,scope:write");
    
    // 解析策略
    let conditions = policy.parse();
    assert_eq!(conditions.len(), 3);
    
    // 验证条件类型
    assert!(matches!(conditions[0], AuthCondition::Role(_)));
    assert!(matches!(conditions[1], AuthCondition::Permission(_)));
    assert!(matches!(conditions[2], AuthCondition::Scope(_)));
    
    // 创建 JWT 令牌
    let mut payload = std::collections::HashMap::new();
    payload.insert("sub".to_string(), json!("123"));
    payload.insert("name".to_string(), json!("Test User"));
    let token = JwtToken { payload };
    
    // 验证令牌功能
    assert!(token.has_role("admin"));
    assert!(token.has_permission("read"));
    assert!(token.has_scope("write"));
}

#[test]
fn test_middleware_chain_integration() {
    let auth_policy = AuthPolicy::new("role:admin");
    let middleware = AuthMiddleware::new(auth_policy);
    
    // 测试无令牌的情况
    let mut context = MiddlewareContext {
        token: None,
        path: "/api/users".to_string(),
        method: "GET".to_string(),
    };
    
    let result = middleware.handle(&mut context);
    assert!(matches!(result, Err(Error::Unauthorized)));
    
    // 测试有令牌的情况
    let mut payload = std::collections::HashMap::new();
    payload.insert("sub".to_string(), json!("123"));
    let token = JwtToken { payload };
    
    context.token = Some(token);
    let result = middleware.handle(&mut context);
    assert!(result.is_ok());
}

#[test]
fn test_route_info_comprehensive() {
    let route = RouteInfo {
        path: "/api/users/{id}".to_string(),
        method: HttpMethod::GET,
        auth_policy: Some(AuthPolicy::new("role:admin,permission:read")),
        description: Some("Get user by ID".to_string()),
    };
    
    // 验证路由信息
    assert_eq!(route.path, "/api/users/{id}");
    assert!(matches!(route.method, HttpMethod::GET));
    assert!(route.auth_policy.is_some());
    assert!(route.description.is_some());
    
    let policy = route.auth_policy.unwrap();
    assert_eq!(policy.policy_expr, "role:admin,permission:read");
    
    let description = route.description.unwrap();
    assert_eq!(description, "Get user by ID");
}

#[test]
fn test_error_handling() {
    // 测试错误显示
    let error = Error::Unauthorized;
    assert_eq!(error.to_string(), "Unauthorized");
    
    let error = Error::Other("Custom error".to_string());
    assert_eq!(error.to_string(), "Custom error");
}

#[test]
fn test_http_method_conversions() {
    // 测试字符串到 HTTP 方法的转换
    assert!(matches!(HttpMethod::from_str("GET"), HttpMethod::GET));
    assert!(matches!(HttpMethod::from_str("POST"), HttpMethod::POST));
    assert!(matches!(HttpMethod::from_str("PUT"), HttpMethod::PUT));
    assert!(matches!(HttpMethod::from_str("DELETE"), HttpMethod::DELETE));
    assert!(matches!(HttpMethod::from_str("PATCH"), HttpMethod::PATCH));
    
    // 测试大小写不敏感
    assert!(matches!(HttpMethod::from_str("get"), HttpMethod::GET));
    assert!(matches!(HttpMethod::from_str("post"), HttpMethod::POST));
    
    // 测试默认情况
    assert!(matches!(HttpMethod::from_str("INVALID"), HttpMethod::GET));
}

#[test]
fn test_auth_condition_edge_cases() {
    // 测试空字符串
    assert!(matches!(AuthCondition::from_str(""), AuthCondition::ValidToken));
    
    // 测试无效前缀
    assert!(matches!(AuthCondition::from_str("invalid:prefix"), AuthCondition::ValidToken));
    
    // 测试只有前缀
    assert!(matches!(AuthCondition::from_str("role:"), AuthCondition::Role(_)));
    assert!(matches!(AuthCondition::from_str("user:"), AuthCondition::User(_)));
    assert!(matches!(AuthCondition::from_str("permission:"), AuthCondition::Permission(_)));
    assert!(matches!(AuthCondition::from_str("scope:"), AuthCondition::Scope(_)));
}