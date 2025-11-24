//! 路由模块的单元测试

use uni_routing::routing::{RouteInfo, HttpMethod};
use uni_routing::auth::AuthPolicy;

#[test]
fn test_route_info_creation() {
    let route = RouteInfo {
        path: "/api/users".to_string(),
        method: HttpMethod::GET,
        auth_policy: Some(AuthPolicy::new("roles:admin")),
        description: Some("Get all users".to_string()),
    };
    
    assert_eq!(route.path, "/api/users");
    assert!(matches!(route.method, HttpMethod::GET));
    assert_eq!(route.auth_policy.unwrap().policy_expr, "roles:admin");
    assert_eq!(route.description.unwrap(), "Get all users");
}

#[test]
fn test_http_method_display() {
    assert_eq!(HttpMethod::GET.to_string(), "GET");
    assert_eq!(HttpMethod::POST.to_string(), "POST");
    assert_eq!(HttpMethod::PUT.to_string(), "PUT");
    assert_eq!(HttpMethod::DELETE.to_string(), "DELETE");
    assert_eq!(HttpMethod::PATCH.to_string(), "PATCH");
}

#[test]
fn test_http_method_from_str() {
    assert!(matches!(HttpMethod::from_str("GET"), HttpMethod::GET));
    assert!(matches!(HttpMethod::from_str("POST"), HttpMethod::POST));
    assert!(matches!(HttpMethod::from_str("PUT"), HttpMethod::PUT));
    assert!(matches!(HttpMethod::from_str("DELETE"), HttpMethod::DELETE));
    assert!(matches!(HttpMethod::from_str("PATCH"), HttpMethod::PATCH));
    // 默认情况
    assert!(matches!(HttpMethod::from_str("INVALID"), HttpMethod::GET));
}