//! Actix-web服务器示例 - 展示 uni_routing 宏的使用

use actix_web::{web, App, HttpServer, HttpResponse, Result as ActixResult};
use serde::{Deserialize, Serialize};
use uni_routing_macros::uni_routing;
use uni_routing::middleware::{MiddlewareChain, AuthMiddleware, LoggingMiddleware, CorsMiddleware};
use uni_routing::auth::AuthPolicy;
use std::sync::Arc;

#[cfg(feature = "swagger")]
use utoipa::OpenApi;

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
async fn health_check() -> ActixResult<HttpResponse> {
    Ok(HttpResponse::Ok().json(serde_json::json!({
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
async fn get_users() -> ActixResult<HttpResponse> {
    let users = vec![
        User { id: 1, name: "Alice".to_string(), email: "alice@example.com".to_string() },
        User { id: 2, name: "Bob".to_string(), email: "bob@example.com".to_string() },
        User { id: 3, name: "Charlie".to_string(), email: "charlie@example.com".to_string() },
    ];
    
    Ok(HttpResponse::Ok().json(users))
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
    user_data: web::Json<CreateUserRequest>
) -> ActixResult<HttpResponse> {
    let new_user = User {
        id: 999,
        name: user_data.name.clone(),
        email: user_data.email.clone(),
    };
    
    Ok(HttpResponse::Created().json(new_user))
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
    path: web::Path<u64>
) -> ActixResult<HttpResponse> {
    let user_id = path.into_inner();
    
    // 模拟数据库查询
    let user = User {
        id: user_id,
        name: format!("User {}", user_id),
        email: format!("user{}@example.com", user_id),
    };
    
    Ok(HttpResponse::Ok().json(user))
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
    path: web::Path<u64>,
    user_data: web::Json<CreateUserRequest>
) -> ActixResult<HttpResponse> {
    let user_id = path.into_inner();
    
    let updated_user = User {
        id: user_id,
        name: user_data.name.clone(),
        email: user_data.email.clone(),
    };
    
    Ok(HttpResponse::Ok().json(updated_user))
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
    path: web::Path<u64>
) -> ActixResult<HttpResponse> {
    let user_id = path.into_inner();
    
    Ok(HttpResponse::Ok().json(serde_json::json!({
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

#[cfg(feature = "swagger")]
#[actix_web::get("/swagger")]
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

#[cfg(feature = "swagger")]
#[actix_web::get("/swagger/openapi.json")]
async fn openapi_spec() -> ActixResult<HttpResponse> {
    let spec = ApiDoc::openapi().to_json().unwrap();
    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body(spec))
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
    println!("   configuration, but the actual routing is handled by Actix-web's native system.");
    
    // 注意：由于 uni_routing 宏生成的函数不能直接用于 Actix-web 的 service 方法，
    // 这里我们使用 Actix-web 的原生注解来注册路由，同时展示 uni_routing 宏的配置。
    // 在实际应用中，可以创建一个适配器来自动处理这种转换。
    
    HttpServer::new(|| {
        #[cfg(feature = "swagger")]
        let app = App::new()
            // 使用 Actix-web 原生路由注册，但展示了 uni_routing 宏的配置
            .service(
                web::resource("/api/health")
                    .route(web::get().to(health_check))
            )
            .service(
                web::resource("/api/users")
                    .route(web::get().to(get_users))
                    .route(web::post().to(create_user))
            )
            .service(
                web::resource("/api/users/{id}")
                    .route(web::get().to(get_user_by_id))
                    .route(web::put().to(update_user))
                    .route(web::delete().to(delete_user))
            )
            .service(swagger_ui)
            .service(openapi_spec);
        
        #[cfg(not(feature = "swagger"))]
        let app = App::new()
            // 使用 Actix-web 原生路由注册，但展示了 uni_routing 宏的配置
            .service(
                web::resource("/api/health")
                    .route(web::get().to(health_check))
            )
            .service(
                web::resource("/api/users")
                    .route(web::get().to(get_users))
                    .route(web::post().to(create_user))
            )
            .service(
                web::resource("/api/users/{id}")
                    .route(web::get().to(get_user_by_id))
                    .route(web::put().to(update_user))
                    .route(web::delete().to(delete_user))
            );
            
        app
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}