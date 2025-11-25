warning: creating a shared reference to mutable static
   --> src/registry.rs:124:9
    |
124 |         GLOBAL_REGISTRY.as_ref().unwrap()
    |         ^^^^^^^^^^^^^^^^^^^^^^^^ shared reference to mutable static
    |
    = note: for more information, see <https://doc.rust-lang.org/edition-guide/rust-2024/static-mut-references.html>
    = note: shared references to mutable statics are dangerous; it's undefined behavior if the static is mutated or if a mutable reference is created for it while the shared reference lives
    = note: `#[warn(static_mut_refs)]` (part of `#[warn(rust_2024_compatibility)]`) on by default
    Checking actix_server v0.1.0 (/root/projects/uni_routing/examples/actix_server)
error: invalid format string: expected `}`, found `\"`
   --> examples/actix_server/src/main.rs:291:103
    |
291 |     println!("  curl -X POST http://localhost:8080/api/users -H 'Content-Type: application/json' -d '{\"name\":\"Test User\",\"email\":\"...
    |                                                                                                      -^ expected `}` in format string
    |                                                                                                      |
    |                                                                                                      because of this opening brace
    |
    = note: if you intended to print `{`, you can escape it using `{{`
error: invalid format string: expected `}`, found `\"`
   --> examples/actix_server/src/main.rs:293:106
    |
293 | .../users/123 -H 'Content-Type: application/json' -d '{\"name\":\"Updated User\",\"email\":\"updated@example.com\"}'");
    |                                                       -^ expected `}` in format string
    |                                                       |
    |                                                       because of this opening brace
    |
    = note: if you intended to print `{`, you can escape it using `{{`
error[E0432]: unresolved import `uni_routing_macros`
 --> examples/actix_server/src/main.rs:5:5
  |
5 | use uni_routing_macros::uni_routing;
  |     ^^^^^^^^^^^^^^^^^^ use of unresolved module or unlinked crate `uni_routing_macros`
  |
  = help: if you wanted to use a crate named `uni_routing_macros`, use `cargo add uni_routing_macros` to add it to your `Cargo.toml`
error[E0425]: cannot find function `__register_route_health_check` in this scope
   --> examples/actix_server/src/main.rs:133:9
    |
133 |         __register_route_health_check(),
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ not found in this scope
error[E0425]: cannot find function `__register_route_get_users` in this scope
   --> examples/actix_server/src/main.rs:134:9
    |
134 |         __register_route_get_users(),
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^ not found in this scope
error[E0425]: cannot find function `__register_route_create_user` in this scope
   --> examples/actix_server/src/main.rs:135:9
    |
135 |         __register_route_create_user(),
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ not found in this scope
error[E0425]: cannot find function `__register_route_get_user_by_id` in this scope
   --> examples/actix_server/src/main.rs:136:9
    |
136 |         __register_route_get_user_by_id(),
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ not found in this scope
error[E0425]: cannot find function `__register_route_update_user` in this scope
   --> examples/actix_server/src/main.rs:137:9
    |
137 |         __register_route_update_user(),
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ not found in this scope
error[E0425]: cannot find function `__register_route_delete_user` in this scope
   --> examples/actix_server/src/main.rs:138:9
    |
138 |         __register_route_delete_user(),
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ not found in this scope
Some errors have detailed explanations: E0425, E0432.
For more information about an error, try `rustc --explain E0425`.

#![feature(prelude_import)]
//! Actix-web服务器示例 - 真正使用 uni_routing 宏进行路由注册
#[macro_use]
extern crate std;
#[prelude_import]
use std::prelude::rust_2021::*;
use actix_web::{web, App, HttpServer, HttpResponse, Result as ActixResult};
use serde::{Deserialize, Serialize};
use uni_routing_macros::uni_routing;
struct User {
    id: u64,
    name: String,
    email: String,
}
#[doc(hidden)]
#[allow(
    non_upper_case_globals,
    unused_attributes,
    unused_qualifications,
    clippy::absolute_paths,
)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl _serde::Serialize for User {
        fn serialize<__S>(
            &self,
            __serializer: __S,
        ) -> _serde::__private228::Result<__S::Ok, __S::Error>
        where
            __S: _serde::Serializer,
        {
            let mut __serde_state = _serde::Serializer::serialize_struct(
                __serializer,
                "User",
                false as usize + 1 + 1 + 1,
            )?;
            _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "id",
                &self.id,
            )?;
            _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "name",
                &self.name,
            )?;
            _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "email",
                &self.email,
            )?;
            _serde::ser::SerializeStruct::end(__serde_state)
        }
    }
};
#[doc(hidden)]
#[allow(
    non_upper_case_globals,
    unused_attributes,
    unused_qualifications,
    clippy::absolute_paths,
)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for User {
        fn deserialize<__D>(
            __deserializer: __D,
        ) -> _serde::__private228::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            #[doc(hidden)]
            enum __Field {
                __field0,
                __field1,
                __field2,
                __ignore,
            }
            #[doc(hidden)]
            struct __FieldVisitor;
            #[automatically_derived]
            impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                type Value = __Field;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private228::Formatter,
                ) -> _serde::__private228::fmt::Result {
                    _serde::__private228::Formatter::write_str(
                        __formatter,
                        "field identifier",
                    )
                }
                fn visit_u64<__E>(
                    self,
                    __value: u64,
                ) -> _serde::__private228::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        0u64 => _serde::__private228::Ok(__Field::__field0),
                        1u64 => _serde::__private228::Ok(__Field::__field1),
                        2u64 => _serde::__private228::Ok(__Field::__field2),
                        _ => _serde::__private228::Ok(__Field::__ignore),
                    }
                }
                fn visit_str<__E>(
                    self,
                    __value: &str,
                ) -> _serde::__private228::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        "id" => _serde::__private228::Ok(__Field::__field0),
                        "name" => _serde::__private228::Ok(__Field::__field1),
                        "email" => _serde::__private228::Ok(__Field::__field2),
                        _ => _serde::__private228::Ok(__Field::__ignore),
                    }
                }
                fn visit_bytes<__E>(
                    self,
                    __value: &[u8],
                ) -> _serde::__private228::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        b"id" => _serde::__private228::Ok(__Field::__field0),
                        b"name" => _serde::__private228::Ok(__Field::__field1),
                        b"email" => _serde::__private228::Ok(__Field::__field2),
                        _ => _serde::__private228::Ok(__Field::__ignore),
                    }
                }
            }
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for __Field {
                #[inline]
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private228::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    _serde::Deserializer::deserialize_identifier(
                        __deserializer,
                        __FieldVisitor,
                    )
                }
            }
            #[doc(hidden)]
            struct __Visitor<'de> {
                marker: _serde::__private228::PhantomData<User>,
                lifetime: _serde::__private228::PhantomData<&'de ()>,
            }
            #[automatically_derived]
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = User;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private228::Formatter,
                ) -> _serde::__private228::fmt::Result {
                    _serde::__private228::Formatter::write_str(
                        __formatter,
                        "struct User",
                    )
                }
                #[inline]
                fn visit_seq<__A>(
                    self,
                    mut __seq: __A,
                ) -> _serde::__private228::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::SeqAccess<'de>,
                {
                    let __field0 = match _serde::de::SeqAccess::next_element::<
                        u64,
                    >(&mut __seq)? {
                        _serde::__private228::Some(__value) => __value,
                        _serde::__private228::None => {
                            return _serde::__private228::Err(
                                _serde::de::Error::invalid_length(
                                    0usize,
                                    &"struct User with 3 elements",
                                ),
                            );
                        }
                    };
                    let __field1 = match _serde::de::SeqAccess::next_element::<
                        String,
                    >(&mut __seq)? {
                        _serde::__private228::Some(__value) => __value,
                        _serde::__private228::None => {
                            return _serde::__private228::Err(
                                _serde::de::Error::invalid_length(
                                    1usize,
                                    &"struct User with 3 elements",
                                ),
                            );
                        }
                    };
                    let __field2 = match _serde::de::SeqAccess::next_element::<
                        String,
                    >(&mut __seq)? {
                        _serde::__private228::Some(__value) => __value,
                        _serde::__private228::None => {
                            return _serde::__private228::Err(
                                _serde::de::Error::invalid_length(
                                    2usize,
                                    &"struct User with 3 elements",
                                ),
                            );
                        }
                    };
                    _serde::__private228::Ok(User {
                        id: __field0,
                        name: __field1,
                        email: __field2,
                    })
                }
                #[inline]
                fn visit_map<__A>(
                    self,
                    mut __map: __A,
                ) -> _serde::__private228::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::MapAccess<'de>,
                {
                    let mut __field0: _serde::__private228::Option<u64> = _serde::__private228::None;
                    let mut __field1: _serde::__private228::Option<String> = _serde::__private228::None;
                    let mut __field2: _serde::__private228::Option<String> = _serde::__private228::None;
                    while let _serde::__private228::Some(__key) = _serde::de::MapAccess::next_key::<
                        __Field,
                    >(&mut __map)? {
                        match __key {
                            __Field::__field0 => {
                                if _serde::__private228::Option::is_some(&__field0) {
                                    return _serde::__private228::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("id"),
                                    );
                                }
                                __field0 = _serde::__private228::Some(
                                    _serde::de::MapAccess::next_value::<u64>(&mut __map)?,
                                );
                            }
                            __Field::__field1 => {
                                if _serde::__private228::Option::is_some(&__field1) {
                                    return _serde::__private228::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("name"),
                                    );
                                }
                                __field1 = _serde::__private228::Some(
                                    _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                );
                            }
                            __Field::__field2 => {
                                if _serde::__private228::Option::is_some(&__field2) {
                                    return _serde::__private228::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("email"),
                                    );
                                }
                                __field2 = _serde::__private228::Some(
                                    _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                );
                            }
                            _ => {
                                let _ = _serde::de::MapAccess::next_value::<
                                    _serde::de::IgnoredAny,
                                >(&mut __map)?;
                            }
                        }
                    }
                    let __field0 = match __field0 {
                        _serde::__private228::Some(__field0) => __field0,
                        _serde::__private228::None => {
                            _serde::__private228::de::missing_field("id")?
                        }
                    };
                    let __field1 = match __field1 {
                        _serde::__private228::Some(__field1) => __field1,
                        _serde::__private228::None => {
                            _serde::__private228::de::missing_field("name")?
                        }
                    };
                    let __field2 = match __field2 {
                        _serde::__private228::Some(__field2) => __field2,
                        _serde::__private228::None => {
                            _serde::__private228::de::missing_field("email")?
                        }
                    };
                    _serde::__private228::Ok(User {
                        id: __field0,
                        name: __field1,
                        email: __field2,
                    })
                }
            }
            #[doc(hidden)]
            const FIELDS: &'static [&'static str] = &["id", "name", "email"];
            _serde::Deserializer::deserialize_struct(
                __deserializer,
                "User",
                FIELDS,
                __Visitor {
                    marker: _serde::__private228::PhantomData::<User>,
                    lifetime: _serde::__private228::PhantomData,
                },
            )
        }
    }
};
struct CreateUserRequest {
    name: String,
    email: String,
}
#[doc(hidden)]
#[allow(
    non_upper_case_globals,
    unused_attributes,
    unused_qualifications,
    clippy::absolute_paths,
)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for CreateUserRequest {
        fn deserialize<__D>(
            __deserializer: __D,
        ) -> _serde::__private228::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            #[doc(hidden)]
            enum __Field {
                __field0,
                __field1,
                __ignore,
            }
            #[doc(hidden)]
            struct __FieldVisitor;
            #[automatically_derived]
            impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                type Value = __Field;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private228::Formatter,
                ) -> _serde::__private228::fmt::Result {
                    _serde::__private228::Formatter::write_str(
                        __formatter,
                        "field identifier",
                    )
                }
                fn visit_u64<__E>(
                    self,
                    __value: u64,
                ) -> _serde::__private228::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        0u64 => _serde::__private228::Ok(__Field::__field0),
                        1u64 => _serde::__private228::Ok(__Field::__field1),
                        _ => _serde::__private228::Ok(__Field::__ignore),
                    }
                }
                fn visit_str<__E>(
                    self,
                    __value: &str,
                ) -> _serde::__private228::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        "name" => _serde::__private228::Ok(__Field::__field0),
                        "email" => _serde::__private228::Ok(__Field::__field1),
                        _ => _serde::__private228::Ok(__Field::__ignore),
                    }
                }
                fn visit_bytes<__E>(
                    self,
                    __value: &[u8],
                ) -> _serde::__private228::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        b"name" => _serde::__private228::Ok(__Field::__field0),
                        b"email" => _serde::__private228::Ok(__Field::__field1),
                        _ => _serde::__private228::Ok(__Field::__ignore),
                    }
                }
            }
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for __Field {
                #[inline]
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private228::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    _serde::Deserializer::deserialize_identifier(
                        __deserializer,
                        __FieldVisitor,
                    )
                }
            }
            #[doc(hidden)]
            struct __Visitor<'de> {
                marker: _serde::__private228::PhantomData<CreateUserRequest>,
                lifetime: _serde::__private228::PhantomData<&'de ()>,
            }
            #[automatically_derived]
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = CreateUserRequest;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private228::Formatter,
                ) -> _serde::__private228::fmt::Result {
                    _serde::__private228::Formatter::write_str(
                        __formatter,
                        "struct CreateUserRequest",
                    )
                }
                #[inline]
                fn visit_seq<__A>(
                    self,
                    mut __seq: __A,
                ) -> _serde::__private228::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::SeqAccess<'de>,
                {
                    let __field0 = match _serde::de::SeqAccess::next_element::<
                        String,
                    >(&mut __seq)? {
                        _serde::__private228::Some(__value) => __value,
                        _serde::__private228::None => {
                            return _serde::__private228::Err(
                                _serde::de::Error::invalid_length(
                                    0usize,
                                    &"struct CreateUserRequest with 2 elements",
                                ),
                            );
                        }
                    };
                    let __field1 = match _serde::de::SeqAccess::next_element::<
                        String,
                    >(&mut __seq)? {
                        _serde::__private228::Some(__value) => __value,
                        _serde::__private228::None => {
                            return _serde::__private228::Err(
                                _serde::de::Error::invalid_length(
                                    1usize,
                                    &"struct CreateUserRequest with 2 elements",
                                ),
                            );
                        }
                    };
                    _serde::__private228::Ok(CreateUserRequest {
                        name: __field0,
                        email: __field1,
                    })
                }
                #[inline]
                fn visit_map<__A>(
                    self,
                    mut __map: __A,
                ) -> _serde::__private228::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::MapAccess<'de>,
                {
                    let mut __field0: _serde::__private228::Option<String> = _serde::__private228::None;
                    let mut __field1: _serde::__private228::Option<String> = _serde::__private228::None;
                    while let _serde::__private228::Some(__key) = _serde::de::MapAccess::next_key::<
                        __Field,
                    >(&mut __map)? {
                        match __key {
                            __Field::__field0 => {
                                if _serde::__private228::Option::is_some(&__field0) {
                                    return _serde::__private228::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("name"),
                                    );
                                }
                                __field0 = _serde::__private228::Some(
                                    _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                );
                            }
                            __Field::__field1 => {
                                if _serde::__private228::Option::is_some(&__field1) {
                                    return _serde::__private228::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("email"),
                                    );
                                }
                                __field1 = _serde::__private228::Some(
                                    _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                );
                            }
                            _ => {
                                let _ = _serde::de::MapAccess::next_value::<
                                    _serde::de::IgnoredAny,
                                >(&mut __map)?;
                            }
                        }
                    }
                    let __field0 = match __field0 {
                        _serde::__private228::Some(__field0) => __field0,
                        _serde::__private228::None => {
                            _serde::__private228::de::missing_field("name")?
                        }
                    };
                    let __field1 = match __field1 {
                        _serde::__private228::Some(__field1) => __field1,
                        _serde::__private228::None => {
                            _serde::__private228::de::missing_field("email")?
                        }
                    };
                    _serde::__private228::Ok(CreateUserRequest {
                        name: __field0,
                        email: __field1,
                    })
                }
            }
            #[doc(hidden)]
            const FIELDS: &'static [&'static str] = &["name", "email"];
            _serde::Deserializer::deserialize_struct(
                __deserializer,
                "CreateUserRequest",
                FIELDS,
                __Visitor {
                    marker: _serde::__private228::PhantomData::<CreateUserRequest>,
                    lifetime: _serde::__private228::PhantomData,
                },
            )
        }
    }
};
#[uni_routing(
    route = "/api/health",
    method = "GET",
    description = "健康检查端点，检查服务器运行状态"
)]
async fn health_check() -> ActixResult<HttpResponse> {
    Ok(
        HttpResponse::Ok()
            .json(
                ::serde_json::Value::Object({
                    let mut object = ::serde_json::Map::new();
                    let _ = object
                        .insert(
                            ("status").into(),
                            ::serde_json::to_value(&"healthy").unwrap(),
                        );
                    let _ = object
                        .insert(
                            ("timestamp").into(),
                            ::serde_json::to_value(&chrono::Utc::now().to_rfc3339())
                                .unwrap(),
                        );
                    object
                }),
            ),
    )
}
#[uni_routing(
    route = "/api/users",
    method = "GET",
    auth_policy = "role:admin,permission:users.read",
    description = "获取所有用户列表，需要管理员权限和用户读取权限"
)]
async fn get_users() -> ActixResult<HttpResponse> {
    let users = <[_]>::into_vec(
        ::alloc::boxed::box_new([
            User {
                id: 1,
                name: "Alice".to_string(),
                email: "alice@example.com".to_string(),
            },
            User {
                id: 2,
                name: "Bob".to_string(),
                email: "bob@example.com".to_string(),
            },
            User {
                id: 3,
                name: "Charlie".to_string(),
                email: "charlie@example.com".to_string(),
            },
        ]),
    );
    Ok(HttpResponse::Ok().json(users))
}
#[uni_routing(
    route = "/api/users",
    method = "POST",
    auth_policy = "role:admin,permission:users.write",
    description = "创建新用户，需要管理员权限和用户写入权限"
)]
async fn create_user(
    user_data: web::Json<CreateUserRequest>,
) -> ActixResult<HttpResponse> {
    let new_user = User {
        id: 999,
        name: user_data.name.clone(),
        email: user_data.email.clone(),
    };
    Ok(HttpResponse::Created().json(new_user))
}
#[uni_routing(
    route = "/api/users/{id}",
    method = "GET",
    auth_policy = "role:admin,permission:users.read",
    description = "根据ID获取特定用户信息"
)]
async fn get_user_by_id(path: web::Path<u64>) -> ActixResult<HttpResponse> {
    let user_id = path.into_inner();
    let user = User {
        id: user_id,
        name: ::alloc::__export::must_use({
            ::alloc::fmt::format(format_args!("User {0}", user_id))
        }),
        email: ::alloc::__export::must_use({
            ::alloc::fmt::format(format_args!("user{0}@example.com", user_id))
        }),
    };
    Ok(HttpResponse::Ok().json(user))
}
#[uni_routing(
    route = "/api/users/{id}",
    method = "PUT",
    auth_policy = "role:admin,permission:users.write",
    description = "更新指定ID的用户信息"
)]
async fn update_user(
    path: web::Path<u64>,
    user_data: web::Json<CreateUserRequest>,
) -> ActixResult<HttpResponse> {
    let user_id = path.into_inner();
    let updated_user = User {
        id: user_id,
        name: user_data.name.clone(),
        email: user_data.email.clone(),
    };
    Ok(HttpResponse::Ok().json(updated_user))
}
#[uni_routing(
    route = "/api/users/{id}",
    method = "DELETE",
    auth_policy = "role:admin,permission:users.delete",
    description = "删除指定ID的用户"
)]
async fn delete_user(path: web::Path<u64>) -> ActixResult<HttpResponse> {
    let user_id = path.into_inner();
    Ok(
        HttpResponse::Ok()
            .json(
                ::serde_json::Value::Object({
                    let mut object = ::serde_json::Map::new();
                    let _ = object
                        .insert(
                            ("message").into(),
                            ::serde_json::to_value(
                                    &::alloc::__export::must_use({
                                        ::alloc::fmt::format(
                                            format_args!("User {0} deleted successfully", user_id),
                                        )
                                    }),
                                )
                                .unwrap(),
                        );
                    object
                }),
            ),
    )
}
fn collect_routes() -> Vec<uni_routing::routing::RouteInfo> {
    <[_]>::into_vec(
        ::alloc::boxed::box_new([
            __register_route_health_check(),
            __register_route_get_users(),
            __register_route_create_user(),
            __register_route_get_user_by_id(),
            __register_route_update_user(),
            __register_route_delete_user(),
        ]),
    )
}
fn generate_openapi() -> serde_json::Value {
    let routes = collect_routes();
    let mut paths = serde_json::Map::new();
    for route in routes {
        let path_item = create_path_item(&route);
        paths.insert(route.path.clone(), serde_json::Value::Object(path_item));
    }
    ::serde_json::Value::Object({
        let mut object = ::serde_json::Map::new();
        let _ = object
            .insert(("openapi").into(), ::serde_json::to_value(&"3.0.0").unwrap());
        let _ = object
            .insert(
                ("info").into(),
                ::serde_json::Value::Object({
                    let mut object = ::serde_json::Map::new();
                    let _ = object
                        .insert(
                            ("title").into(),
                            ::serde_json::to_value(&"Uni Routing API").unwrap(),
                        );
                    let _ = object
                        .insert(
                            ("version").into(),
                            ::serde_json::to_value(&"1.0.0").unwrap(),
                        );
                    let _ = object
                        .insert(
                            ("description").into(),
                            ::serde_json::to_value(
                                    &"API documentation automatically generated from uni_routing macros",
                                )
                                .unwrap(),
                        );
                    object
                }),
            );
        let _ = object
            .insert(
                ("paths").into(),
                ::serde_json::to_value(&serde_json::Value::Object(paths)).unwrap(),
            );
        let _ = object
            .insert(
                ("components").into(),
                ::serde_json::Value::Object({
                    let mut object = ::serde_json::Map::new();
                    let _ = object
                        .insert(
                            ("securitySchemes").into(),
                            ::serde_json::Value::Object({
                                let mut object = ::serde_json::Map::new();
                                let _ = object
                                    .insert(
                                        ("bearerAuth").into(),
                                        ::serde_json::Value::Object({
                                            let mut object = ::serde_json::Map::new();
                                            let _ = object
                                                .insert(
                                                    ("type").into(),
                                                    ::serde_json::to_value(&"http").unwrap(),
                                                );
                                            let _ = object
                                                .insert(
                                                    ("scheme").into(),
                                                    ::serde_json::to_value(&"bearer").unwrap(),
                                                );
                                            let _ = object
                                                .insert(
                                                    ("bearerFormat").into(),
                                                    ::serde_json::to_value(&"JWT").unwrap(),
                                                );
                                            object
                                        }),
                                    );
                                object
                            }),
                        );
                    object
                }),
            );
        object
    })
}
fn create_path_item(
    route: &uni_routing::routing::RouteInfo,
) -> serde_json::Map<String, serde_json::Value> {
    let mut path_item = serde_json::Map::new();
    let method_str = match route.method {
        uni_routing::routing::HttpMethod::GET => "get",
        uni_routing::routing::HttpMethod::POST => "post",
        uni_routing::routing::HttpMethod::PUT => "put",
        uni_routing::routing::HttpMethod::DELETE => "delete",
        uni_routing::routing::HttpMethod::PATCH => "patch",
    };
    let mut operation = serde_json::Map::new();
    operation
        .insert(
            "summary".to_string(),
            serde_json::Value::String(
                route.description.clone().unwrap_or_else(|| "API endpoint".to_string()),
            ),
        );
    if route.description.is_some() {
        operation
            .insert(
                "description".to_string(),
                serde_json::Value::String(route.description.clone().unwrap()),
            );
    }
    if route.auth_policy.is_some() {
        operation
            .insert(
                "security".to_string(),
                ::serde_json::Value::Array(
                    <[_]>::into_vec(
                        ::alloc::boxed::box_new([
                            ::serde_json::Value::Object({
                                let mut object = ::serde_json::Map::new();
                                let _ = object
                                    .insert(
                                        ("bearerAuth").into(),
                                        ::serde_json::Value::Array(::alloc::vec::Vec::new()),
                                    );
                                object
                            }),
                        ]),
                    ),
                ),
            );
    }
    let mut responses = serde_json::Map::new();
    responses
        .insert(
            "200".to_string(),
            ::serde_json::Value::Object({
                let mut object = ::serde_json::Map::new();
                let _ = object
                    .insert(
                        ("description").into(),
                        ::serde_json::to_value(&"Successful response").unwrap(),
                    );
                object
            }),
        );
    operation.insert("responses".to_string(), serde_json::Value::Object(responses));
    path_item.insert(method_str.to_string(), serde_json::Value::Object(operation));
    path_item
}
async fn swagger_ui() -> ActixResult<HttpResponse> {
    let html = r#"
<!DOCTYPE html>
<html>
<head>
    <title>Uni Routing API Documentation</title>
    <link rel="stylesheet" type="text/css" href="https://unpkg.com/swagger-ui-dist@4/swagger-ui.css" />
    <style>
        html { box-sizing: border-box; overflow: -moz-scrollbars-vertical; overflow-y: scroll; }
        *, *:before, *:after { box-sizing: inherit; }
        body { margin:0; background: #fafafa; }
    </style>
</head>
<body>
    <div id="swagger-ui"></div>
    <script src="https://unpkg.com/swagger-ui-dist@4/swagger-ui-bundle.js"></script>
    <script src="https://unpkg.com/swagger-ui-dist@4/swagger-ui-standalone-preset.js"></script>
    <script>
        window.onload = function() {
            const ui = SwaggerUIBundle({
                url: '/swagger/openapi.json',
                dom_id: '#swagger-ui',
                deepLinking: true,
                presets: [
                    SwaggerUIBundle.presets.apis,
                    SwaggerUIStandalonePreset
                ],
                plugins: [
                    SwaggerUIBundle.plugins.DownloadUrl
                ],
                layout: "StandaloneLayout"
            });
        };
    </script>
</body>
</html>
    "#;
    Ok(HttpResponse::Ok().content_type("text/html").body(html))
}
async fn openapi_spec() -> ActixResult<HttpResponse> {
    let spec = generate_openapi();
    Ok(HttpResponse::Ok().json(spec))
}
fn main() -> std::io::Result<()> {
    <::actix_web::rt::System>::new()
        .block_on(async move {
            {
                env_logger::init();
                {
                    ::std::io::_print(
                        format_args!(
                            "🚀 Starting Actix-web server with uni_routing...\n",
                        ),
                    );
                };
                {
                    ::std::io::_print(format_args!("\n"));
                };
                let routes = collect_routes();
                {
                    ::std::io::_print(
                        format_args!(
                            "📋 Registered routes (from uni_routing macro):\n",
                        ),
                    );
                };
                for route in &routes {
                    let auth_info = if route.auth_policy.is_some() {
                        "🔒 (requires auth)"
                    } else {
                        "🔓 (public)"
                    };
                    {
                        ::std::io::_print(
                            format_args!(
                                "  {0} {1} {2} - {3}\n",
                                route.method,
                                route.path,
                                auth_info,
                                route.description.as_deref().unwrap_or("No description"),
                            ),
                        );
                    };
                }
                {
                    ::std::io::_print(format_args!("\n"));
                };
                {
                    ::std::io::_print(
                        format_args!("🌐 Server starting on http://localhost:8080\n"),
                    );
                };
                {
                    ::std::io::_print(
                        format_args!("📖 Swagger UI: http://localhost:8080/swagger\n"),
                    );
                };
                {
                    ::std::io::_print(
                        format_args!(
                            "📄 OpenAPI Spec: http://localhost:8080/swagger/openapi.json\n",
                        ),
                    );
                };
                {
                    ::std::io::_print(format_args!("\n"));
                };
                {
                    ::std::io::_print(format_args!("🧪 Try these commands:\n"));
                };
                {
                    ::std::io::_print(
                        format_args!("  curl -X GET http://localhost:8080/api/health\n"),
                    );
                };
                {
                    ::std::io::_print(
                        format_args!("  curl -X GET http://localhost:8080/api/users\n"),
                    );
                };
                {
                    ::std::io::_print((/*ERROR*/));
                };
                {
                    ::std::io::_print(
                        format_args!(
                            "  curl -X GET http://localhost:8080/api/users/123\n",
                        ),
                    );
                };
                {
                    ::std::io::_print((/*ERROR*/));
                };
                {
                    ::std::io::_print(
                        format_args!(
                            "  curl -X DELETE http://localhost:8080/api/users/123\n",
                        ),
                    );
                };
                {
                    ::std::io::_print(format_args!("\n"));
                };
                {
                    ::std::io::_print(
                        format_args!(
                            "📝 Note: All routes are defined using #[uni_routing] macro and automatically registered!\n",
                        ),
                    );
                };
                HttpServer::new(|| {
                        App::new()
                            .service(
                                web::resource("/api/health")
                                    .route(web::get().to(health_check)),
                            )
                            .service(
                                web::resource("/api/users")
                                    .route(web::get().to(get_users))
                                    .route(web::post().to(create_user)),
                            )
                            .service(
                                web::resource("/api/users/{id}")
                                    .route(web::get().to(get_user_by_id))
                                    .route(web::put().to(update_user))
                                    .route(web::delete().to(delete_user)),
                            )
                            .service(
                                web::resource("/swagger").route(web::get().to(swagger_ui)),
                            )
                            .service(
                                web::resource("/swagger/openapi.json")
                                    .route(web::get().to(openapi_spec)),
                            )
                    })
                    .bind("127.0.0.1:8080")?
                    .run()
                    .await
            }
        })
}
