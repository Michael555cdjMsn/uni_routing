//! Rocket服务器示例 - 展示 uni_routing 宏的使用

use rocket::{get, post, serde::json::Json};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uni_routing_macros::uni_routing;
use uni_routing::middleware::{MiddlewareChain, AuthMiddleware, LoggingMiddleware, CorsMiddleware};
use uni_routing::auth::AuthPolicy;

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
#[get("/api/health")]
async fn health_check() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "status": "healthy",
        "timestamp": chrono::Utc::now().to_rfc3339()
    }))
}

// 使用 uni_routing 宏定义获取用户列表端点（需要管理员权限）
#[uni_routing(
    route = "/api/users",
    method = "GET",
    auth_policy = "role:admin,permission:users.read",
    description = "获取所有用户列表，需要管理员权限和用户读取权限"
)]
#[get("/api/users")]
async fn get_users() -> Json<Vec<User>> {
    let users = vec![
        User { id: 1, name: "Alice".to_string(), email: "alice@example.com".to_string() },
        User { id: 2, name: "Bob".to_string(), email: "bob@example.com".to_string() },
        User { id: 3, name: "Charlie".to_string(), email: "charlie@example.com".to_string() },
    ];
    
    Json(users)
}

// 使用 uni_routing 宏定义创建用户端点（需要用户管理权限）
#[uni_routing(
    route = "/api/users",
    method = "POST",
    auth_policy = "role:admin,permission:users.write",
    description = "创建新用户，需要管理员权限和用户写入权限"
)]
#[post("/api/users", format = "json")]
async fn create_user(
    user_data: Json<CreateUserRequest>
) -> (rocket::http::Status, Json<User>) {
    let new_user = User {
        id: 999,
        name: user_data.name.clone(),
        email: user_data.email.clone(),
    };
    
    (rocket::http::Status::Created, Json(new_user))
}

// Swagger UI 端点
#[get("/swagger")]
async fn swagger_ui() -> String {
    r#"
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
    "#.to_string()
}

// OpenAPI 规范端点
#[get("/swagger/openapi.json")]
async fn openapi_spec() -> Json<serde_json::Value> {
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
    
    Json(spec)
}

#[rocket::main]
async fn main() {
    // 初始化日志
    env_logger::init();
    
    println!("🚀 Starting Rocket server with uni_routing...");
    
    // 设置中间件链（演示用途）
    let auth_policy = AuthPolicy::new("scoped:scope1,scope2");
    let _middleware_chain = MiddlewareChain::new()
        .add_middleware(Arc::new(LoggingMiddleware))
        .add_middleware(Arc::new(CorsMiddleware::default()))
        .add_middleware(Arc::new(AuthMiddleware::new(auth_policy)));
        
    println!("✅ Middleware chain configured");
    
    println!("🌐 Server starting on http://localhost:8080");
    println!();
    println!("📖 Available endpoints (using uni_routing macro):");
    println!("  GET  http://localhost:8080/api/health           - Health check (no auth)");
    println!("  GET  http://localhost:8080/api/users            - Get all users (admin:read)");
    println!("  POST http://localhost:8080/api/users            - Create user (admin:write)");
    println!("  GET  http://localhost:8080/swagger              - Swagger UI");
    println!("  GET  http://localhost:8080/swagger/openapi.json  - OpenAPI spec");
    println!();
    println!("🧪 Try these commands:");
    println!("  curl -X GET http://localhost:8080/api/health");
    println!("  curl -X GET http://localhost:8080/api/users");
    println!("  curl -X POST http://localhost:8080/api/users -H 'Content-Type: application/json' -d '{{\"name\":\"Test User\",\"email\":\"test@example.com\"}}'");
    println!();
    println!("📝 Note: The uni_routing macro provides route metadata and authentication");
    println!("   configuration, but actual routing is handled by Rocket's native system.");
    
    // 配置并启动 Rocket - 使用简化的路由注册
    let _rocket = rocket::build()
        // 使用 Rocket 原生路由注册，但展示了 uni_routing 宏的配置
        .mount("/api/health", routes![health_check])
        .mount("/api/users", routes![get_users, create_user])
        .mount("/swagger", routes![swagger_ui])
        .mount("/swagger/openapi.json", routes![openapi_spec])
        .configure(rocket::Config::figment().merge(("port", 8080)))
        .launch()
        .await;
}