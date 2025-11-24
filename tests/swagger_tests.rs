//! Swagger模块的单元测试

#[cfg(feature = "swagger")]
use uni_routing::swagger::SwaggerGenerator;
#[cfg(feature = "swagger")]
use uni_routing::routing::{RouteInfo, HttpMethod};
#[cfg(feature = "swagger")]
use uni_routing::auth::AuthPolicy;

#[test]
#[cfg(feature = "swagger")]
fn test_swagger_generator_creation() {
    let generator = SwaggerGenerator::new();
    assert_eq!(generator.route_count(), 0);
}

#[test]
#[cfg(feature = "swagger")]
fn test_swagger_generator_add_route() {
    let mut generator = SwaggerGenerator::new();
    
    let route = RouteInfo {
        path: "/api/users".to_string(),
        method: HttpMethod::GET,
        auth_policy: Some(AuthPolicy::new("roles:admin")),
        description: Some("Get all users".to_string()),
    };
    
    generator.add_route(route);
    assert_eq!(generator.route_count(), 1);
}

#[test]
#[cfg(feature = "swagger")]
fn test_generate_openapi_spec() {
    let generator = SwaggerGenerator::new();
    let spec = generator.generate_openapi_spec();
    
    assert!(spec.contains("openapi"));
    assert!(spec.contains("3.0.0"));
    assert!(spec.contains("Uni Routing API"));
}

#[test]
#[cfg(feature = "swagger")]
fn test_generate_swagger_ui() {
    let generator = SwaggerGenerator::new();
    let ui_html = generator.generate_swagger_ui();
    
    assert!(ui_html.contains("swagger-ui"));
    assert!(ui_html.contains("SwaggerUIBundle"));
    assert!(ui_html.contains("/swagger/openapi.json"));
}