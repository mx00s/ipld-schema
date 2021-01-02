extern crate proc_macro;

use ipld_schema_model::schema::*;
use proc_macro::TokenStream;
use proc_macro2 as pm2;
use quote::quote;
use std::fmt::Write;
use syn::parse_macro_input;

// TODO: finish proc-macro implementation

#[proc_macro]
pub fn schema_data_types(input: TokenStream) -> TokenStream {
    dbg!(&input);
    let schema = parse_macro_input!(input as Schema);

    // TODO: handle schema.advanced

    let mut result = TokenStream::new();

    for (name, ty) in schema.types.0 {
        dbg!((&name, &ty));

        let decl = match ty {
            Type::String(t) => type_string(name, t),
            Type::Map(t) => type_map(name, t),
            Type::Union(t) => type_union(name, t),
            Type::Struct(t) => type_struct(name, t),
            _ => {
                dbg!(&ty);
                todo!("other types")
            }
        };

        dbg!(decl.to_string());

        result.extend(vec![TokenStream::from(decl)]);
    }

    result
}

fn type_string(name: TypeName, _t: TypeString) -> pm2::TokenStream {
    quote! { struct #name(String); }
}

fn type_map(name: TypeName, t: TypeMap) -> pm2::TokenStream {
    let k = &t.key_type;
    let v = &t.value_type;

    match &t.representation {
        MapRepresentation::Map(m) => {
            dbg!(&m);
            quote! {
                struct #name(std::collections::BTreeMap<#k, #v>);
            }
        }
        _ => {
            dbg!(t);
            todo!("handle other map representations");
        }
    }
}

fn type_union(name: TypeName, t: TypeUnion) -> pm2::TokenStream {
    match &t.representation {
        UnionRepresentation::Kinded(u) => {
            let mut decl = String::new();
            write!(decl, "enum {} {{", name).unwrap();
            for (kind, ty_name) in &u.0 {
                todo!("kinded union variants")
            }
            write!(decl, "}}").unwrap();
            decl.parse().unwrap()
        }
        _ => todo!(),
    }
}

fn type_struct(name: TypeName, t: TypeStruct) -> pm2::TokenStream {
    match t.representation {
        StructRepresentation::Map(_s) => {
            let mut decl = String::new();
            write!(decl, "struct {} {{", name).unwrap();
            for (field_name, struct_field) in t.fields {
                dbg!(&field_name);
                dbg!(&struct_field);

                // TODO: check if s.fields.get(field_name) has any field details

                // TODO: handle struct field
            }
            write!(decl, "}}").unwrap();
            decl.parse().unwrap()
        }
        _ => {
            dbg!(t);
            todo!("handle other struct representations")
        }
    }
}
