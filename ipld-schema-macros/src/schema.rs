#![allow(dead_code)]

// TODO: Consider switching to modeling the synatic forms of schemas
//   rather than what they represent conceptually. For example, make a
//   type to describe a brace-enclosed segment of code along with a generic
//   parameter to describe the shape of its contents. Then
//   the import_schema macro can generate conceptual types from compositions
//   of these syntactic forms.

use std::{fmt, str::FromStr};

use proptest::{collection::btree_map, prelude::*};
use quote::{quote, ToTokens};
use serde::{Deserialize, Serialize};

#[cfg(feature = "fast-test")]
const DEFAULT_SIZE_RANGE: std::ops::RangeInclusive<usize> = 0..=10;
#[cfg(not(feature = "fast-test"))]
const DEFAULT_SIZE_RANGE: std::ops::RangeInclusive<usize> = 0..=100;

type Int = i64;
type Float = f64;
type Map<K, V> = std::collections::BTreeMap<K, V>;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, test_strategy::Arbitrary)]
pub struct Null;

#[derive(
    Serialize, Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, test_strategy::Arbitrary,
)]
pub struct TypeName(#[strategy("[A-Z][a-z0-9_]*")] String);

impl ToTokens for TypeName {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let name = proc_macro2::Ident::new(&self.0, proc_macro2::Span::call_site());
        tokens.extend(vec![quote!(#name)])
    }
}

impl ToTokens for TypeTerm {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match self {
            TypeTerm::TypeName(t) => t.to_tokens(tokens),
            TypeTerm::InlineDefn(_) => unimplemented!(
                "macros must determine how inline definitions are representated as Rust types"
            ),
        };
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, test_strategy::Arbitrary)]
pub struct SchemaMap(
    #[strategy(btree_map(any::<TypeName>(), any::<Type>(), DEFAULT_SIZE_RANGE))]
    pub  Map<TypeName, Type>,
);

#[derive(
    Serialize, Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, test_strategy::Arbitrary,
)]
pub struct AdvancedDataLayoutName(String);

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default, test_strategy::Arbitrary)]
pub struct AdvancedDataLayoutMap(
    #[strategy(Just(Map::new()))] pub Map<AdvancedDataLayoutName, AdvancedDataLayout>,
);

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
#[derive(test_strategy::Arbitrary)]
pub struct Schema {
    pub types: SchemaMap,
    #[serde(default, skip_serializing_if = "is_default")]
    pub advanced: AdvancedDataLayoutMap,
}

impl Schema {
    fn schema_schema() -> Self {
        std::include_str!("../../specs/schemas/schema-schema.ipldsch")
            .parse()
            .unwrap()
    }
}

impl FromStr for Schema {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        schema_dsl::parse(s).or(Err(()))
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "kind", rename_all = "lowercase")]
#[derive(test_strategy::Arbitrary)]
// TODO: can't handle some variants until fields referred to by representation exist and field orders matches set of fields
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
    #[weight(0)]
    Struct(TypeStruct),
    #[weight(0)]
    Enum(TypeEnum),
    Copy(TypeCopy),
}

/*
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "lowercase")]
#[derive(test_strategy::Arbitrary)]
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
*/

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "lowercase")]
#[derive(test_strategy::Arbitrary)]
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
#[derive(test_strategy::Arbitrary)]
pub enum AnyScalar {
    Bool(bool),
    String(String),
    Bytes(Vec<u8>),
    Int(Int),
    Float(Float),
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, test_strategy::Arbitrary)]
pub struct AdvancedDataLayout;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, test_strategy::Arbitrary)]
pub struct TypeBool;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, test_strategy::Arbitrary)]
pub struct TypeString;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, test_strategy::Arbitrary)]
pub struct TypeBytes {
    representation: BytesRepresentation,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "lowercase")]
#[derive(test_strategy::Arbitrary)]
// TODO: generate all variants
enum BytesRepresentation {
    Bytes(bytes_representation::Bytes),
    #[weight(0)]
    Advanced(AdvancedDataLayoutName),
}

mod bytes_representation {
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, Clone, Debug, PartialEq, test_strategy::Arbitrary)]
    pub struct Bytes;
}

impl Default for BytesRepresentation {
    fn default() -> Self {
        Self::Bytes(bytes_representation::Bytes)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, test_strategy::Arbitrary)]
pub struct TypeInt;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, test_strategy::Arbitrary)]
pub struct TypeFloat;

fn is_default<D: Default + PartialEq>(d: &D) -> bool {
    *d == D::default()
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
#[derive(test_strategy::Arbitrary)]
pub struct TypeMap {
    pub key_type: TypeName,

    pub value_type: TypeTerm,

    #[serde(default, skip_serializing_if = "is_default")]
    pub value_nullable: bool,

    #[serde(default, skip_serializing_if = "is_default")]
    pub representation: MapRepresentation,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "lowercase")]
#[derive(test_strategy::Arbitrary)]
// TODO: generate all variants
pub enum MapRepresentation {
    Map(map_representation::Map),
    StringPairs(map_representation::StringPairs),
    ListPairs(map_representation::ListPairs),
    #[weight(0)]
    Advanced(AdvancedDataLayoutName),
}

mod map_representation {
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, Clone, Debug, PartialEq, test_strategy::Arbitrary)]
    pub struct Map;

    #[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
    #[serde(rename_all = "camelCase")]
    #[derive(test_strategy::Arbitrary)]
    pub struct StringPairs {
        #[strategy("[^\"]+")]
        pub inner_delim: String,

        #[strategy("[^\"]+")]
        pub entry_delim: String,
    }

    #[derive(Serialize, Deserialize, Clone, Debug, PartialEq, test_strategy::Arbitrary)]
    pub struct ListPairs;
}

impl Default for MapRepresentation {
    fn default() -> Self {
        Self::Map(map_representation::Map)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
#[derive(test_strategy::Arbitrary)]
pub struct TypeList {
    pub value_type: TypeTerm,

    #[serde(default, skip_serializing_if = "is_default")]
    pub value_nullable: bool,

    #[serde(default, skip_serializing_if = "is_default")]
    pub representation: ListRepresentation,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, test_strategy::Arbitrary)]
pub enum ListRepresentation {
    List(list_representation::List),
    #[weight(0)]
    Advanced(AdvancedDataLayoutName),
}

mod list_representation {
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, Clone, Debug, PartialEq, test_strategy::Arbitrary)]
    pub struct List;
}

impl Default for ListRepresentation {
    fn default() -> Self {
        Self::List(list_representation::List)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
#[derive(test_strategy::Arbitrary)]
pub struct TypeLink {
    #[strategy("[A-Z][a-z0-9_]*")]
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
#[derive(test_strategy::Arbitrary)]
pub struct TypeUnion {
    pub representation: UnionRepresentation,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "lowercase")]
#[derive(test_strategy::Arbitrary)]
pub enum UnionRepresentation {
    Kinded(union_representation::Kinded),
    Keyed(union_representation::Keyed),
    Envelope(union_representation::Envelope),
    Inline(union_representation::Inline),
    BytePrefix(union_representation::BytePrefix),
}

mod union_representation {
    use super::{Map, RepresentationKind, TypeName, DEFAULT_SIZE_RANGE};
    use proptest::{collection::btree_map, prelude::any};
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, Clone, Debug, PartialEq, test_strategy::Arbitrary)]
    pub struct Kinded(pub Map<RepresentationKind, TypeName>);

    #[derive(Serialize, Deserialize, Clone, Debug, PartialEq, test_strategy::Arbitrary)]
    pub struct Keyed(
        #[strategy(btree_map("[^\"]*", any::<TypeName>(), DEFAULT_SIZE_RANGE))]
        pub  Map<String, TypeName>,
    );

    #[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
    #[serde(rename_all = "camelCase")]
    #[derive(test_strategy::Arbitrary)]
    pub struct Envelope {
        #[strategy("[^\"]*")]
        pub discriminant_key: String,

        #[strategy("[^\"]*")]
        pub content_key: String,

        #[strategy(btree_map("[^\"]*", any::<TypeName>(), DEFAULT_SIZE_RANGE))]
        pub discriminant_table: Map<String, TypeName>,
    }

    #[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
    #[serde(rename_all = "camelCase")]
    #[derive(test_strategy::Arbitrary)]
    pub struct Inline {
        #[strategy("[^\"]*")]
        pub discriminant_key: String,

        #[strategy(btree_map("[^\"]*", any::<TypeName>(), DEFAULT_SIZE_RANGE))]
        pub discriminant_table: Map<String, TypeName>,
    }

    #[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
    #[serde(rename_all = "camelCase")]
    #[derive(test_strategy::Arbitrary)]
    pub struct BytePrefix {
        #[strategy(btree_map(any::<TypeName>(), any::<u8>(), DEFAULT_SIZE_RANGE))]
        pub discriminant_table: Map<TypeName, u8>,
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
#[derive(test_strategy::Arbitrary)]
pub struct TypeStruct {
    // TODO: increase size range
    #[strategy(btree_map(any::<FieldName>(), any::<StructField>(), DEFAULT_SIZE_RANGE))]
    pub fields: Map<FieldName, StructField>,
    pub representation: StructRepresentation,
}

#[derive(
    Serialize, Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, test_strategy::Arbitrary,
)]
pub struct FieldName(#[strategy("[a-zA-Z0-9_]+")] String);

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
#[derive(test_strategy::Arbitrary)]
pub struct StructField {
    pub r#type: TypeTerm,

    #[serde(default, skip_serializing_if = "is_default")]
    pub optional: bool,

    #[serde(default, skip_serializing_if = "is_default")]
    pub nullable: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
#[derive(test_strategy::Arbitrary)]
// TODO: allow all variants; may require proptest's prop_recursive strategy
pub enum TypeTerm {
    TypeName(TypeName),
    #[weight(0)]
    InlineDefn(Box<InlineDefn>),
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "kind", rename_all = "lowercase")]
#[derive(test_strategy::Arbitrary)]
pub enum InlineDefn {
    Map(TypeMap),
    List(TypeList),
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "lowercase")]
#[derive(test_strategy::Arbitrary)]
// TODO: generate all variants
// TODO: all FieldNames generated here should correspond to one of TypeStruct's fields
pub enum StructRepresentation {
    Map(struct_representation::Map),
    Tuple(struct_representation::Tuple),
    StringPairs(struct_representation::StringPairs),

    // can't handle this variant until field order matches set of fields
    #[weight(0)]
    StringJoin(struct_representation::StringJoin),
    ListPairs(struct_representation::ListPairs),
}

pub mod struct_representation {
    use super::{AnyScalar, FieldName};
    use serde::{Deserialize, Serialize};

    use super::DEFAULT_SIZE_RANGE;
    use proptest::{collection::btree_map, prelude::any};

    #[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
    #[serde(rename_all = "camelCase")]
    #[derive(test_strategy::Arbitrary)]
    pub struct Map {
        #[serde(default)]
        #[strategy(btree_map(any::<FieldName>(), any::<MapFieldDetails>(), DEFAULT_SIZE_RANGE))]
        pub fields: super::Map<FieldName, MapFieldDetails>,
    }

    #[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
    #[serde(rename_all = "camelCase")]
    #[derive(test_strategy::Arbitrary)]
    pub struct MapFieldDetails {
        pub rename: Option<String>,
        pub implicit: Option<AnyScalar>,
    }

    #[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
    #[serde(rename_all = "camelCase")]
    #[derive(test_strategy::Arbitrary)]
    pub struct Tuple {
        // TODO: remove Option
        pub field_order: Option<Vec<FieldName>>,
    }

    #[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
    #[serde(rename_all = "camelCase")]
    #[derive(test_strategy::Arbitrary)]
    pub struct StringPairs {
        #[strategy("[^\"]+")]
        pub inner_delim: String,

        #[strategy("[^\"]+")]
        pub entry_delim: String,
    }

    #[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
    #[serde(rename_all = "camelCase")]
    #[derive(test_strategy::Arbitrary)]
    pub struct StringJoin {
        #[strategy("[^\"]+")]
        pub join: String,

        pub field_order: Vec<FieldName>,
    }

    #[derive(Serialize, Deserialize, Clone, Debug, PartialEq, test_strategy::Arbitrary)]
    pub struct ListPairs;
}

impl Default for StructRepresentation {
    fn default() -> Self {
        Self::Map(struct_representation::Map::default())
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, test_strategy::Arbitrary)]
pub struct TypeEnum {
    #[strategy(btree_map(any::<EnumValue>(), any::<Null>(), DEFAULT_SIZE_RANGE))]
    pub members: Map<EnumValue, Null>,
    pub representation: EnumRepresentation,
}

#[derive(
    Serialize, Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, test_strategy::Arbitrary,
)]
pub struct EnumValue(#[strategy("[a-z0-9_]+")] String);

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "lowercase")]
#[derive(test_strategy::Arbitrary)]
pub enum EnumRepresentation {
    String(enum_representation::String),
    #[weight(0)]
    Int(enum_representation::Int),
}

mod enum_representation {
    use super::{EnumValue, Map, DEFAULT_SIZE_RANGE};
    use proptest::{collection::btree_map, prelude::*};
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default, test_strategy::Arbitrary)]
    pub struct String(
        #[strategy(btree_map(any::<EnumValue>(), "[^\"]*", DEFAULT_SIZE_RANGE))]
        pub  Map<EnumValue, std::string::String>,
    );

    #[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default, test_strategy::Arbitrary)]
    pub struct Int(
        #[strategy(btree_map(any::<EnumValue>(), any::<Int>(), DEFAULT_SIZE_RANGE))]
        Map<EnumValue, Int>,
    );
}

impl Default for EnumRepresentation {
    fn default() -> Self {
        Self::String(enum_representation::String::default())
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, test_strategy::Arbitrary)]
pub struct TypeCopy {
    from_type: TypeName,
}

const L_BOOL: &str = "bool";
const L_STRING: &str = "string";
const L_BYTES: &str = "bytes";
const L_INT: &str = "int";
const L_FLOAT: &str = "float";
const L_MAP: &str = "map";
const L_LIST: &str = "list";
const L_LINK: &str = "link";
const L_UNION: &str = "union";
const L_STRUCT: &str = "struct";
const L_ENUM: &str = "enum";

const L_TYPE: &str = "type";
const L_OPTIONAL: &str = "optional";
const L_NULLABLE: &str = "nullable";
const L_LINK_REF: &str = "&";
const L_COPY: &str = "=";
const L_REPRESENTATION: &str = "representation";
const L_KINDED: &str = "kinded";
const L_KEYED: &str = "keyed";
const L_ENVELOPE: &str = "envelope";
const L_INLINE: &str = "inline";
const L_TUPLE: &str = "tuple";
const L_STRINGPAIRS: &str = "stringpairs";
const L_STRINGJOIN: &str = "stringjoin";
const L_LISTPAIRS: &str = "listpairs";
const L_DISCRIMINANT_KEY: &str = "discriminantKey";
const L_CONTENT_KEY: &str = "contentKey";
const L_IMPLICIT: &str = "implicit";
const L_BYTEPREFIX: &str = "byteprefix";

impl fmt::Display for TypeName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{}", self.0)
    }
}

impl fmt::Display for SchemaMap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        for (name, ty) in &self.0 {
            write!(f, "{} {} {}\n\n", L_TYPE, name, ty)?;
        }
        Ok(())
    }
}

impl fmt::Display for Schema {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        // TODO: self.advanced
        write!(f, "{}", &self.types)
    }
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match self {
            Self::Bool(x) => write!(f, "{}", x),
            Self::String(x) => write!(f, "{}", x),
            Self::Bytes(x) => write!(f, "{}", x),
            Self::Int(x) => write!(f, "{}", x),
            Self::Float(x) => write!(f, "{}", x),
            Self::Map(x) => write!(f, "{}", x),
            Self::List(x) => write!(f, "{}", x),
            Self::Link(x) => write!(f, "{}", x),
            Self::Union(x) => write!(f, "{}", x),
            Self::Struct(x) => write!(f, "{}", x),
            Self::Enum(x) => write!(f, "{}", x),
            Self::Copy(x) => write!(f, "{}", x),
        }
    }
}

/*
impl fmt::Display for TypeKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match self {
            Self::Bool => write!(f, "{}", L_BOOL),
            Self::String => write!(f, "{}", L_STRING),
            Self::Bytes => write!(f, "{}", L_BYTES),
            Self::Int => write!(f, "{}", L_INT),
            Self::Float => write!(f, "{}", L_FLOAT),
            Self::Map => write!(f, "{}", L_MAP),
            Self::List => write!(f, "{}", L_LIST),
            Self::Link => write!(f, "{}", L_LINK),
            Self::Union => write!(f, "{}", L_UNION),
            Self::Struct => write!(f, "{}", L_STRUCT),
            Self::Enum => write!(f, "{}", L_ENUM),
        }
    }
}
*/

impl fmt::Display for RepresentationKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match self {
            Self::Bool => write!(f, "{}", L_BOOL),
            Self::String => write!(f, "{}", L_STRING),
            Self::Bytes => write!(f, "{}", L_BYTES),
            Self::Int => write!(f, "{}", L_INT),
            Self::Float => write!(f, "{}", L_FLOAT),
            Self::Map => write!(f, "{}", L_MAP),
            Self::List => write!(f, "{}", L_LIST),
            Self::Link => write!(f, "{}", L_LINK),
        }
    }
}

impl fmt::Display for AnyScalar {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match self {
            Self::Bool(x) => write!(f, "\"{}\"", x),
            Self::String(x) => write!(f, "\"{}\"", x),
            Self::Bytes(_x) => todo!("literal bytes"), // write!(f, "{}", x),
            Self::Int(x) => write!(f, "{}", x),
            Self::Float(x) => write!(f, "{}", x),
        }
    }
}

impl fmt::Display for TypeBool {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{}", L_BOOL)
    }
}

impl fmt::Display for TypeString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{}", L_STRING)
    }
}

impl fmt::Display for TypeBytes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{}", self.representation)
    }
}

impl fmt::Display for BytesRepresentation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match self {
            Self::Bytes(b) => write!(f, "{}", b),
            Self::Advanced(_name) => todo!("advanced layout for bytes"),
        }
    }
}

impl fmt::Display for bytes_representation::Bytes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{}", L_BYTES)
    }
}

impl fmt::Display for TypeInt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{}", L_INT)
    }
}

impl fmt::Display for TypeFloat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{}", L_FLOAT)
    }
}

impl fmt::Display for TypeMap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match &self.representation {
            MapRepresentation::Map(_) => {
                if self.value_nullable {
                    write!(f, "{} ", L_NULLABLE)?;
                }
                write!(f, "{{{}:{}}}", self.key_type, self.value_type)
            }
            MapRepresentation::StringPairs(sp) => {
                if self.value_nullable {
                    write!(f, "{} ", L_NULLABLE)?;
                }
                writeln!(
                    f,
                    "{{{}:{}}} {} {} {{",
                    self.key_type, self.value_type, L_REPRESENTATION, L_STRINGPAIRS
                )?;
                writeln!(f, "  innerDelim \"{}\"", sp.inner_delim)?;
                writeln!(f, "  entryDelim \"{}\"", sp.entry_delim)?;
                writeln!(f, "}}")
            }
            MapRepresentation::ListPairs(_) => {
                if self.value_nullable {
                    write!(f, "{} ", L_NULLABLE)?;
                }
                writeln!(
                    f,
                    "{{{}:{}}} representation listpairs",
                    self.key_type, self.value_type
                )
            }
            MapRepresentation::Advanced(_) => todo!(),
        }
    }
}

impl fmt::Display for TypeList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        if self.value_nullable {
            write!(f, "{} ", L_NULLABLE)?;
        }
        write!(f, "[{}]", self.value_type)

        // TODO: handle self.representation
    }
}

impl fmt::Display for TypeLink {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{}{}", L_LINK_REF, self.expected_type)
    }
}

impl fmt::Display for TypeUnion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{}", self.representation)
    }
}

impl fmt::Display for UnionRepresentation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match self {
            Self::Kinded(x) => write!(f, "{}", x),
            Self::Keyed(x) => write!(f, "{}", x),
            Self::Envelope(x) => write!(f, "{}", x),
            Self::Inline(x) => write!(f, "{}", x),
            Self::BytePrefix(x) => write!(f, "{}", x),
        }
    }
}

impl fmt::Display for union_representation::Kinded {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{} {{", L_UNION)?;
        for (kind, name) in &self.0 {
            write!(f, "\n  | {} {}", name, kind)?;
        }
        if !self.0.is_empty() {
            writeln!(f)?;
        }
        write!(f, "}} {} {}", L_REPRESENTATION, L_KINDED)
    }
}

impl fmt::Display for union_representation::Keyed {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{} {{", L_UNION)?;
        for (ty, name) in &self.0 {
            write!(f, "\n  | {} \"{}\"", name, ty)?;
        }
        if !self.0.is_empty() {
            writeln!(f)?;
        }
        write!(f, "}} {} {}", L_REPRESENTATION, L_KEYED)
    }
}

impl fmt::Display for union_representation::Envelope {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{} {{", L_UNION)?;
        for (ty, name) in &self.discriminant_table {
            write!(f, "\n  | {} \"{}\"", name, ty)?;
        }
        if !self.discriminant_table.is_empty() {
            writeln!(f)?;
        }
        writeln!(f, "}} {} {} {{", L_REPRESENTATION, L_ENVELOPE)?;
        writeln!(f, "  {} \"{}\"", L_DISCRIMINANT_KEY, self.discriminant_key)?;
        writeln!(f, "  {} \"{}\"", L_CONTENT_KEY, self.content_key)?;
        writeln!(f, "}}")
    }
}

impl fmt::Display for union_representation::Inline {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{} {{", L_UNION)?;
        for (ty, name) in &self.discriminant_table {
            write!(f, "\n  | {} \"{}\"", name, ty)?;
        }
        if !self.discriminant_table.is_empty() {
            writeln!(f)?;
        }
        write!(
            f,
            "}} {} {} {{\n  {} \"{}\"\n}}",
            L_REPRESENTATION, L_INLINE, L_DISCRIMINANT_KEY, self.discriminant_key
        )
    }
}

impl fmt::Display for union_representation::BytePrefix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{} {{", L_UNION)?;
        for (ty, byte) in &self.discriminant_table {
            write!(f, "\n  | {} {}", ty, byte)?;
        }
        if !self.discriminant_table.is_empty() {
            writeln!(f)?;
        }
        write!(f, "}} {} {}", L_REPRESENTATION, L_BYTEPREFIX)
    }
}

impl fmt::Display for TypeStruct {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match &self.representation {
            StructRepresentation::Map(m) => {
                write!(f, "{} {{", L_STRUCT)?;
                for (name, val) in &self.fields {
                    write!(f, "\n  {} {}", name, val)?;
                    if let Some(details) = m.fields.get(name) {
                        write!(f, " (")?;
                        if let Some(implicit) = &details.implicit {
                            write!(f, "{} {}", L_IMPLICIT, implicit)?;
                        }
                        write!(f, ")")?;
                    }
                }
                if !self.fields.is_empty() {
                    writeln!(f)?;
                }
                write!(f, "}}")
            }
            StructRepresentation::Tuple(t) => {
                write!(f, "{} {{", L_STRUCT)?;
                for (name, val) in &self.fields {
                    write!(f, "\n  {} {}", name, val)?;
                }
                if !self.fields.is_empty() {
                    writeln!(f)?;
                }
                write!(f, "}} {} {}", L_REPRESENTATION, L_TUPLE)?;
                if let Some(field_order) = &t.field_order {
                    writeln!(f, " {{")?;
                    writeln!(
                        f,
                        "  fieldOrder [{}]",
                        field_order
                            .iter()
                            .map(|f| format!("\"{}\"", f))
                            .collect::<Vec<_>>()
                            .join(", ")
                    )?;
                    writeln!(f, "}}")?;
                }
                Ok(())
            }
            StructRepresentation::StringPairs(sp) => {
                write!(f, "{} {{", L_STRUCT)?;
                for (name, val) in &self.fields {
                    write!(f, "\n  {} {}", name, val)?;
                }
                if !self.fields.is_empty() {
                    writeln!(f)?;
                }
                writeln!(f, "}} {} {} {{", L_REPRESENTATION, L_STRINGPAIRS)?;
                writeln!(f, "  innerDelim \"{}\"", sp.inner_delim)?;
                writeln!(f, "  entryDelim \"{}\"", sp.entry_delim)?;
                writeln!(f, "}}")
            }
            StructRepresentation::StringJoin(sj) => {
                write!(f, "{} {{", L_STRUCT)?;
                for (name, val) in &self.fields {
                    write!(f, "\n  {} {}", name, val)?;
                }
                if !self.fields.is_empty() {
                    writeln!(f)?;
                }
                writeln!(f, "}} {} {} {{", L_REPRESENTATION, L_STRINGJOIN)?;
                writeln!(f, "  join \"{}\"", sj.join)?;
                writeln!(f, "}}")
            }
            StructRepresentation::ListPairs(_) => {
                write!(f, "{} {{", L_STRUCT)?;
                for (name, val) in &self.fields {
                    write!(f, "\n  {} {}", name, val)?;
                }
                if !self.fields.is_empty() {
                    writeln!(f)?;
                }
                write!(f, "}} {} {}", L_REPRESENTATION, L_LISTPAIRS)
            }
        }
    }
}

impl fmt::Display for FieldName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{}", self.0)
    }
}

impl fmt::Display for StructField {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        if self.optional {
            write!(f, "{} ", L_OPTIONAL)?;
        }
        if self.nullable {
            write!(f, "{} ", L_NULLABLE)?;
        }
        write!(f, "{}", self.r#type)
    }
}

impl fmt::Display for TypeTerm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match self {
            Self::TypeName(name) => write!(f, "{}", name),
            Self::InlineDefn(inline) => write!(f, "{}", inline),
        }
    }
}

impl fmt::Display for InlineDefn {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match self {
            Self::Map(map) => write!(f, "{}", map),
            Self::List(list) => write!(f, "{}", list),
        }
    }
}

impl fmt::Display for TypeEnum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{} {{", L_ENUM)?;
        #[allow(clippy::for_kv_map)]
        for (value, _null) in &self.members {
            write!(f, "\n  | {}", value)?;
        }
        if !self.members.is_empty() {
            writeln!(f)?;
        }
        write!(f, "}}")

        // TODO: handle self.representation
    }
}

impl fmt::Display for EnumValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{}", self.0)
    }
}

impl fmt::Display for TypeCopy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{} {}", L_COPY, self.from_type)
    }
}

peg::parser! {
    pub grammar schema_dsl() for str {
        rule _eof() -> () = ![_] { }
        rule _eol() -> () = "\n" / "\r\n" { }
        rule _ws1() -> () = " " / "\t" { }

        rule _comment() -> String = _ws1()* "#" s:$((!_eol() [_])*) _eol() { s.to_string() }
        rule _empty_line() -> () = _ws1()* _eol() { }
        rule _ws_block() -> () = (_comment() / _empty_line())* _ws1()* { }

        pub(crate) rule type_name() -> TypeName = cs:$(['A'..='Z'] (['A'..='Z'] / ['a'..='z'] / ['0'..='9'] / "_")*) { TypeName(cs.to_string()) }

        rule schema_map() -> SchemaMap = _ws_block() decls:(type_decl() ** _ws_block()) _ws_block() _eof() { SchemaMap(decls.into_iter().collect()) }

        // TODO: `advanced`
        pub rule parse() -> Schema = types:schema_map() { Schema { types, advanced: AdvancedDataLayoutMap::default() } }

        rule m_map() -> TypeMap
            = nil:("nullable" _ws1()+)? "{" _ws1()* n:type_name() _ws1()* ":" _ws1()* t:type_term() "}"
        {
            TypeMap {
                key_type: n,
                value_type: t,
                value_nullable: nil.is_some(),
                representation: MapRepresentation::default()
            }
        }
        rule m_stringpairs() -> TypeMap
            = nil:("nullable" _ws1()+)? "{" _ws1()* n:type_name() _ws1()* ":" _ws1()* t:type_term() "}"
              // TODO: support either field order
              _ws1()* "representation" _ws1()+ "stringpairs" _ws1()* "{" _eol() _ws1()* "innerDelim" _ws1()+ id:string() _eol() _ws1()* "entryDelim" _ws1()+ ed:string() _ws_block() "}"
        {
            TypeMap {
                key_type: n,
                value_type: t,
                value_nullable: nil.is_some(),
                representation: MapRepresentation::StringPairs(map_representation::StringPairs { inner_delim: id, entry_delim: ed })
            }
        }
        rule m_listpairs() -> TypeMap
            = nil:("nullable" _ws1()+)? "{" _ws1()* n:type_name() _ws1()* ":" _ws1()* t:type_term() "}" _ws1()* "representation" _ws1()+ "listpairs"
        {
            TypeMap {
                key_type: n,
                value_type: t,
                value_nullable: nil.is_some(),
                representation: MapRepresentation::ListPairs(map_representation::ListPairs)
            }
        }
        rule type_map() -> TypeMap = m:(
            m_stringpairs() /
            m_listpairs() /
            m_map()
        )

        // TODO: nullable and non-default representation
        rule type_list() -> TypeList = nil:("nullable" _ws1()+)?  "[" _ws1()* t:type_term() _ws1()* "]" { TypeList { value_type: t, value_nullable: nil.is_some(), representation: ListRepresentation::default()} }

        rule t_bool() -> Type = "bool" { Type::Bool(TypeBool) }
        rule t_string() -> Type = "string" { Type::String(TypeString) }
        rule t_bytes() -> Type = "bytes" { Type::Bytes(TypeBytes { representation: BytesRepresentation::default() }) }
        rule t_int() -> Type = "int" { Type::Int(TypeInt) }
        rule t_float() -> Type = "float" { Type::Float(TypeFloat) }
        rule t_map() -> Type = m:type_map() { Type::Map(m) }
        rule t_list() -> Type = l:type_list() { Type::List(l) }
        rule t_link() -> Type = "&" t:type_name() { Type::Link(TypeLink { expected_type: t.to_string() }) }
        rule t_union() -> Type = r:union_representation() { Type::Union(TypeUnion { representation: r }) }
        rule t_struct() -> Type = s:struct_model() { Type::Struct(s) }
        rule t_enum() -> Type = "enum" _ws1()* "{" _ws_block() ms:(enum_member()*) _ws_block() "}" { Type::Enum(TypeEnum { members: ms.into_iter().map(|m| (m, Null)).collect(), representation: EnumRepresentation::default() }) }
        rule t_copy() -> Type = "=" _ws1()* n:type_name() { Type::Copy(TypeCopy { from_type: n }) }
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
            ur_envelope() /
            ur_inline() /
            ur_byteprefix()
        ) { ur }
        rule ur_kinded() -> UnionRepresentation = "union" _ws1()* "{" _ws_block() ts:(type_name_and_representation_kind()*) _ws1()* "}" _ws1()* "representation" _ws1()+ "kinded" _ws1()* (_eol() / _eof()) { UnionRepresentation::Kinded(union_representation::Kinded(ts.into_iter().map(|(tn, rk)| (rk, tn)).collect())) }
        rule ur_keyed() -> UnionRepresentation = "union" _ws1()* "{" _ws_block() ts:(type_name_and_string()*) _ws1()* "}" _ws1()* "representation" _ws1()+ "keyed" _ws1()* (_eol() / _eof())  { UnionRepresentation::Keyed(union_representation::Keyed(ts.into_iter().map(|(tn, s)| (s, tn)).collect())) }
        rule ur_envelope() -> UnionRepresentation = "union" _ws1()* "{" _ws_block() ts:(type_name_and_string()*) _ws1()* "}" _ws1()* "representation" _ws1()+ "envelope" _ws1()* "{" _ws_block() "discriminantKey" _ws1()+ dk:string() _ws_block() "contentKey" _ws1()+ ck:string() _ws_block() "}" (_eol() / _eof())  { UnionRepresentation::Envelope(union_representation::Envelope { discriminant_table: ts.into_iter().map(|(tn, s)| (s, tn)).collect(), discriminant_key: dk, content_key: ck }) }
        rule ur_inline() -> UnionRepresentation = "union" _ws1()* "{" _ws_block() ts:(type_name_and_string()*) _ws1()* "}" _ws1()* "representation" _ws1()+ "inline" _ws1()* "{" _ws_block() "discriminantKey" _ws1()+ k:string() _ws_block() "}" (_eol() / _eof())  { UnionRepresentation::Inline(union_representation::Inline { discriminant_key: k, discriminant_table: ts.into_iter().map(|(tn, s)| (s, tn)).collect() }) }
        rule ur_byteprefix() -> UnionRepresentation = "union" _ws1()* "{" _ws_block() ts:(type_name_and_byte()*) _ws1()* "}" _ws1()* "representation" _ws1()+ "byteprefix" (_eol() / _eof())  { UnionRepresentation::BytePrefix(union_representation::BytePrefix { discriminant_table: ts.into_iter().collect() }) }

        rule type_name_and_string() -> (TypeName, String) = _ws1()* "|" _ws1()* t:type_name() _ws1()+ s:string() _ws1()* _eol() { (t, s) }
        rule string() -> String = "\"" cs:$((!"\"" [_])*) "\"" { cs.to_string() }

        rule type_name_and_byte() -> (TypeName, u8) = _ws1()* "|" _ws1()* s:type_name() _ws1()+ b:$(['0'..='9']+) _ws1()* _eol() { (s, b.parse().unwrap()) }

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
        rule quoted_field_name() -> FieldName = "\"" f:field_name() "\"" { f }
        // TODO: support different ordering of optional and nullable
        rule struct_field() -> StructField = o:("optional" _ws1()+)? n:("nullable" _ws1()+)? t:type_term() { StructField { r#type: t, optional: o.is_some(), nullable: n.is_some() } }

        rule tt_type_name() -> TypeTerm = n:type_name() { TypeTerm::TypeName(n) }
        rule id_map() -> InlineDefn = m:type_map() { InlineDefn::Map(m) }
        rule id_list() -> InlineDefn = l:type_list() { InlineDefn::List(l) }
        rule tt_inline_defn() -> TypeTerm = i:(id_map() / id_list()) { TypeTerm::InlineDefn(Box::new(i)) }
        pub rule type_term() -> TypeTerm = tt:(tt_type_name() / tt_inline_defn()) { tt }

        rule st_map() -> TypeStruct = "struct" _ws1()* "{" _ws_block() fs:(st_map_field()*) _ws1()* "}" {
            let fields = fs.iter().cloned().map(|(f, s, _)| (f, s)).collect();
            let representation = StructRepresentation::Map(struct_representation::Map {
                fields: fs.into_iter().filter_map(|(f, _, x)| x.map(|x| (f, x))).collect()
            });

            TypeStruct { fields, representation }
        }
        rule st_map_field() -> (FieldName, StructField, Option<struct_representation::MapFieldDetails>) = _ws1()* n:field_name() _ws1()+ f:struct_field() x:st_map_field_details()? _ws1()* _eol() { (n, f, x) }
        rule st_map_field_details() -> struct_representation::MapFieldDetails = _ws1()* "(" _ws1()* "implicit" _ws1()+ i:any_scalar()? _ws1()* ")" { struct_representation::MapFieldDetails { implicit: i, rename: None } }

        rule st_tuple() -> TypeStruct
            = "struct" _ws1()* "{" _ws_block() fs:(st_map_field()*) _ws1()* "}"
            _ws1()* "representation" _ws1()+ "tuple" _ws1()* o:st_field_order()?
        {
            let fields = fs.iter().cloned().map(|(f, s, _)| (f, s)).collect();
            let representation = StructRepresentation::Tuple(struct_representation::Tuple {
                field_order: o,
            });

            TypeStruct { fields, representation }
        }
        rule st_field_order() -> Vec<FieldName> = "{" _ws_block() "fieldOrder" _ws1()+ "[" fs:(quoted_field_name() ** ("," _ws_block())) "]" _ws_block() "}" { fs }

        rule st_stringpairs() -> TypeStruct
            = "struct" _ws1()* "{" _ws_block() fs:(st_map_field()*) _ws1()* "}"
              // TODO: support either field ordering
              _ws1()* "representation" _ws1()+ "stringpairs" _ws1()* "{" _eol() _ws1()* "innerDelim" _ws1()+ id:string() _eol() _ws1()* "entryDelim" _ws1()+ ed:string() _ws_block() "}"
        {
            TypeStruct {
                fields: fs.iter().cloned().map(|(f, s, _)| (f, s)).collect(),
                representation: StructRepresentation::StringPairs(struct_representation::StringPairs {
                    inner_delim: id,
                    entry_delim: ed,
                }),
            }
        }

        rule st_stringjoin() -> TypeStruct
        = "struct" _ws1()* "{" _ws_block() fs:(st_map_field()*) _ws1()* "}"
          // TODO: support either field ordering
          _ws1()* "representation" _ws1()+ "stringjoin" _ws1()* "{" _eol() _ws1()* "join" _ws1()+ j:string() _ws_block() "}"
        {
            TypeStruct {
                fields: fs.iter().cloned().map(|(f, s, _)| (f, s)).collect(),
                representation: StructRepresentation::StringJoin(struct_representation::StringJoin {
                    join: j,
                    field_order: fs.into_iter().map(|(f, _, _)| f).collect::<Vec<_>>(),
                }),
            }
        }

        rule st_listpairs() -> TypeStruct
        = "struct" _ws1()* "{" _ws_block() fs:(st_map_field()*) _ws1()* "}" _ws1()* "representation" _ws1()+ "listpairs"
        {
            TypeStruct {
                fields: fs.iter().cloned().map(|(f, s, _)| (f, s)).collect(),
                representation: StructRepresentation::ListPairs(struct_representation::ListPairs),
            }
        }

        pub rule struct_model() -> TypeStruct = s:(
            st_tuple() /
            st_stringpairs() /
            st_stringjoin() /
            st_listpairs() /
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
    use test_strategy::proptest;

    #[cfg(feature = "fast-test")]
    const CASES: u32 = 10;
    #[cfg(not(feature = "fast-test"))]
    const CASES: u32 = 1000;

    #[cfg(feature = "fast-test")]
    const MAX_SHRINK_ITERS: u32 = 100;
    #[cfg(not(feature = "fast-test"))]
    const MAX_SHRINK_ITERS: u32 = 10000;

    fn schema_schema_json() -> String {
        read_to_string("../specs/schemas/schema-schema.ipldsch.json").unwrap()
    }

    fn schema_roundtrips_through_json(schema: &Schema) {
        assert_eq!(
            *schema,
            serde_json::from_str(&serde_json::to_string(schema).unwrap()).unwrap()
        );
    }

    fn schema_roundtrips_through_dsl(schema: &Schema) {
        let rendered = schema.to_string();

        for (n, line) in rendered.lines().enumerate() {
            eprintln!("  {:>4}  │ {}", n + 1, line);
        }

        assert_eq!(*schema, rendered.parse::<Schema>().unwrap());
    }

    #[test]
    fn snapshot_of_parsed_schema_schema() {
        assert_debug_snapshot!(Schema::schema_schema());
    }

    #[test]
    fn snapshot_of_reified_json_form_of_schema_schema() {
        with_settings!({sort_maps => true}, {
            assert_json_snapshot!(Schema::schema_schema())
        });
    }

    #[test]
    fn struct_representation_tuple_reifies_correctly() {
        schema_roundtrips_through_json(
            &schema_dsl::parse(
                r#"type StructRepresentation_Tuple struct {
            fieldOrder optional [FieldName]
        }"#,
            )
            .unwrap(),
        );
    }

    #[test]
    fn reified_form_of_schema_schema_matches_parsed_dsl_form() {
        assert_eq!(
            Schema::schema_schema(),
            serde_json::from_str(&schema_schema_json()).unwrap()
        );
    }

    #[test]
    fn schema_schema_roundtrips_through_parsing_and_display() {
        schema_roundtrips_through_dsl(&Schema::schema_schema());
    }

    #[proptest(cases = CASES, max_shrink_iters = MAX_SHRINK_ITERS)]
    fn roundtrips_through_dsl_form(schema: Schema) {
        schema_roundtrips_through_dsl(&schema);
    }

    #[proptest(cases = CASES, max_shrink_iters = MAX_SHRINK_ITERS)]
    fn roundtrips_through_json_form(schema: Schema) {
        schema_roundtrips_through_json(&schema);
    }
}
