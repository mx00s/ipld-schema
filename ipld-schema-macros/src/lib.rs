extern crate proc_macro;

mod schema;

use proc_macro::TokenStream;
use proc_macro2 as pm2;
use quote::quote;
use schema::*;
use std::fmt::Write;
use syn::parse_macro_input;

// TODO: finish proc-macro implementation

// TODO: add serde hints so reified JSON representation is correct

// expects a string literal representing the path to an IPLD schema file (relative to the consuming crate's cargo manifest directory)
#[proc_macro_attribute]
pub fn ipld_schema(attr: TokenStream, item: TokenStream) -> TokenStream {
    let path = parse_macro_input!(attr as syn::LitStr);
    let types = types_for(load_schema(&path.value()));

    let item = parse_macro_input!(item as syn::ItemMod);

    (quote! {
        #types

        #item
    })
    .into()
}

fn load_schema(path: &str) -> Schema {
    let path = format!("{}/{}", env!("CARGO_MANIFEST_DIR"), path);
    let contents = std::fs::read_to_string(&path).unwrap();
    contents.parse::<Schema>().unwrap()
}

fn types_for(schema: Schema) -> pm2::TokenStream {
    // TODO: handle schema.advanced

    let mut result = quote! {
        use serde::{Serialize, Deserialize};
        use proptest::{collection::btree_map, prelude::*};

        const DEFAULT_SIZE_RANGE: std::ops::RangeInclusive<usize> = 0..=10;

        type Bool = bool;
        // type String = String;
        type Bytes = Vec<u8>;
        type Int = i64;
        type Float = f64;
        type Map<K, V> = std::collections::BTreeMap<K, V>;
        type Null = ();
    };

    for (name, ty) in schema.types.0 {
        let decl = match ty {
            Type::String(t) => type_string(&name, t),
            Type::Map(t) => type_map(&name, t),
            Type::Union(t) => type_union(&name, t),
            Type::Struct(t) => type_struct(&name, t),
            Type::Enum(t) => type_enum(&name, t),
            _ => {
                todo!("generate rust type for {:?}", ty);
            }
        };

        result.extend(vec![
            quote! {
                #[derive(Serialize, Deserialize, PartialEq, PartialOrd, Clone, Debug, test_strategy::Arbitrary)]
                #decl

                // TODO: Reconsider whether every type for every schema should have Display and FromStr impls.
                //   It's convenient for the schema-schema types because their implementations enable
                //   parsing and unparsing schemas. However, the Display and FromStr impls for types from other
                //   schemas don't have much reason to parse/unparse their DSL representations.
                //
                //   A function that takes a TypeName and renders its DSL definition would be heplful, though.

                impl std::fmt::Display for #name {
                    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        todo!("render DSL form of type (ideally a copy from the source along with preceding comments)")
                    }
                }

                impl std::str::FromStr for #name {
                    type Err = ();

                    fn from_str(_s: &str) -> Result<Self, Self::Err> {
                        todo!("implement DSL parser for generated types for schema-schema")
                    }
                }
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
            use super::*;

            #[test]
            #[ignore = "Doesn't yield consistent results in CI"]
            fn macro_snapshot() {
                // TODO: compute relative path instead
                let manifest = concat!(env!("CARGO_MANIFEST_DIR"), "/Cargo.toml");
                let parent_module = module_path!()
                    // TODO: don't assume the name of the calling crate
                    .strip_prefix("ipld_schema")
                    .unwrap()
                    .strip_suffix("::generated_tests")
                    .unwrap();

                let cmd_result = std::process::Command::new("cargo")
                    .args(&["+nightly", "expand", "--manifest-path", manifest, "--lib", "--tests", parent_module])
                    .output()
                    .unwrap();

                insta::assert_snapshot!(
                    String::from_utf8_lossy(&cmd_result.stdout)
                );
            }

            #[cfg(feature = "fast-test")]
            const CASES: u32 = 10;
            #[cfg(not(feature = "fast-test"))]
            const CASES: u32 = 1000;

            #[cfg(feature = "fast-test")]
            const MAX_SHRINK_ITERS: u32 = 100;
            #[cfg(not(feature = "fast-test"))]
            const MAX_SHRINK_ITERS: u32 = 10000;

            macro_rules! type_tests {
                ($($t:ty,)*) => {
                    mod roundtrip_json {
                        use super::*;

                        $(paste::paste! {
                            #[test_strategy::proptest(cases = CASES, max_shrink_iters = MAX_SHRINK_ITERS)]
                            #[allow(non_snake_case)]
                            fn [<$t>](val: $t) {
                                let there = serde_json::to_string(&val).unwrap();
                                let back = serde_json::from_str(&there).unwrap();

                                proptest::prop_assert_eq!(val, back);
                            }
                        })*
                    }

                    mod roundtrip_dsl {
                        use super::*;

                        $(paste::paste! {
                            #[test_strategy::proptest(cases = CASES, max_shrink_iters = MAX_SHRINK_ITERS)]
                            #[allow(non_snake_case)]
                            #[ignore = "implement ToString and FromStr for generated types first"]
                            fn [<$t>](val: $t) {
                                let there = val.to_string();
                                let back = there.parse::<$t>().unwrap();

                                for (n, line) in there.lines().enumerate() {
                                    eprintln!("  {:>4}  │ {}", n + 1, line);
                                }

                                proptest::prop_assert_eq!(val, back);
                            }
                        })*
                    }
                }
            }

            // TODO: generate set of types so their tests can be implemented automatically

            type_tests!(
                /* STRUCTS */
                AdvancedDataLayout,
                AdvancedDataLayoutMap,
                AdvancedDataLayoutName,
                BytesRepresentation_Bytes,
                EnumRepresentation_Int,
                EnumRepresentation_String,
                EnumValue,
                FieldName,
                ListRepresentation_List,
                MapRepresentation_ListPairs,
                MapRepresentation_Map,
                MapRepresentation_StringPairs,
                Schema,
                SchemaMap,
                StructField,
                StructRepresentation_ListPairs,
                StructRepresentation_Map,
                StructRepresentation_Map_FieldDetails,
                StructRepresentation_StringJoin,
                StructRepresentation_StringPairs,
                StructRepresentation_Tuple,
                TypeBool,
                TypeBytes,
                TypeCopy,
                TypeEnum,
                TypeFloat,
                TypeInt,
                TypeLink,
                TypeList,
                TypeMap,
                TypeName,
                TypeString,
                TypeStruct,
                TypeUnion,
                UnionRepresentation_BytePrefix,
                UnionRepresentation_Envelope,
                UnionRepresentation_Inline,
                UnionRepresentation_Keyed,
                UnionRepresentation_Kinded,
                /* ENUMS */
                AnyScalar,
                BytesRepresentation,
                EnumRepresentation,
                InlineDefn,
                ListRepresentation,
                MapRepresentation,
                RepresentationKind,
                StructRepresentation,
                Type,
                TypeKind,
                TypeTerm,
                UnionRepresentation,
            );
        }
    }
}

fn type_string(name: &TypeName, _t: TypeString) -> pm2::TokenStream {
    quote! {
        // TODO: relax the strategy
        #[derive(Eq, Ord)]
        pub struct #name(#[strategy("[A-Z][a-z0-9_]*")] pub String);
    }
}

fn type_map(name: &TypeName, t: TypeMap) -> pm2::TokenStream {
    let k = &t.key_type;
    let v = &t.value_type;

    match &t.representation {
        MapRepresentation::Map(_m) => {
            // TODO: relax the strategy
            quote! {
                pub struct #name(#[strategy(btree_map(any::<#k>(), any::<#v>(), DEFAULT_SIZE_RANGE))] pub Map<#k, #v>);
            }
        }
        _ => {
            todo!("handle map representation: {:?}", t);
        }
    }
}

fn type_union(name: &TypeName, t: TypeUnion) -> pm2::TokenStream {
    match &t.representation {
        UnionRepresentation::Kinded(r) => {
            let mut decl = String::new();
            writeln!(decl, "pub enum {} {{", name).unwrap();
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

                // TODO: remove after fixing stack overflow and float roundtripping in proptest strategies
                if (name.to_string() == "TypeTerm" && ty == "Map")
                    || (name.to_string() == "AnyScalar" && ty == "Float")
                {
                    writeln!(decl, "    #[weight(0)]").unwrap();
                }

                writeln!(decl, "    r#{}({}),", ty, ty_name).unwrap();
            }
            writeln!(decl, "}}").unwrap();

            // TODO: `impl From<#kind> for #name` for each variant
            // TODO: `impl From<#name> for #kind` for each variant

            decl.parse().unwrap()
        }
        UnionRepresentation::Keyed(r) => {
            let mut decl = String::new();
            writeln!(decl, "pub enum {} {{", name).unwrap();
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
            writeln!(decl, "pub enum {} {{", name).unwrap();
            for (discrim, ty_name) in &r.discriminant_table {
                writeln!(decl, "    r#{}({}),", discrim, ty_name).unwrap();
            }
            writeln!(decl, "}}").unwrap();

            decl.parse().unwrap()
        }
        _ => todo!(),
    }
}

fn type_struct(name: &TypeName, t: TypeStruct) -> pm2::TokenStream {
    match t.representation {
        StructRepresentation::Map(_r) => {
            let mut decl = String::new();
            writeln!(decl, "pub struct {} {{", name).unwrap();
            for (field_name, struct_field) in t.fields {
                let ty = type_term(struct_field.r#type.clone());

                // TODO: handle struct_field's `optional` and `nullable` flags

                // TODO: only use Box when necessary

                if let TypeTerm::InlineDefn(inline) = struct_field.r#type {
                    if let InlineDefn::Map(m) = *inline {
                        let k = &m.key_type;
                        let v = &m.value_type;
                        writeln!(decl, "    {}", quote!(#[strategy(btree_map(any::<#k>(), any::<#v>(), DEFAULT_SIZE_RANGE).prop_map(Box::new))])).unwrap();
                    }
                }
                writeln!(decl, "    pub r#{}: Box<{}>,", field_name, ty).unwrap();
            }
            writeln!(decl, "}}").unwrap();
            decl.parse().unwrap()
        }
        _ => {
            todo!("handle struct representation: {:?}", t)
        }
    }
}

fn type_term(t: TypeTerm) -> pm2::TokenStream {
    match t {
        TypeTerm::TypeName(name) => name.to_string().parse().unwrap(),
        TypeTerm::InlineDefn(inline) => match *inline {
            InlineDefn::Map(m) => {
                let k = &m.key_type;
                let v = &m.value_type;

                // TODO: handle `value_nullable` and `representation`

                quote! {
                    Map<#k, #v>
                }
            }
            InlineDefn::List(l) => {
                let v = &l.value_type;

                // TODO: handle `value_nullable` and `representation`

                quote! {
                    Vec<#v>
                }
            }
        },
    }
}

fn type_enum(name: &TypeName, t: TypeEnum) -> pm2::TokenStream {
    match t.representation {
        EnumRepresentation::String(r) => {
            let mut decl = String::new();
            writeln!(decl, "#[derive(Eq, Ord)]").unwrap();
            writeln!(decl, "pub enum {} {{", name).unwrap();
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
