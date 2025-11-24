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

#[derive(Serialize, Deserialize, Clone)]
struct AppState {
    // 应用状态可以在这里添加
}

#[derive(Serialize, Deserialize)]
struct User {
    id: u64,
    name: String,
    email: String,
}

#[derive(Deserialize)]
struct CreateUserRequest {
    name: String,
    email: String,
}

// 使用 uni_routing 宏定义健康检查端点（无需认证）
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

// Swagger UI 端点
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
async fn openapi_spec() -> Result<Json<serde_json::Value>, StatusCode> {
    let spec = serde_json::json!({
        "openapi": "3.0.0",
        "info": {
            "title": "Uni Routing API",
            "version": "1.0.0",
            "description": "API documentation for Uni Routing framework with uni_routing macro"
        },
        "paths": {
            "/api/health": {
                "get": {
                    "summary": "Health check",
                    "description": "健康检查端点，检查服务器运行状态",
                    "responses": {
                        "200": {
                            "description": "Server is healthy"
                        }
                    }
                }
            },
            
            "/api/users": {
                "get": {
                    "summary": "Get all users",
                    "description": "获取所有用户列表，需要管理员权限和用户读取权限",
                    "security": [{"bearerAuth": []}],
                    "responses": {
                        "200": {
                            "description": "List of users"
                        }
                    }
                },
                "post": {
                    "summary": "Create a new user",
                    "description": "创建新用户，需要管理员权限和用户写入权限",
                    "security": [{"bearerAuth": []}],
                    "requestBody": {
                        "required": true,
                        "content": {
                            "application/json": {
                                "schema": {
                                    "type": "object",
                                    "properties": {
                                        "name": {"type": "string"},
                                        "email": {"type": "string"}
                                    }
                                }
                            }
                        }
                    },
                    "responses": {
                        "201": {
                            "description": "User created successfully"
                        }
                    }
                }
            },
            
            "/api/users/{id}": {
                "get": {
                    "summary": "Get user by ID",
                    "description": "根据ID获取特定用户信息",
                    "security": [{"bearerAuth": []}],
                    "responses": {
                        "200": {
                            "description": "User details"
                        }
                    }
                },
                "put": {
                    "summary": "Update user",
                    "description": "更新指定ID的用户信息",
                    "security": [{"bearerAuth": []}],
                    "responses": {
                        "200": {
                            "description": "User updated successfully"
                        }
                    }
                },
                "delete": {
                    "summary": "Delete user",
                    "description": "删除指定ID的用户",
                    "security": [{"bearerAuth": []}],
                    "responses": {
                        "200": {
                            "description": "User deleted successfully"
                        }
                    }
                }
            }
        },
        "components": {
            "securitySchemes": {
                "bearerAuth": {
                    "type": "http",
                    "scheme": "bearer",
                    "bearerFormat": "JWT"
                }
            }
        }
    });
    
    Ok(Json(spec))
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
    println!("  GET    http://localhost:8080/swagger              - Swagger UI");
    println!("  GET    http://localhost:8080/swagger/openapi.json  - OpenAPI spec");
    println!();
    println!("🧪 Try these commands:");
    println!("  curl -X GET http://localhost:8080/api/health");
    println!("  curl -X GET http://localhost:8080/api/users");
    println!("  curl -X POST http://localhost:8080/api/users -H 'Content-Type: application/json' -d '{{\"name\":\"Test User\",\"email\":\"test@example.com\"}}'");
    println!("  curl -X GET http://localhost:8080/api/users/123");
    println!("  curl -X PUT http://localhost:8080/api/users/123 -H 'Content-Type: application/json' -d '{{\"name\":\"Updated User\",\"email\":\"updated@example.com\"}}'");
    println!("  curl -X DELETE http://localhost:8080/api/users/123");
    println!();
    println!("📝 Note: The uni_routing macro provides route metadata and authentication");
    println!("   configuration, but actual routing is handled by Axum's native system.");
    
    // 创建路由
    let app = Router::new()
        // 使用 Axum 原生路由注册，但展示了 uni_routing 宏的配置
        .route("/api/health", get(health_check))
        .route("/api/users", get(get_users).post(create_user))
        .route("/api/users/:id", get(get_user_by_id).put(update_user).delete(delete_user))
        .route("/swagger", get(swagger_ui))
        .route("/swagger/openapi.json", get(openapi_spec))
        .with_state(app_state);
    
    // 启动服务器
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}