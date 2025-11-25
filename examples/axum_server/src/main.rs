//! Axum服务器示例 - 展示 uni_routing 宏的使用

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
    routing::{get, post, put, delete},
    Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uni_routing_macros::uni_routing;
use uni_routing::middleware::{MiddlewareChain, AuthMiddleware, LoggingMiddleware, CorsMiddleware};
use uni_routing::auth::AuthPolicy;

#[cfg(feature = "swagger")]
use utoipa::OpenApi;

#[derive(Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "swagger", derive(utoipa::ToSchema))]
struct AppState {
    // 应用状态可以在这里添加
}

#[derive(Serialize, Deserialize)]
#[cfg_attr(feature = "swagger", derive(utoipa::ToSchema))]
struct User {
    id: u64,
    name: String,
    email: String,
}

#[derive(Deserialize)]
#[cfg_attr(feature = "swagger", derive(utoipa::ToSchema))]
struct CreateUserRequest {
    name: String,
    email: String,
}

// 使用 uni_routing 宏定义健康检查端点（无需认证）
#[cfg_attr(feature = "swagger", utoipa::path(
    get,
    path = "/api/health",
    responses(
        (status = 200, description = "服务器健康状态")
    )
))]
#[uni_routing(
    route = "/api/health",
    method = "GET",
    description = "健康检查端点，检查服务器运行状态"
)]
async fn health_check() -> Result<Json<serde_json::Value>, StatusCode> {
    Ok(Json(serde_json::json!({
        "status": "healthy",
        "timestamp": chrono::Utc::now().to_rfc3339()
    })))
}

// 使用 uni_routing 宏定义获取用户列表端点（需要管理员权限）
#[cfg_attr(feature = "swagger", utoipa::path(
    get,
    path = "/api/users",
    responses(
        (status = 200, description = "用户列表", body = [User])
    ),
    security(
        ("bearerAuth" = [])
    )
))]
#[uni_routing(
    route = "/api/users",
    method = "GET",
    auth_policy = "role:admin,permission:users.read",
    description = "获取所有用户列表，需要管理员权限和用户读取权限"
)]
async fn get_users() -> Result<Json<Vec<User>>, StatusCode> {
    let users = vec![
        User { id: 1, name: "Alice".to_string(), email: "alice@example.com".to_string() },
        User { id: 2, name: "Bob".to_string(), email: "bob@example.com".to_string() },
        User { id: 3, name: "Charlie".to_string(), email: "charlie@example.com".to_string() },
    ];
    
    Ok(Json(users))
}

// 使用 uni_routing 宏定义创建用户端点（需要用户管理权限）
#[cfg_attr(feature = "swagger", utoipa::path(
    post,
    path = "/api/users",
    request_body = CreateUserRequest,
    responses(
        (status = 201, description = "创建的用户", body = User)
    ),
    security(
        ("bearerAuth" = [])
    )
))]
#[uni_routing(
    route = "/api/users",
    method = "POST",
    auth_policy = "role:admin,permission:users.write",
    description = "创建新用户，需要管理员权限和用户写入权限"
)]
async fn create_user(
    Json(user_data): Json<CreateUserRequest>
) -> Result<(StatusCode, Json<User>), StatusCode> {
    let new_user = User {
        id: 999,
        name: user_data.name,
        email: user_data.email,
    };
    
    Ok((StatusCode::CREATED, Json(new_user)))
}

// 使用 uni_routing 宏定义获取单个用户端点
#[cfg_attr(feature = "swagger", utoipa::path(
    get,
    path = "/api/users/{id}",
    responses(
        (status = 200, description = "特定用户信息", body = User)
    ),
    params(
        ("id" = u64, Path, description = "用户ID")
    ),
    security(
        ("bearerAuth" = [])
    )
))]
#[uni_routing(
    route = "/api/users/{id}",
    method = "GET",
    auth_policy = "role:admin,permission:users.read",
    description = "根据ID获取特定用户信息"
)]
async fn get_user_by_id(
    Path(user_id): Path<u64>
) -> Result<Json<User>, StatusCode> {
    let user = User {
        id: user_id,
        name: format!("User {}", user_id),
        email: format!("user{}@example.com", user_id),
    };
    
    Ok(Json(user))
}

// 使用 uni_routing 宏定义更新用户端点
#[cfg_attr(feature = "swagger", utoipa::path(
    put,
    path = "/api/users/{id}",
    request_body = CreateUserRequest,
    responses(
        (status = 200, description = "更新的用户", body = User)
    ),
    params(
        ("id" = u64, Path, description = "用户ID")
    ),
    security(
        ("bearerAuth" = [])
    )
))]
#[uni_routing(
    route = "/api/users/{id}",
    method = "PUT",
    auth_policy = "role:admin,permission:users.write",
    description = "更新指定ID的用户信息"
)]
async fn update_user(
    Path(user_id): Path<u64>,
    Json(user_data): Json<CreateUserRequest>
) -> Result<Json<User>, StatusCode> {
    let updated_user = User {
        id: user_id,
        name: user_data.name,
        email: user_data.email,
    };
    
    Ok(Json(updated_user))
}

// 使用 uni_routing 宏定义删除用户端点
#[cfg_attr(feature = "swagger", utoipa::path(
    delete,
    path = "/api/users/{id}",
    responses(
        (status = 200, description = "用户删除成功")
    ),
    params(
        ("id" = u64, Path, description = "用户ID")
    ),
    security(
        ("bearerAuth" = [])
    )
))]
#[uni_routing(
    route = "/api/users/{id}",
    method = "DELETE",
    auth_policy = "role:admin,permission:users.delete",
    description = "删除指定ID的用户"
)]
async fn delete_user(
    Path(user_id): Path<u64>
) -> Result<Json<serde_json::Value>, StatusCode> {
    Ok(Json(serde_json::json!({
        "message": format!("User {} deleted successfully", user_id)
    })))
}

#[cfg(feature = "swagger")]
#[derive(OpenApi)]
#[openapi(
    paths(
        health_check,
        get_users,
        create_user,
        get_user_by_id,
        update_user,
        delete_user,
    ),
    components(schemas(User, CreateUserRequest)),
    tags(
        (name = "users", description = "用户管理端点")
    ),
    security(
        ("bearerAuth" = [])
    )
)]
struct ApiDoc;

// Swagger UI 端点
#[cfg(feature = "swagger")]
async fn swagger_ui() -> Result<String, StatusCode> {
    let html = r#"
<!DOCTYPE html>
<html>
<head>
    <title>Uni Routing API Documentation</title>
    <link rel="stylesheet" type="text/css" href="https://unpkg.com/swagger-ui-dist@4/swagger-ui.css" />
</head>
<body>
    <div id="swagger-ui"></div>
    <script src="https://unpkg.com/swagger-ui-dist@4/swagger-ui-bundle.js"></script>
    <script>
        SwaggerUIBundle({
            url: '/swagger/openapi.json',
            dom_id: '#swagger-ui'
        });
    </script>
</body>
</html>
    "#;
    
    Ok(html.to_string())
}

// OpenAPI 规范端点
#[cfg(feature = "swagger")]
async fn openapi_spec() -> Result<Json<serde_json::Value>, StatusCode> {
    let spec = ApiDoc::openapi().to_json().unwrap();
    Ok(Json(serde_json::from_str(&spec).unwrap()))
}

#[tokio::main]
async fn main() {
    // 初始化日志
    tracing_subscriber::fmt::init();
    
    println!("🚀 Starting Axum server with uni_routing...");
    
    // 设置中间件链（演示用途）
    let auth_policy = AuthPolicy::new("scoped:scope1,scope2");
    let _middleware_chain = MiddlewareChain::new()
        .add_middleware(Arc::new(LoggingMiddleware))
        .add_middleware(Arc::new(CorsMiddleware::default()))
        .add_middleware(Arc::new(AuthMiddleware::new(auth_policy)));
        
    println!("✅ Middleware chain configured");
    
    // 创建应用状态
    let app_state = Arc::new(AppState {});
    
    println!("🌐 Server starting on http://localhost:8080");
    println!();
    println!("📖 Available endpoints (using uni_routing macro):");
    println!("  GET    http://localhost:8080/api/health           - Health check (no auth)");
    println!("  GET    http://localhost:8080/api/users            - Get all users (admin:read)");
    println!("  POST   http://localhost:8080/api/users            - Create user (admin:write)");
    println!("  GET    http://localhost:8080/api/users/123        - Get user by ID (admin:read)");
    println!("  PUT    http://localhost:8080/api/users/123        - Update user (admin:write)");
    println!("  DELETE http://localhost:8080/api/users/123        - Delete user (admin:delete)");
    
    #[cfg(feature = "swagger")]
    println!("  GET    http://localhost:8080/swagger              - Swagger UI");
    #[cfg(feature = "swagger")]
    println!("  GET    http://localhost:8080/swagger/openapi.json  - OpenAPI spec");
    
    println!();
    println!("🧪 Try these commands:");
    println!("  curl -X GET http://localhost:8080/api/health");
    println!("  curl -X GET http://localhost:8080/api/users");
    println!("  curl -X POST http://localhost:8080/api/users -H 'Content-Type: application/json' -d '{\"name\":\"Test User\",\"email\":\"test@example.com\"}'");
    println!("  curl -X GET http://localhost:8080/api/users/123");
    println!("  curl -X PUT http://localhost:8080/api/users/123 -H 'Content-Type: application/json' -d '{\"name\":\"Updated User\",\"email\":\"updated@example.com\"}'");
    println!("  curl -X DELETE http://localhost:8080/api/users/123");
    println!();
    println!("📝 Note: The uni_routing macro provides route metadata and authentication");
    println!("   configuration, but actual routing is handled by Axum's native system.");
    
    // 创建路由
    let app = {
        #[cfg(feature = "swagger")]
        {
            Router::new()
                // 使用 Axum 原生路由注册，但展示了 uni_routing 宏的配置
                .route("/api/health", get(health_check))
                .route("/api/users", get(get_users).post(create_user))
                .route("/api/users/:id", get(get_user_by_id).put(update_user).delete(delete_user))
                .route("/swagger", get(swagger_ui))
                .route("/swagger/openapi.json", get(openapi_spec))
                .with_state(app_state)
        }
        
        #[cfg(not(feature = "swagger"))]
        {
            Router::new()
                // 使用 Axum 原生路由注册，但展示了 uni_routing 宏的配置
                .route("/api/health", get(health_check))
                .route("/api/users", get(get_users).post(create_user))
                .route("/api/users/:id", get(get_user_by_id).put(update_user).delete(delete_user))
                .with_state(app_state)
        }
    };
    
    // 启动服务器
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080").await.unwrap();
    println!("Server running on http://localhost:8080");
    axum::serve(listener, app).await.unwrap();
}