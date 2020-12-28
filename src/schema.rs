#![allow(dead_code)]

use serde::{Deserialize, Serialize};

type Int = isize;
type Float = f64;
type Map<K, V> = std::collections::BTreeMap<K, V>;

// TODO: revisit public API

// TODO: docs

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
// TODO: not defined in the schema-schema, but described in authoring guide as a kind
struct Null;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct TypeName(String);

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
struct SchemaMap(Map<TypeName, Type>);

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct AdvancedDataLayoutName(String);

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
pub struct AdvancedDataLayoutMap(Map<AdvancedDataLayoutName, AdvancedDataLayout>);

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Schema {
    types: SchemaMap,
    #[serde(default, skip_serializing_if = "is_default")]
    advanced: AdvancedDataLayoutMap,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "kind", rename_all = "lowercase")]
pub enum Type {
    Bool(TypeBool),
    String(TypeString),
    Bytes(TypeBytes),
    Int(TypeInt),
    Float(TypeFloat),
    Map(TypeMap),
    List(TypeList),
    Link(TypeLink),
    Union(TypeUnion),
    Struct(TypeStruct),
    Enum(TypeEnum),
    Copy(TypeCopy),
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum TypeKind {
    Bool,
    String,
    Bytes,
    Int,
    Float,
    Map,
    List,
    Link,
    Union,
    Struct,
    Enum,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "lowercase")]
pub enum RepresentationKind {
    Bool,
    String,
    Bytes,
    Int,
    Float,
    Map,
    List,
    Link,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum AnyScalar {
    Bool(bool),
    String(String),
    Bytes(Vec<u8>),
    Int(Int),
    Float(Float),
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
struct AdvancedDataLayout;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct TypeBool;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct TypeString;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct TypeBytes {
    representation: BytesRepresentation,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "lowercase")]
enum BytesRepresentation {
    Bytes(bytes_representation::Bytes),
    Advanced(AdvancedDataLayoutName),
}

mod bytes_representation {
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
    pub struct Bytes;
}

impl Default for BytesRepresentation {
    fn default() -> Self {
        Self::Bytes(bytes_representation::Bytes)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct TypeInt;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct TypeFloat;

fn is_default<D: Default + PartialEq>(d: &D) -> bool {
    *d == D::default()
}

// mod implicit_field {
//     use serde::{Serializer, Deserializer};

//     pub fn serialize<S, T>(_: &T, _: S) -> Result<S::Ok, S::Error> where S: Serializer { todo!() }
//     pub fn deserialize<'de, D, T>(_: D) -> Result<T, D::Error> where D: Deserializer<'de> { todo!() }
// }

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct TypeMap {
    key_type: TypeName,
    value_type: TypeTerm,

    // TODO: confirm implicit semantics and implement special serde with-module for it
    #[serde(default, skip_serializing_if = "is_default")]
    // #[serde(with = "implicit_field")]
    value_nullable: bool,
    // TODO: implement Serialize and Deserialize for TypeMap based on its representation
    #[serde(default, skip_serializing_if = "is_default")]
    representation: MapRepresentation,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "lowercase")]
enum MapRepresentation {
    Map(map_representation::Map),
    StringPairs(map_representation::StringPairs),
    ListPairs(map_representation::ListPairs),
    Advanced(AdvancedDataLayoutName),
}

mod map_representation {
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
    pub struct Map;

    #[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
    #[serde(rename_all = "camelCase")]
    pub struct StringPairs {
        inner_delim: String,
        entry_delim: String,
    }

    #[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
    pub struct ListPairs;
}

impl Default for MapRepresentation {
    fn default() -> Self {
        Self::Map(map_representation::Map)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct TypeList {
    value_type: TypeTerm,

    #[serde(default, skip_serializing_if = "is_default")]
    value_nullable: bool,

    #[serde(default, skip_serializing_if = "is_default")]
    representation: ListRepresentation,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
enum ListRepresentation {
    List(list_representation::List),
    Advanced(AdvancedDataLayoutName),
}

mod list_representation {
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
    pub struct List;
}

impl Default for ListRepresentation {
    fn default() -> Self {
        Self::List(list_representation::List)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct TypeLink {
    expected_type: String,
}

impl Default for TypeLink {
    fn default() -> Self {
        Self {
            expected_type: "Any".to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct TypeUnion {
    representation: UnionRepresentation,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "lowercase")]
enum UnionRepresentation {
    Kinded(union_representation::Kinded),
    Keyed(union_representation::Keyed),
    Envelope(union_representation::Envelope),
    Inline(union_representation::Inline),
    BytePrefix(union_representation::BytePrefix),
}

mod union_representation {
    use super::{Int, Map, RepresentationKind, TypeName};
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
    pub struct Kinded(pub Map<RepresentationKind, TypeName>);

    #[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
    pub struct Keyed(pub Map<String, TypeName>);

    #[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
    #[serde(rename_all = "camelCase")]
    pub struct Envelope {
        pub discriminant_key: String,
        pub content_key: String,
        pub discriminant_table: Map<String, TypeName>,
    }

    #[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
    #[serde(rename_all = "camelCase")]
    pub struct Inline {
        pub discriminant_key: String,
        pub discriminant_table: Map<String, TypeName>,
    }

    #[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
    #[serde(rename_all = "camelCase")]
    pub struct BytePrefix {
        pub discriminant_table: Map<String, Int>,
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct TypeStruct {
    fields: Map<FieldName, StructField>,
    representation: StructRepresentation,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct FieldName(String);

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
struct StructField {
    r#type: TypeTerm,

    #[serde(default, skip_serializing_if = "is_default")]
    optional: bool,

    #[serde(default, skip_serializing_if = "is_default")]
    nullable: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum TypeTerm {
    TypeName(TypeName),
    InlineDefn(Box<InlineDefn>),
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "kind", rename_all = "lowercase")]
pub enum InlineDefn {
    Map(TypeMap),
    List(TypeList),
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "lowercase")]
enum StructRepresentation {
    Map(struct_representation::Map),
    Tuple(struct_representation::Tuple),
    StringPairs(struct_representation::StringPairs),
    StringJoin(struct_representation::StringJoin),
    ListPairs(struct_representation::ListPairs),
}

mod struct_representation {
    use super::{AnyScalar, FieldName};
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
    #[serde(rename_all = "camelCase")]
    pub struct Map {
        #[serde(default)]
        pub fields: Option<super::Map<FieldName, MapFieldDetails>>,
    }

    #[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
    #[serde(rename_all = "camelCase")]
    pub struct MapFieldDetails {
        pub rename: Option<String>,
        pub implicit: Option<AnyScalar>,
    }

    #[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
    #[serde(rename_all = "camelCase")]
    pub struct Tuple {
        field_order: Option<Vec<FieldName>>,
    }

    #[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
    #[serde(rename_all = "camelCase")]
    pub struct StringPairs {
        inner_delim: String,
        entry_delim: String,
    }

    #[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
    #[serde(rename_all = "camelCase")]
    pub struct StringJoin {
        join: String,
        field_order: Option<Vec<FieldName>>,
    }

    #[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
    pub struct ListPairs;
}

impl Default for StructRepresentation {
    fn default() -> Self {
        Self::Map(struct_representation::Map::default())
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct TypeEnum {
    members: Map<EnumValue, Null>,
    representation: EnumRepresentation,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct EnumValue(String);

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "lowercase")]
enum EnumRepresentation {
    String(enum_representation::String),
    Int(enum_representation::Int),
}

mod enum_representation {
    use super::{EnumValue, Map};
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
    pub struct String(Map<EnumValue, String>);

    #[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
    pub struct Int(Map<EnumValue, Int>);
}

impl Default for EnumRepresentation {
    fn default() -> Self {
        Self::String(enum_representation::String::default())
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct TypeCopy {
    from_type: TypeName,
}

peg::parser! {
    grammar schema_dsl() for str {
        rule _eof() -> () = ![_] { }
        rule _eol() -> () = "\n" / "\r\n" { }
        rule _ws1() -> () = " " / "\t" { }

        rule _comment() -> String = _ws1()* "#" s:$((!_eol() [_])*) _eol() { s.to_string() }
        rule _empty_line() -> () = _ws1()* _eol() { }
        rule _ws_block() -> () = (_comment() / _empty_line())* { }

        pub rule type_name() -> TypeName = cs:$(['A'..='Z'] (['A'..='Z'] / ['a'..='z'] / ['0'..='9'] / "_")*) { TypeName(cs.to_string()) }

        rule schema_map() -> SchemaMap = _ws_block() decls:(type_decl() ** _ws_block()) _ws_block() _eof() { SchemaMap(decls.into_iter().collect()) }

        // TODO: `advanced`
        pub rule parse() -> Schema = types:schema_map() { Schema { types, advanced: AdvancedDataLayoutMap::default() } }

        rule type_map() -> TypeMap = "{" _ws1()* n:type_name() _ws1()* ":" _ws1()* t:type_term() "}" { TypeMap { key_type: n, value_type: t, value_nullable: false, representation: MapRepresentation::default() } }

        // TODO: nullable and non-default representation
        rule type_list() -> TypeList = "[" _ws1()* t:type_term() _ws1()* "]" { TypeList { value_type: t, value_nullable: false, representation: ListRepresentation::default()} }

        rule t_bool() -> Type = "bool" { Type::Bool(TypeBool) }
        rule t_string() -> Type = "string" { Type::String(TypeString) }
        rule t_bytes() -> Type = "bytes" { unimplemented!("bytes") }
        rule t_int() -> Type = "int" { Type::Int(TypeInt) }
        rule t_float() -> Type = "float" { Type::Float(TypeFloat) }
        rule t_map() -> Type = m:type_map() { Type::Map(m) }
        rule t_list() -> Type = l:type_list() { Type::List(l) }
        rule t_link() -> Type = "link" { unimplemented!("link") }
        rule t_union() -> Type = r:union_representation() { Type::Union(TypeUnion { representation: r }) }
        rule t_struct() -> Type = s:struct_model() { Type::Struct(s) }
        rule t_enum() -> Type = "enum" _ws1()* "{" _ws_block() ms:(enum_member()*) _ws_block() "}" { Type::Enum(TypeEnum { members: ms.into_iter().map(|m| (m, Null)).collect(), representation: EnumRepresentation::default() }) }
        rule t_copy() -> Type = "copy" { unimplemented!("copy") }
        rule r#type() -> Type = t:(
            t_bool() /
            t_string() /
            t_bytes() /
            t_int() /
            t_float() /
            t_map() /
            t_list() /
            t_link() /
            t_union() /
            t_struct() /
            t_enum() /
            t_copy()
        ) { t }
        pub rule schema_type() -> Type = t:r#type() { t }


        rule union_representation() -> UnionRepresentation = ur:(
            ur_kinded() /
            ur_keyed() /
            ur_inline()
        ) { ur }
        rule ur_kinded() -> UnionRepresentation = "union" _ws1()* "{" _ws_block() ts:(type_name_and_representation_kind()*) _ws1()* "}" _ws1()* "representation" _ws1()+ "kinded" _ws1()* (_eol() / _eof()) { UnionRepresentation::Kinded(union_representation::Kinded(ts.into_iter().map(|(tn, rk)| (rk, tn)).collect())) }
        rule ur_keyed() -> UnionRepresentation = "union" _ws1()* "{" _ws_block() ts:(type_name_and_string()*) _ws1()* "}" _ws1()* "representation" _ws1()+ "keyed" _ws1()* (_eol() / _eof())  { UnionRepresentation::Keyed(union_representation::Keyed(ts.into_iter().map(|(tn, s)| (s, tn)).collect())) }
        rule ur_inline() -> UnionRepresentation = "union" _ws1()* "{" _ws_block() ts:(type_name_and_string()*) _ws1()* "}" _ws1()* "representation" _ws1()+ "inline" _ws1()* "{" _ws_block() _ws1()* "discriminantKey" _ws1()+ k:string() _ws_block() "}" (_eol() / _eof())  { UnionRepresentation::Inline(union_representation::Inline { discriminant_key: k, discriminant_table: ts.into_iter().map(|(tn, s)| (s, tn)).collect() }) }

        rule type_name_and_string() -> (TypeName, String) = _ws1()* "|" _ws1()* t:type_name() _ws1()+ s:string() _ws1()* _eol() { (t, s) }
        rule string() -> String = "\"" cs:$((!"\"" [_])*) "\"" { cs.to_string() }

        rule type_name_and_representation_kind() -> (TypeName, RepresentationKind) = _ws1()* "|" _ws1()* t:type_name() _ws1()+ r:representation_kind() _ws1()* _eol() { (t, r) }
        rule rk_bool() -> RepresentationKind = "bool" { RepresentationKind::Bool }
        rule rk_string() -> RepresentationKind = "string" { RepresentationKind::String }
        rule rk_bytes() -> RepresentationKind = "bytes" { RepresentationKind::Bytes }
        rule rk_int() -> RepresentationKind = "int" { RepresentationKind::Int }
        rule rk_float() -> RepresentationKind = "float" { RepresentationKind::Float }
        rule rk_map() -> RepresentationKind = "map" { RepresentationKind::Map }
        rule rk_list() -> RepresentationKind = "list" { RepresentationKind::List }
        rule rk_link() -> RepresentationKind = "link" { RepresentationKind::Link }
        rule representation_kind() -> RepresentationKind = r:(
            rk_bool() /
            rk_string() /
            rk_bytes() /
            rk_int() /
            rk_float() /
            rk_map() /
            rk_list() /
            rk_link()
        ) { r }

        rule as_bool_false() -> AnyScalar = "\"false\"" { AnyScalar::Bool(false) }
        rule as_bool_true() -> AnyScalar = "\"true\"" { AnyScalar::Bool(true) }
        rule as_string() -> AnyScalar = s:string() { AnyScalar::String(s) }
        rule as_bytes() -> AnyScalar = "x" { todo!() }
        rule as_int() -> AnyScalar = "x" { todo!() }
        rule as_float() -> AnyScalar = "x" { todo!() }
        rule any_scalar() -> AnyScalar = a:(
            as_bool_false() /
            as_bool_true() /
            as_string() /
            as_bytes() /
            as_int() /
            as_float()
        ) { a }


        pub rule field_name() -> FieldName = cs:$((['A'..='Z'] / ['a'..='z'] / ['0'..='9'] / "_")+) { FieldName(cs.to_string()) }
        rule struct_field() -> StructField = o:("optional" _ws1()+)? /* TODO: nullable */ t:type_term() { StructField { r#type: t, optional: o.is_some(), nullable: false } }

        rule tt_type_name() -> TypeTerm = n:type_name() { TypeTerm::TypeName(n) }
        rule id_map() -> InlineDefn = m:type_map() { InlineDefn::Map(m) }
        rule id_list() -> InlineDefn = l:type_list() { InlineDefn::List(l) }
        rule tt_inline_defn() -> TypeTerm = i:(id_map() / id_list()) { TypeTerm::InlineDefn(Box::new(i)) }
        pub rule type_term() -> TypeTerm = tt:(tt_type_name() / tt_inline_defn()) { tt }

        rule st_map() -> TypeStruct = "struct" _ws1()* "{" _ws_block() fs:(st_map_field()*) _ws1()* "}" {
            let fields = fs.iter().cloned().map(|(f, s, _)| (f, s)).collect();
            let representation = if fs.iter().all(|(_, _, x)| x.is_none()) {
                StructRepresentation::default()
            } else {
                StructRepresentation::Map(struct_representation::Map { fields: Some(fs.into_iter().filter_map(|(f, _, x)| x.map(|x| (f, x))).collect()) })
            };

            TypeStruct { fields, representation }
        }
        rule st_map_field() -> (FieldName, StructField, Option<struct_representation::MapFieldDetails>) = _ws1()* n:field_name() _ws1()+ f:struct_field() x:st_map_field_details()? _ws1()* _eol() { dbg!((n, f, x)) }
        rule st_map_field_details() -> struct_representation::MapFieldDetails = _ws1()* "(" _ws1()* "implicit" _ws1()+ i:any_scalar()? _ws1()* ")" { struct_representation::MapFieldDetails { implicit: i, rename: None } }
        pub rule struct_model() -> TypeStruct = s:(
            st_map()
        ) { s }


        pub rule enum_value() -> EnumValue = cs:$((['A'..='Z'] / ['a'..='z'] / ['0'..='9'] / "_")+) { EnumValue(cs.to_string()) }
        rule enum_member() -> EnumValue = _ws1()* "|" _ws1()* ev:enum_value() _ws1()* _eol() { ev }

        rule type_decl() -> (TypeName, Type) = "type" _ws1()+ n:type_name() _ws1()+ t:r#type() (_eol() / _eof()) { (n, t) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::fs::read_to_string;

    use insta::{assert_debug_snapshot, assert_json_snapshot, with_settings};
    use pretty_assertions::assert_eq;

    fn schema_schema() -> Schema {
        schema_dsl::parse(&read_to_string("./specs/schemas/schema-schema.ipldsch").unwrap())
            .unwrap()
    }

    fn schema_schema_json() -> String {
        read_to_string("./specs/schemas/schema-schema.ipldsch.json").unwrap()
    }

    #[test]
    fn snapshot_of_parsed_schema_schema() {
        assert_debug_snapshot!(schema_schema());
    }

    #[test]
    fn struct_representation_tuple_reifies_correctly() {
        let actual = schema_dsl::schema_type(
            r#"struct {
            fieldOrder optional [FieldName]
        }"#,
        )
        .unwrap();

        let expected: Type = serde_json::from_str(
            r#"
        {
			"kind": "struct",
			"fields": {
				"fieldOrder": {
					"type": {
						"kind": "list",
						"valueType": "FieldName"
					},
					"optional": true
				}
			},
			"representation": {
				"map": {}
			}
		}
        "#,
        )
        .unwrap();

        assert_eq!(actual, expected);
    }

    #[test]
    fn reified_form_of_schema_schema_matches_parsed_dsl_form() {
        assert_eq!(
            schema_schema(),
            serde_json::from_str(&schema_schema_json()).unwrap()
        );
    }

    #[test]
    fn snapshot_of_reified_json_form_of_schema_schema() {
        with_settings!({sort_maps => true}, {
            assert_json_snapshot!(schema_schema())
        });
    }
}
