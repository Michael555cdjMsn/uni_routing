
//! Actix-web服务器示例

use actix_web::{get, post, web, App, HttpServer, HttpResponse, Result as ActixResult};
use serde::{Deserialize, Serialize};
use uni_routing::middleware::{MiddlewareChain, AuthMiddleware, LoggingMiddleware, CorsMiddleware};
use uni_routing::auth::AuthPolicy;
use std::sync::Arc;

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

// 健康检查端点（无需认证）
#[get("/api/health")]
async fn health_check() -> ActixResult<HttpResponse> {
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "status": "healthy",
        "timestamp": chrono::Utc::now().to_rfc3339()
    })))
}

// 获取用户列表端点
#[get("/api/users")]
async fn get_users() -> ActixResult<HttpResponse> {
    let users = vec![
        User { id: 1, name: "Alice".to_string(), email: "alice@example.com".to_string() },
        User { id: 2, name: "Bob".to_string(), email: "bob@example.com".to_string() },
        User { id: 3, name: "Charlie".to_string(), email: "charlie@example.com".to_string() },
    ];
    
    Ok(HttpResponse::Ok().json(users))
}

// 创建用户端点
#[post("/api/users")]
async fn create_user(
    user_data: web::Json<CreateUserRequest>
) -> ActixResult<HttpResponse> {
    let new_user = User {
        id: 999,
        name: user_data.name.clone(),
        email: user_data.email.clone(),
    };
    
    Ok(HttpResponse::Created().json(new_user))
}

// Swagger文档端点
#[get("/swagger")]
async fn swagger_ui() -> ActixResult<HttpResponse> {
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
    
    Ok(HttpResponse::Ok()
        .content_type("text/html")
        .body(html))
}

#[get("/swagger/openapi.json")]
async fn openapi_spec() -> ActixResult<HttpResponse> {
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
    
    Ok(HttpResponse::Ok().json(spec))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    
    println!("🚀 Starting Actix-web server with uni_routing...");
    
    // 设置中间件链（演示用途）
    let auth_policy = AuthPolicy::new("scoped:scope1,scope2");
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
    
    HttpServer::new(|| {
        App::new()
            .service(health_check)
            .service(get_users)
            .service(create_user)
            .service(swagger_ui)
            .service(openapi_spec)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}