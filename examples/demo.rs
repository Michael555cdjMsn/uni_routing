//! uni_routing 宏使用演示

#![allow(dead_code)]

use uni_routing_macros::uni_routing;
use uni_routing::routing::RouteInfo;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[allow(dead_code)]
struct User {
    id: u64,
    name: String,
    email: String,
}

// 使用 uni_routing 宏定义 API 端点
#[uni_routing(
    route = "/api/users",
    method = "GET",
    auth_policy = "role:admin,permission:read",
    description = "获取所有用户"
)]
#[allow(dead_code)]
async fn get_all_users() -> Result<Vec<User>, Box<dyn std::error::Error>> {
    let users = vec![
        User { 
            id: 1, 
            name: "Alice".to_string(), 
            email: "alice@example.com".to_string() 
        },
        User { 
            id: 2, 
            name: "Bob".to_string(), 
            email: "bob@example.com".to_string() 
        },
    ];
    
    Ok(users)
}

#[uni_routing(
    route = "/api/users",
    method = "POST",
    auth_policy = "role:admin,permission:write",
    description = "创建新用户"
)]
async fn create_user(user_data: User) -> Result<User, Box<dyn std::error::Error>> {
    // 模拟创建用户
    Ok(User {
        id: 999,
        name: user_data.name,
        email: user_data.email,
    })
}

#[uni_routing(
    route = "/api/health",
    method = "GET",
    description = "健康检查"
)]
#[allow(dead_code)]
async fn health_check() -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    Ok(serde_json::json!({
        "status": "healthy",
        "timestamp": chrono::Utc::now().to_rfc3339()
    }))
}

fn main() {
    println!("🚀 Uni Routing Framework 演示");
    println!();
    
    // 演示路由信息获取
    let route1 = __register_route_get_all_users();
    let route2 = __register_route_create_user();
    let route3 = __register_route_health_check();
    
    println!("📋 已注册的路由:");
    print_route_info(&route1);
    print_route_info(&route2);
    print_route_info(&route3);
    
    println!();
    println!("✅ 演示完成！uni_routing 宏成功生成了路由信息。");
}

fn print_route_info(route: &RouteInfo) {
    println!("  📍 {} {}", route.method, route.path);
    if let Some(desc) = &route.description {
        println!("     📝 {}", desc);
    }
    if let Some(policy) = &route.auth_policy {
        println!("     🔐 认证策略: {}", policy.policy_expr);
    }
    println!();
}