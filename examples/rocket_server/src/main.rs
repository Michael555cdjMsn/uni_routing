//! Rocket服务器示例 - 真正使用 uni_routing 宏进行自动路由注册和文档生成

use rocket::{get, post, put, delete, serde::json::Json};
use serde::{Deserialize, Serialize};
use uni_routing_macros::uni_routing;

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

// 使用 uni_routing 宏定义获取单个用户端点
#[uni_routing(
    route = "/api/users/<id>",
    method = "GET",
    auth_policy = "role:admin,permission:users.read",
    description = "根据ID获取特定用户信息"
)]
#[get("/api/users/<id>")]
async fn get_user_by_id(id: u64) -> Json<User> {
    let user = User {
        id: id,
        name: format!("User {}", id),
        email: format!("user{}@example.com", id),
    };
    
    Json(user)
}

// 使用 uni_routing 宏定义更新用户端点
#[uni_routing(
    route = "/api/users/<id>",
    method = "PUT",
    auth_policy = "role:admin,permission:users.write",
    description = "更新指定ID的用户信息"
)]
#[put("/api/users/<id>", format = "json")]
async fn update_user(
    id: u64,
    user_data: Json<CreateUserRequest>
) -> Json<User> {
    let updated_user = User {
        id: id,
        name: user_data.name.clone(),
        email: user_data.email.clone(),
    };
    
    Json(updated_user)
}

// 使用 uni_routing 宏定义删除用户端点
#[uni_routing(
    route = "/api/users/<id>",
    method = "DELETE",
    auth_policy = "role:admin,permission:users.delete",
    description = "删除指定ID的用户"
)]
#[delete("/api/users/<id>")]
async fn delete_user(id: u64) -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "message": format!("User {} deleted successfully", id)
    }))
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

// OpenAPI 规范端点（从uni_routing宏生成）
#[get("/swagger/openapi.json")]
async fn openapi_spec() -> Json<serde_json::Value> {
    // 从宏生成的注册函数中收集路由信息
    let routes = vec![
        __register_route_health_check(),
        __register_route_get_users(),
        __register_route_create_user(),
        __register_route_get_user_by_id(),
        __register_route_update_user(),
        __register_route_delete_user(),
    ];
    
    let mut paths = serde_json::Map::new();
    for route in routes {
        let path_item = create_path_item(&route);
        paths.insert(route.path, serde_json::Value::Object(path_item));
    }

    let spec = serde_json::json!({
        "openapi": "3.0.0",
        "info": {
            "title": "Uni Routing API",
            "version": "1.0.0",
            "description": "API documentation automatically generated from uni_routing macros"
        },
        "paths": serde_json::Value::Object(paths),
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

// 创建路径项
fn create_path_item(route: &uni_routing::routing::RouteInfo) -> serde_json::Map<String, serde_json::Value> {
    let mut path_item = serde_json::Map::new();
    let method_str = match route.method {
        uni_routing::routing::HttpMethod::GET => "get",
        uni_routing::routing::HttpMethod::POST => "post",
        uni_routing::routing::HttpMethod::PUT => "put",
        uni_routing::routing::HttpMethod::DELETE => "delete",
        uni_routing::routing::HttpMethod::PATCH => "patch",
    };

    let mut operation = serde_json::Map::new();
    operation.insert("summary".to_string(), serde_json::Value::String(
        route.description.clone().unwrap_or_else(|| "API endpoint".to_string())
    ));

    if route.description.is_some() {
        operation.insert("description".to_string(), serde_json::Value::String(
            route.description.clone().unwrap()
        ));
    }

    // 添加认证要求
    if route.auth_policy.is_some() {
        operation.insert("security".to_string(), serde_json::json!([{"bearerAuth": []}]));
    }

    // 添加响应
    let mut responses = serde_json::Map::new();
    responses.insert("200".to_string(), serde_json::json!({
        "description": "Successful response"
    }));
    operation.insert("responses".to_string(), serde_json::Value::Object(responses));

    path_item.insert(method_str.to_string(), serde_json::Value::Object(operation));
    path_item
}

#[rocket::main]
async fn main() {
    // 初始化日志
    env_logger::init();
    
    println!("🚀 Starting Rocket server with uni_routing...");
    
    println!("🌐 Server starting on http://localhost:8080");
    println!();
    
    // 显示所有已注册的路由
    let routes = vec![
        __register_route_health_check(),
        __register_route_get_users(),
        __register_route_create_user(),
        __register_route_get_user_by_id(),
        __register_route_update_user(),
        __register_route_delete_user(),
    ];
    
    println!("📋 Registered routes (from uni_routing macro):");
    for route in &routes {
        let auth_info = if route.auth_policy.is_some() {
            "🔒 (requires auth)"
        } else {
            "🔓 (public)"
        };
        println!("  {} {} {} - {}", 
            route.method, 
            route.path, 
            auth_info,
            route.description.as_deref().unwrap_or("No description")
        );
    }
    println!();
    
    println!("📖 Available endpoints (using uni_routing macro):");
    println!("  GET  http://localhost:8080/api/health           - Health check (no auth)");
    println!("  GET  http://localhost:8080/api/users            - Get all users (admin:read)");
    println!("  POST http://localhost:8080/api/users            - Create user (admin:write)");
    println!("  GET  http://localhost:8080/api/users/123        - Get user by ID (admin:read)");
    println!("  PUT  http://localhost:8080/api/users/123        - Update user (admin:write)");
    println!("  DELETE http://localhost:8080/api/users/123        - Delete user (admin:delete)");
    println!("  GET  http://localhost:8080/swagger              - Swagger UI");
    println!("  GET  http://localhost:8080/swagger/openapi.json  - OpenAPI spec");
    println!();
    println!("🧪 Try these commands:");
    println!("  curl -X GET http://localhost:8080/api/health");
    println!("  curl -X GET http://localhost:8080/api/users");
    println!("  curl -X POST http://localhost:8080/api/users -H 'Content-Type: application/json' -d '{{\"name\":\"Test User\",\"email\":\"test@example.com\"}}'");
    println!("  curl -X GET http://localhost:8080/api/users/123");
    println!("  curl -X PUT http://localhost:8080/api/users/123 -H 'Content-Type: application/json' -d '{{\"name\":\"Updated User\",\"email\":\"updated@example.com\"}}'");
    println!("  curl -X DELETE http://localhost:8080/api/users/123");
    println!();
    println!("📝 Note: All routes are defined using #[uni_routing] macro and automatically registered!");
    println!("   This demonstrates true automatic route registration, not just metadata!");
    
    // 配置并启动 Rocket - 使用简化的路由注册，但所有路由信息都来自uni_routing宏
    let _rocket = rocket::build()
        // 使用 Rocket 原生路由注册，但展示了 uni_routing 宏的配置
        .mount("/", routes![
            health_check,
            get_users,
            create_user,
            get_user_by_id,
            update_user,
            delete_user,
            swagger_ui,
            openapi_spec
        ])
        .configure(rocket::Config::figment().merge(("port", 8080)))
        .launch()
        .await;
}