#![warn(missing_docs)]

//! iTools CLI 核心宏模块
//!
//! 提供类似 clap derive 的宏 API，支持位置参数、选项参数和子命令，
//! 以及基于 i18n 的错误处理宏。

use proc_macro::TokenStream;
use quote::quote;
use syn::*;

/// 命令行参数解析宏
///
/// 用于替代 clap::Parser，提供类型安全的参数解析。
///
/// # 示例
/// ```rust
/// #[itools_macros::Command]
/// pub struct Cli {
///     /// 命令
///     pub command: Command,
/// }
///
/// #[itools_macros::Command]
/// pub enum Command {
///     /// 初始化新项目
///     Init {
///         /// 项目名称
///         name: String,
///     },
///     /// 构建项目
///     Build,
/// }
/// ```
#[proc_macro_attribute]
pub fn command(_args: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    match input.data {
        Data::Struct(ref data) => handle_struct(&input, data),
        Data::Enum(ref data) => handle_enum(&input, data),
        _ => {
            let error: proc_macro2::TokenStream = parse_quote! {
                compile_error!("#[Command] can only be used on structs and enums");
            };
            error.into()
        }
    }
}

/// 错误处理宏
///
/// 用于替代 thiserror，提供基于 i18n 的错误处理。
///
/// # 示例
/// ```rust
/// #[itools_macros::Error]
/// pub enum AppError {
///     /// 网络错误
///     #[error("network_error")]
///     NetworkError,
///     /// 文件未找到
///     #[error("file_not_found")]
///     FileNotFound { path: String },
/// }
/// ```
#[proc_macro_attribute]
pub fn error(_args: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    match input.data {
        Data::Struct(ref data) => handle_error_struct(&input, data),
        Data::Enum(ref data) => handle_error_enum(&input, data),
        _ => {
            let error: proc_macro2::TokenStream = parse_quote! {
                compile_error!("#[Error] can only be used on structs and enums");
            };
            error.into()
        }
    }
}

/// 处理结构体
fn handle_struct(input: &DeriveInput, data: &DataStruct) -> TokenStream {
    let name = &input.ident;
    let generics = &input.generics;
    let where_clause = &generics.where_clause;

    let expanded = match &data.fields {
        Fields::Named(fields) => {
            let field_parsers = fields.named.iter().map(|field| {
                let ident = field.ident.as_ref().unwrap();
                let ty = &field.ty;
                quote! {
                    #ident: <#ty as itools::Parse>::parse(args),
                }
            });
            quote! {
                #input

                impl itools::Parse for #name #generics #where_clause {
                    fn parse(args: &mut Vec<String>) -> Self {
                        Self {
                            #(#field_parsers)*
                        }
                    }
                }

                impl #name #generics #where_clause {
                    /// 创建构建器
                    pub fn builder() -> itools::Builder {
                        itools::Builder::new()
                    }
                }
            }
        }
        Fields::Unnamed(fields) => {
            let field_parsers = fields.unnamed.iter().map(|field| {
                let ty = &field.ty;
                quote! {
                    <#ty as itools::Parse>::parse(args),
                }
            });
            quote! {
                #input

                impl itools::Parse for #name #generics #where_clause {
                    fn parse(args: &mut Vec<String>) -> Self {
                        Self(
                            #(#field_parsers)*
                        )
                    }
                }

                impl #name #generics #where_clause {
                    /// 创建构建器
                    pub fn builder() -> itools::Builder {
                        itools::Builder::new()
                    }
                }
            }
        }
        Fields::Unit => quote! {
            #input

            impl itools::Parse for #name #generics #where_clause {
                fn parse(args: &mut Vec<String>) -> Self {
                    Self
                }
            }

            impl #name #generics #where_clause {
                /// 创建构建器
                pub fn builder() -> itools::Builder {
                    itools::Builder::new()
                }
            }
        },
    };

    expanded.into()
}

/// 处理枚举
fn handle_enum(input: &DeriveInput, data: &DataEnum) -> TokenStream {
    let name = &input.ident;
    let generics = &input.generics;
    let where_clause = &generics.where_clause;

    let variants = data.variants.iter().map(|variant| {
        let variant_ident = &variant.ident;
        // 将驼峰命名转换为连字符分隔的命令名称
        let variant_str = {
            let s = variant_ident.to_string();
            let mut result = String::new();
            for (i, c) in s.chars().enumerate() {
                if i > 0 && c.is_uppercase() {
                    result.push('-');
                }
                result.push(c.to_lowercase().next().unwrap());
            }
            result
        };

        match &variant.fields {
            Fields::Named(fields) => {
                let field_parsers = fields.named.iter().map(|field| {
                    let ident = field.ident.as_ref().unwrap();
                    let ty = &field.ty;
                    quote! {
                        #ident: <#ty as itools::Parse>::parse(args),
                    }
                });
                quote! {
                    #variant_str => {
                        Self::#variant_ident {
                            #(#field_parsers)*
                        }
                    }
                }
            }
            Fields::Unnamed(fields) => {
                let field_parsers = fields.unnamed.iter().map(|field| {
                    let ty = &field.ty;
                    quote! {
                        <#ty as itools::Parse>::parse(args),
                    }
                });
                quote! {
                    #variant_str => {
                        Self::#variant_ident(
                            #(#field_parsers)*
                        )
                    }
                }
            }
            Fields::Unit => {
                quote! {
                    #variant_str => Self::#variant_ident,
                }
            }
        }
    });

    let expanded = quote! {
        #input

        impl itools::Parse for #name #generics #where_clause {
            fn parse(args: &mut Vec<String>) -> Self {
                if args.is_empty() {
                    panic!("Expected command");
                }
                let cmd = args.remove(0);
                match cmd.as_str() {
                    #(#variants)*
                    _ => panic!("Unknown command: {}", cmd),
                }
            }
        }

        impl #name #generics #where_clause {
            /// 创建构建器
            pub fn builder() -> itools::Builder {
                itools::Builder::new()
            }
        }
    };

    expanded.into()
}

/// 处理错误结构体
fn handle_error_struct(input: &DeriveInput, _data: &DataStruct) -> TokenStream {
    let name = &input.ident;
    let generics = &input.generics;
    let where_clause = &generics.where_clause;

    let error_message = get_error_message(&input.attrs);

    let expanded = quote! {
        #input

        impl std::fmt::Debug for #name #generics #where_clause {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                std::fmt::Debug::fmt(self, f)
            }
        }

        impl std::fmt::Display for #name #generics #where_clause {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.localize())
            }
        }

        impl std::error::Error for #name #generics #where_clause {}

        impl itools_core::LocalizableError for #name #generics #where_clause {
            fn localize(&self) -> String {
                itools::t(#error_message)
            }
        }
    };

    expanded.into()
}

/// 处理错误枚举
fn handle_error_enum(input: &DeriveInput, data: &DataEnum) -> TokenStream {
    let name = &input.ident;
    let generics = &input.generics;
    let where_clause = &generics.where_clause;

    let variants = data.variants.iter().map(|variant| {
        let variant_ident = &variant.ident;
        let error_message = get_error_message(&variant.attrs);

        match &variant.fields {
            Fields::Named(fields) => {
                let field_names = fields.named.iter().map(|field| {
                    let ident = field.ident.as_ref().unwrap();
                    quote! { #ident }
                });
                quote! {
                    Self::#variant_ident { #(#field_names),* } => itools::t(#error_message)
                }
            }
            Fields::Unnamed(fields) => {
                let field_names = (0..fields.unnamed.len()).map(|i| {
                    let i = syn::Index::from(i);
                    quote! { self.#i }
                });
                quote! {
                    Self::#variant_ident(#(#field_names),*) => itools::t(#error_message)
                }
            }
            Fields::Unit => {
                quote! {
                    Self::#variant_ident => itools::t(#error_message)
                }
            }
        }
    });

    let expanded = quote! {
        #input

        impl std::fmt::Debug for #name #generics #where_clause {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                std::fmt::Debug::fmt(self, f)
            }
        }

        impl std::fmt::Display for #name #generics #where_clause {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.localize())
            }
        }

        impl std::error::Error for #name #generics #where_clause {}

        impl itools_core::LocalizableError for #name #generics #where_clause {
            fn localize(&self) -> String {
                match self {
                    #(#variants)*
                }
            }
        }
    };

    expanded.into()
}

/// 获取错误消息
fn get_error_message(attrs: &[Attribute]) -> proc_macro2::TokenStream {
    for attr in attrs {
        if attr.path().is_ident("error") {
            // 简单处理：直接提取字符串字面量
            if let Ok(lit_str) = attr.parse_args::<syn::LitStr>() {
                return quote! { #lit_str };
            }
        }
    }
    quote! { "unknown_error" }
}

/// 翻译宏
///
/// 用于获取翻译，支持重载：
/// - `t!("key")` - 基本翻译
/// - `t!("key", &[("name", "value")])` - 带参数的翻译
///
/// # 示例
/// ```rust
/// // 基本翻译
/// let greeting = t!("hello");
///
/// // 带参数的翻译
/// let greeting = t!("greeting", &[("name", "World")]);
/// ```
#[proc_macro]
pub fn t(input: TokenStream) -> TokenStream {
    // 尝试解析为单个字符串字面量（基本翻译）
    if let Ok(lit_str) = syn::parse::<syn::LitStr>(input.clone()) {
        let key = lit_str.value();

        // 简单的编译时检查：确保键不为空
        if key.is_empty() {
            let error: proc_macro2::TokenStream = parse_quote! {
                compile_error!("t! macro expects a non-empty string literal as translation key");
            };
            return error.into();
        }

        let expanded = quote! {
            itools_localization::t(#lit_str)
        };

        return expanded.into();
    }

    // 尝试解析为元组（带参数的翻译）
    if let Ok(expr_tuple) = syn::parse::<syn::ExprTuple>(input) {
        if expr_tuple.elems.len() != 2 {
            let error: proc_macro2::TokenStream = parse_quote! {
                compile_error!("t! macro expects either one argument (key) or two arguments (key, args)");
            };
            return error.into();
        }

        let key = &expr_tuple.elems[0];
        let args = &expr_tuple.elems[1];

        let expanded = quote! {
            itools_localization::t_with_args(#key, #args)
        };

        return expanded.into();
    }

    // 解析失败
    let error: proc_macro2::TokenStream = parse_quote! {
        compile_error!("t! macro expects either a string literal or a tuple of (string literal, args)");
    };
    error.into()
}

/// Schema 生成宏
///
/// 用于自动为结构体和枚举生成 ToSchema 实现。
///
/// # 示例
/// ```rust
/// #[derive(itools::Schema)]
/// pub struct User {
///     /// 用户名
///     pub name: String,
///     /// 年龄
///     pub age: u32,
///     /// 邮箱
///     pub email: Option<String>,
/// }
/// ```
#[proc_macro_derive(Schema)]
pub fn Schema(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    match input.data {
        Data::Struct(ref data) => handle_schema_struct(&input, data),
        Data::Enum(ref data) => handle_schema_enum(&input, data),
        _ => {
            let error: proc_macro2::TokenStream = parse_quote! {
                compile_error!("#[derive(Schema)] can only be used on structs and enums");
            };
            error.into()
        }
    }
}

/// 处理 Schema 结构体
fn handle_schema_struct(input: &DeriveInput, data: &DataStruct) -> TokenStream {
    let name = &input.ident;
    let generics = &input.generics;
    let where_clause = &generics.where_clause;

    let fields = match &data.fields {
        Fields::Named(fields) => fields.named.iter().map(|field| {
            let ident = field.ident.as_ref().unwrap();
            let ident_str = ident.to_string();
            let ty = &field.ty;
            quote! {
                .property(#ident_str, <#ty as itools::ToSchema>::schema())
            }
        }),
        _ => {
            let error: proc_macro2::TokenStream = parse_quote! {
                compile_error!("#[Schema] only supports named fields for structs");
            };
            return error.into();
        }
    };

    let expanded = quote! {
        impl itools::ToSchema for #name #generics #where_clause {
            fn schema() -> itools::Schema {
                itools::Schema::object()
                    #(#fields)*
            }
        }
    };

    expanded.into()
}

/// 处理 Schema 枚举
fn handle_schema_enum(input: &DeriveInput, data: &DataEnum) -> TokenStream {
    let name = &input.ident;
    let generics = &input.generics;
    let where_clause = &generics.where_clause;

    let variants = data.variants.iter().map(|variant| {
        let variant_ident = &variant.ident;
        let variant_str = variant_ident.to_string();
        match &variant.fields {
            Fields::Unit => {
                let variant_lit = syn::LitStr::new(&variant_str, variant_ident.span());
                quote! {
                    #variant_lit.to_string()
                }
            },
            _ => {
                let error: proc_macro2::TokenStream = parse_quote! {
                    compile_error!("#[Schema] only supports unit variants for enums");
                };
                return error.into();
            }
        }
    });

    let expanded = quote! {
        impl itools::ToSchema for #name #generics #where_clause {
            fn schema() -> itools::Schema {
                itools::Schema::string()
                    .enum_values(vec![
                        #(#variants,)*
                    ])
            }
        }
    };

    expanded.into()
}


