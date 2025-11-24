//! Rocket服务器示例

use rocket::{get, post, routes, launch, serde::json::Json, response::content};
use rocket::http::Status;
use serde::{Deserialize, Serialize};
use uni_routing::middleware::{MiddlewareChain, AuthMiddleware, LoggingMiddleware, CorsMiddleware};
use uni_routing::auth::AuthPolicy;
use std::sync::Arc;

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct User {
    id: u64,
    name: String,
    email: String,
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
struct CreateUserRequest {
    name: String,
    email: String,
}

// 健康检查端点
#[get("/api/health")]
fn health_check() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "status": "healthy",
        "timestamp": chrono::Utc::now().to_rfc3339()
    }))
}

// 获取用户列表端点
#[get("/api/users")]
fn get_users() -> Json<Vec<User>> {
    let users = vec![
        User { id: 1, name: "Alice".to_string(), email: "alice@example.com".to_string() },
        User { id: 2, name: "Bob".to_string(), email: "bob@example.com".to_string() },
        User { id: 3, name: "Charlie".to_string(), email: "charlie@example.com".to_string() },
    ];
    
    Json(users)
}

// 创建用户端点
#[post("/api/users", format = "json", data = "<user_data>")]
fn create_user(user_data: Json<CreateUserRequest>) -> (Status, Json<User>) {
    let new_user = User {
        id: 999,
        name: user_data.name.clone(),
        email: user_data.email.clone(),
    };
    
    (Status::Created, Json(new_user))
}

// Swagger UI端点
#[get("/swagger")]
fn swagger_ui() -> content::RawHtml<String> {
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
    "#.to_string();
    
    content::RawHtml(html)
}

// OpenAPI规范端点
#[get("/swagger/openapi.json")]
fn openapi_spec() -> Json<serde_json::Value> {
    let spec = serde_json::json!({
        "openapi": "3.0.0",
        "info": {
            "title": "Uni Routing API",
            "version": "1.0.0",
            "description": "API documentation for Uni Routing framework"
        },
        "paths": {
            "/api/health": {
                "get": {
                    "summary": "Health check",
                    "description": "Check if the server is running",
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
                    "description": "Retrieve a list of all users",
                    "responses": {
                        "200": {
                            "description": "List of users"
                        }
                    }
                },
                "post": {
                    "summary": "Create a new user",
                    "description": "Create a new user with the provided data",
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
        }
    });
    
    Json(spec)
}

#[launch]
fn rocket() -> _ {
    println!("🚀 Starting Rocket server with uni_routing...");
    
    // 设置中间件链（演示用途）
    let auth_policy = AuthPolicy::new("roles:admins,users");
    let _middleware_chain = MiddlewareChain::new()
        .add_middleware(Arc::new(LoggingMiddleware))
        .add_middleware(Arc::new(CorsMiddleware::default()))
        .add_middleware(Arc::new(AuthMiddleware::new(auth_policy)));
        
    println!("✅ Middleware chain configured");
    println!("🌐 Server starting on http://localhost:8080");
    println!();
    println!("📖 Available endpoints:");
    println!("  GET  http://localhost:8080/api/health      - Health check (no auth)");
    println!("  GET  http://localhost:8080/api/users       - Get all users");
    println!("  POST http://localhost:8080/api/users       - Create user");
    println!("  GET  http://localhost:8080/swagger         - Swagger UI");
    println!("  GET  http://localhost:8080/swagger/openapi.json - OpenAPI spec");
    println!();
    println!("🧪 Try these commands:");
    println!("  curl -X GET http://localhost:8080/api/health");
    println!("  curl -X GET http://localhost:8080/api/users");
    println!("  curl -X POST http://localhost:8080/api/users -H 'Content-Type: application/json' -d '{{\"name\":\"Test User\",\"email\":\"test@example.com\"}}'");

    rocket::build()
        .mount("/", routes![
            health_check,
            get_users,
            create_user,
            swagger_ui,
            openapi_spec
        ])
}