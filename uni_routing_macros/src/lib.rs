//! 过程宏定义，实现uni_routing属性宏

use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;
use syn::ItemFn;

/// uni_routing属性宏，用于简化路由配置
///
/// # 示例
///
/// ```rust
/// #[uni_routing(
///     route = "/api/users",
///     method = "GET",
///     auth_policy = "roles:admins,users",
///     description = "获取所有用户"
/// )]
/// async fn get_users() -> Result<Json<Vec<User>>, Error> {
///     // 函数实现
/// }
/// ```
#[proc_macro_attribute]
pub fn uni_routing(args: TokenStream, input: TokenStream) -> TokenStream {
    let input_fn = parse_macro_input!(input as ItemFn);
    
    let fn_name = &input_fn.sig.ident;
    let fn_vis = &input_fn.vis;
    let fn_sig = &input_fn.sig;
    let fn_block = &input_fn.block;
    
    // 解析属性参数
    let args_str = args.to_string();
    let mut route = None;
    let mut method = None;
    let mut auth_policy = None;
    let mut description = None;
    
    // 简单的字符串解析
    for pair in args_str.split(',') {
        let pair = pair.trim();
        if let Some((key, value)) = pair.split_once('=') {
            let key = key.trim();
            let value = value.trim().trim_matches('"');
            
            match key {
                "route" => route = Some(value.to_string()),
                "method" => method = Some(value.to_string()),
                "auth_policy" => auth_policy = Some(value.to_string()),
                "description" => description = Some(value.to_string()),
                _ => {}
            }
        }
    }
    
    // 生成路由信息常量
    let _route_info_name = syn::Ident::new(&format!("__ROUTE_INFO_{}", fn_name), fn_name.span());
    let route_lit = route.unwrap_or_else(|| "/".to_string());
    let method_lit = method.unwrap_or_else(|| "GET".to_string());
    let auth_policy_lit = auth_policy.unwrap_or_else(|| String::new());
    let description_lit = description.unwrap_or_else(|| String::new());
    
    // 将字符串转换为HttpMethod
    let method_enum = match method_lit.to_uppercase().as_str() {
        "GET" => quote!(uni_routing::routing::HttpMethod::GET),
        "POST" => quote!(uni_routing::routing::HttpMethod::POST),
        "PUT" => quote!(uni_routing::routing::HttpMethod::PUT),
        "DELETE" => quote!(uni_routing::routing::HttpMethod::DELETE),
        "PATCH" => quote!(uni_routing::routing::HttpMethod::PATCH),
        _ => quote!(uni_routing::routing::HttpMethod::GET),
    };

    // 生成路由注册函数
    let register_fn_name = syn::Ident::new(&format!("__register_route_{}", fn_name), fn_name.span());
    
    let expanded = quote! {
        // 原始函数
        #fn_vis #fn_sig #fn_block
        
        // 路由注册函数
        #[allow(dead_code)]
        fn #register_fn_name() -> uni_routing::routing::RouteInfo {
            uni_routing::routing::RouteInfo {
                path: #route_lit.to_string(),
                method: #method_enum,
                auth_policy: if #auth_policy_lit.is_empty() { 
                    None 
                } else { 
                    Some(uni_routing::auth::AuthPolicy::new(#auth_policy_lit)) 
                },
                description: if #description_lit.is_empty() { 
                    None 
                } else { 
                    Some(#description_lit.to_string()) 
                },
            }
        }
    };
    
    TokenStream::from(expanded)
}

/// 用于生成路由收集器的宏
#[proc_macro]
pub fn collect_routes(_input: TokenStream) -> TokenStream {
    let expanded = quote! {
        /// 收集所有使用uni_routing宏定义的路由
        pub fn collect_all_routes() -> Vec<uni_routing::routing::RouteInfo> {
            // 在实际实现中，这里会从link_section中读取所有路由信息
            // 目前返回空向量作为占位符
            vec![]
        }
    };
    
    TokenStream::from(expanded)
}