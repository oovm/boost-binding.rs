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
pub fn Command(_args: TokenStream, input: TokenStream) -> TokenStream {
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
pub fn Error(_args: TokenStream, input: TokenStream) -> TokenStream {
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
