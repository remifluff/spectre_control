#![feature(proc_macro_quote)]
#![feature(trace_macros)]

#[macro_use]
extern crate proc_macro;

use proc_macro::{Ident, TokenStream};
use syn::{parse_macro_input, Data, DeriveInput, Fields};

// trait WgslUsable {
//     fn field_names() -> (&'static [&'static str], &'static [&'static str]);
//     fn struct_name() -> &'static str;
//     fn new_wgsl_block(group: i32, binding: i32) -> String;
// }

#[proc_macro_derive(StructToWgsl)]

pub fn struct_to_wgsl_derive(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);

    // Get the name of the struct being derived on
    let name = &input.ident;

    // Get the names and types of the struct's fields
    let (field_names, field_types): (Vec<_>, Vec<_>) = match input.data {
        Data::Struct(ref data) =>
            match data.fields {
                Fields::Named(ref fields) =>
                    fields.named.iter().map(|f| (f.ident.as_ref().unwrap(), &f.ty)).unzip(),
                _ => panic!("ZoomAndEnhance only works with named fields"),
            },
        _ => panic!("ZoomAndEnhance only works with structs"),
    };

    let generated_code = quote::quote! {

        impl UniformStruct for #name {
            fn field_names() -> (&'static [&'static str], &'static [&'static str]) {
            static NAMES: &'static [&'static str] = &[ #(stringify!(#field_names), )* ];

            static TYPES: &'static [&'static str] = &[ #( stringify!(#field_types), )* ];

            (NAMES, TYPES)

            }

            fn struct_name() -> &'static str {
                stringify!(#name)
            }

            fn new_wgsl_block(group: i32, binding: i32) -> String {
                let (field_names, field_types) = Self::field_names();
                let struct_name = Self::struct_name();
                let mut wgsl = String::new();
                wgsl.push_str(&format!("[[block]]\n"));
                wgsl.push_str(&format!("struct {} {{ \n",struct_name));

                for (name, typ) in field_names.iter().zip(field_types.iter()) {
                    let field_type = match *typ {
                        "f32" => "f32",
                        "u32" => "u32",
                        "i32" => "i32",
                        _ => panic!("Unsupported type: {}", typ),
                    };
                    wgsl.push_str(&format!("    {} : {};\n", name, field_type));
                }
                wgsl.push_str(&format!("}}; \n"));

                wgsl.push_str(&format!(
                    "[[group({}), binding({})]] \n var<uniform> {}: {}; \n",
                    group,
                    binding,
                    struct_name.to_lowercase(),
                    struct_name
                ));

                wgsl
            }
        };

    };
    generated_code.into()
}
