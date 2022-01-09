use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, Data, DeriveInput, Fields, FieldsNamed};

#[proc_macro_derive(Builder)]
pub fn derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    let target_name = &ast.ident;
    let builder_name = format_ident!("{}Builder", &ast.ident);
    let builder_fields = builder_fields_token(&ast.data);
    let init_builder_fields = init_builder_fields_token(&ast.data);
    let setters = setters_token(&ast.data);

    let tokens = quote! {
        impl #target_name {
            pub fn builder() -> #builder_name {
                #builder_name {
                    #init_builder_fields
                }
            }
        }

        pub struct #builder_name {
            #builder_fields
        }

        impl #builder_name {
            #setters
        }
    };

    tokens.into()
}

/// 構造体の本体、フィールドリストのトークンを生成する
fn builder_fields_token(data: &Data) -> TokenStream {
    let fields = fields_token(data).named.iter().map(|field| {
        let ty = &field.ty;
        let ident = &field.ident;
        quote! {
            #ident: std::option::Option<#ty>
        }
    });

    quote! {
        #(#fields),*
    }
}

/// Builder を初期化するコードのトークンを生成する
fn init_builder_fields_token(data: &Data) -> TokenStream {
    let fields = fields_token(data).named.iter().map(|field| {
        let ident = &field.ident;
        quote! {
            #ident: None
        }
    });

    quote! {
        #(#fields),*
    }
}

/// フィールドのトークンを返す
///
/// ここでは構造体以外に Builder を適用することを考えないので、それ以外の場合は panic するようにしてある
fn fields_token(data: &Data) -> &FieldsNamed {
    match *data {
        Data::Struct(ref s) => match s.fields {
            Fields::Named(ref fields) => fields,
            _ => panic!("Fields must be named"),
        },
        _ => panic!("Builder can apply for only struct"),
    }
}

/// すべてのフィールドに対する setter のトークンを生成する
fn setters_token(data: &Data) -> TokenStream {
    let setters = fields_token(data).named.iter().map(|field| {
        let ty = &field.ty;
        let ident = &field.ident;
        quote! {
            pub fn #ident(&mut self, #ident: #ty) -> &mut Self {
                self.#ident = Some(#ident);
                self
            }
        }
    });

    quote! {
        #(#setters)*
    }
}
