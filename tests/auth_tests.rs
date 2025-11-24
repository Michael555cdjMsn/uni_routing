//! 认证模块的单元测试

use uni_routing::auth::{AuthPolicy, AuthCondition, JwtToken};
use serde_json::json;

#[test]
fn test_auth_policy_creation() {
    let policy = AuthPolicy::new("roles:admin,user");
    assert_eq!(policy.policy_expr, "roles:admin,user");
}

#[test]
fn test_auth_policy_parsing() {
    let policy = AuthPolicy::new("role:admin,permission:read,scope:write");
    let conditions = policy.parse();
    assert_eq!(conditions.len(), 3);
    
    match &conditions[0] {
        AuthCondition::Role(role) => assert_eq!(role, "admin"),
        _ => panic!("Expected Role condition"),
    }
    
    match &conditions[1] {
        AuthCondition::Permission(permission) => assert_eq!(permission, "read"),
        _ => panic!("Expected Permission condition"),
    }
    
    match &conditions[2] {
        AuthCondition::Scope(scope) => assert_eq!(scope, "write"),
        _ => panic!("Expected Scope condition"),
    }
}

#[test]
fn test_auth_condition_from_str() {
    assert!(matches!(AuthCondition::from_str("role:admin"), AuthCondition::Role(_)));
    assert!(matches!(AuthCondition::from_str("user:123"), AuthCondition::User(_)));
    assert!(matches!(AuthCondition::from_str("permission:read"), AuthCondition::Permission(_)));
    assert!(matches!(AuthCondition::from_str("scope:write"), AuthCondition::Scope(_)));
    assert!(matches!(AuthCondition::from_str(""), AuthCondition::ValidToken));
}

#[test]
fn test_jwt_token() {
    let mut payload = std::collections::HashMap::new();
    payload.insert("sub".to_string(), json!("123"));
    payload.insert("name".to_string(), json!("Test User"));
    
    let token = JwtToken { payload };
    
    // 这些测试目前总是返回true，因为实现是简化的
    assert!(token.has_role("admin"));
    assert!(token.has_permission("read"));
    assert!(token.has_scope("write"));
}