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

    let mut result = quote! {
        use serde::{Serialize, Deserialize};

        type Bool = bool;
        // type String = String;
        type Bytes = Vec<u8>;
        type Int = i64;
        type Float = f64;
        type Map<K, V> = std::collections::BTreeMap<K, V>;
    };

    for (name, ty) in schema.types.0 {
        // dbg!((&name, &ty));

        let decl = match ty {
            Type::String(t) => type_string(name, t),
            Type::Map(t) => type_map(name, t),
            Type::Union(t) => type_union(name, t),
            Type::Struct(t) => type_struct(name, t),
            Type::Enum(t) => type_enum(name, t),
            _ => {
                dbg!(&ty);
                todo!("other types")
            }
        };

        // dbg!(decl.to_string());

        result.extend(vec![
            quote! {
                #[derive(Serialize, Deserialize, PartialEq, PartialOrd, Clone, Debug, test_strategy::Arbitrary)]
                #decl
            },
        ]);
    }

    // TODO: only use if user enables a feature
    result.extend(vec![generated_tests()]);

    result.into()
}

fn generated_tests() -> pm2::TokenStream {
    quote! {
        #[cfg(test)]
        mod generated_tests {
            #[test]
            fn macro_snapshot() {
                // TODO: compute relative path instead
                let manifest = concat!(env!("CARGO_MANIFEST_DIR"), "/Cargo.toml");
                let parent_module = module_path!()
                    // TODO: don't assume the name of the calling crate
                    .strip_prefix("ipld_schema")
                    .unwrap()
                    .strip_suffix("::generated_tests")
                    .unwrap();

                insta::assert_snapshot!(
                    String::from_utf8_lossy(&std::process::Command::new("cargo")
                        .args(&["expand", "--manifest-path", manifest, "--lib", "--tests", parent_module])
                        .output()
                        .unwrap()
                        .stdout)
                );
            }

            // TODO: roundtripping through serialize and deserialize
            //   - build up a set of Rust types so tests can be generated for each
        }
    }
}

fn type_string(name: TypeName, _t: TypeString) -> pm2::TokenStream {
    quote! {
        #[derive(Eq, Ord)]
        struct #name(String);
    }
}

fn type_map(name: TypeName, t: TypeMap) -> pm2::TokenStream {
    let k = &t.key_type;
    let v = &t.value_type;

    match &t.representation {
        MapRepresentation::Map(_m) => {
            // TODO: ensure Ord is implemented for #k
            quote! {
                struct #name(Map<#k, #v>);
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
        UnionRepresentation::Kinded(r) => {
            let mut decl = String::new();
            writeln!(decl, "enum {} {{", name).unwrap();
            for (kind, ty_name) in &r.0 {
                let ty = match kind {
                    RepresentationKind::Bool => "Bool",
                    RepresentationKind::String => "String",
                    RepresentationKind::Bytes => "Bytes",
                    RepresentationKind::Int => "Int",
                    RepresentationKind::Float => "Float",
                    RepresentationKind::Map => "Map",
                    RepresentationKind::List => "List",
                    RepresentationKind::Link => "Link",
                };
                writeln!(decl, "    r#{}({}),", ty, ty_name).unwrap();
            }
            writeln!(decl, "}}").unwrap();

            // TODO: `impl From<#kind> for #name` for each variant
            // TODO: `impl From<#name> for #kind` for each variant

            decl.parse().unwrap()
        }
        UnionRepresentation::Keyed(r) => {
            let mut decl = String::new();
            writeln!(decl, "enum {} {{", name).unwrap();
            for (discrim, ty_name) in &r.0 {
                writeln!(decl, "    r#{}({}),", discrim, ty_name).unwrap();
            }
            writeln!(decl, "}}").unwrap();

            decl.parse().unwrap()
        }
        UnionRepresentation::Inline(r) => {
            // TODO: have serde use this
            let _discriminant_key = &r.discriminant_key;

            let mut decl = String::new();
            writeln!(decl, "enum {} {{", name).unwrap();
            for (discrim, ty_name) in &r.discriminant_table {
                writeln!(decl, "    r#{}({}),", discrim, ty_name).unwrap();
            }
            writeln!(decl, "}}").unwrap();

            decl.parse().unwrap()
        }
        _ => todo!(),
    }
}

fn type_struct(name: TypeName, t: TypeStruct) -> pm2::TokenStream {
    match t.representation {
        StructRepresentation::Map(_r) => {
            let mut decl = String::new();
            writeln!(decl, "struct {} {{", name).unwrap();
            for (_field_name, _struct_field) in t.fields {
                // dbg!(&field_name);
                // dbg!(&struct_field);

                // TODO: check if s.fields.get(field_name) has any field details

                // TODO: handle struct field

                // TODO: use raw identifier (r#) for field names
            }
            writeln!(decl, "}}").unwrap();
            decl.parse().unwrap()
        }
        _ => {
            dbg!(t);
            todo!("handle other struct representations")
        }
    }
}

fn type_enum(name: TypeName, t: TypeEnum) -> pm2::TokenStream {
    match t.representation {
        EnumRepresentation::String(r) => {
            let mut decl = String::new();
            writeln!(decl, "#[derive(Eq, Ord)]").unwrap();
            writeln!(decl, "enum {} {{", name).unwrap();
            for (val, _null) in t.members {
                let _name = r.0.get(&val).unwrap_or(&val.to_string());
                // TODO: have serde associate `_name` with this variant
                writeln!(decl, "    r#{},", val).unwrap();
            }
            writeln!(decl, "}}").unwrap();
            decl.parse().unwrap()
        }
        EnumRepresentation::Int(_r) => todo!("enum int representation"),
    }
}
