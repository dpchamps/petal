#![allow(clippy::vec_box)]
#![allow(missing_copy_implementations)]
use std::fmt;

use is_macro::Is;
use serde::{
    de::{self, Unexpected, Visitor},
    Deserialize, Deserializer, Serialize,
};
use string_enum::StringEnum;
use swc_atoms::JsWord;
use swc_common::{ast_node, EqIgnoreSpan, Span};

use crate::{
    class::Decorator,
    expr::Expr,
    ident::Ident,
    lit::{Bool, Number, Str},
    module::ModuleItem,
    pat::{ArrayPat, AssignPat, ObjectPat, Pat, RestPat},
    BigInt, BindingIdent, TplElement,
};

#[ast_node("EsTypeAnnotation")]
#[derive(Eq, Hash, EqIgnoreSpan)]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
pub struct EsTypeAnn {
    pub span: Span,
    #[serde(rename = "typeAnnotation")]
    pub type_ann: Box<EsType>,
}

#[ast_node("EsTypeParameterDeclaration")]
#[derive(Eq, Hash, EqIgnoreSpan)]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
pub struct EsTypeParamDecl {
    pub span: Span,
    #[serde(rename = "parameters")]
    pub params: Vec<EsTypeParam>,
}

#[ast_node("EsTypeParameter")]
#[derive(Eq, Hash, EqIgnoreSpan)]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
pub struct EsTypeParam {
    pub span: Span,
    pub name: Ident,

    #[serde(default, rename = "in")]
    pub is_in: bool,

    #[serde(default, rename = "out")]
    pub is_out: bool,

    #[serde(default)]
    pub constraint: Option<Box<EsType>>,

    #[serde(default)]
    pub default: Option<Box<EsType>>,
}

#[ast_node("EsTypeParameterInstantiation")]
#[derive(Eq, Hash, EqIgnoreSpan)]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
pub struct EsTypeParamInstantiation {
    pub span: Span,
    pub params: Vec<Box<EsType>>,
}

#[ast_node("EsParameterProperty")]
#[derive(Eq, Hash, EqIgnoreSpan)]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
pub struct EsParamProp {
    pub span: Span,
    #[serde(default)]
    pub decorators: Vec<Decorator>,
    /// At least one of `accessibility` or `readonly` must be set.
    #[serde(default)]
    pub accessibility: Option<Accessibility>,
    #[serde(rename = "override")]
    pub is_override: bool,
    pub readonly: bool,
    pub param: EsParamPropParam,
}

#[ast_node]
#[derive(Eq, Hash, Is, EqIgnoreSpan)]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
pub enum EsParamPropParam {
    #[tag("Identifier")]
    Ident(BindingIdent),

    #[tag("AssignmentPattern")]
    Assign(AssignPat),
}

#[ast_node("EsQualifiedName")]
#[derive(Eq, Hash, EqIgnoreSpan)]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
pub struct EsQualifiedName {
    #[span(lo)]
    pub left: EsEntityName,
    #[span(hi)]
    pub right: Ident,
}

#[ast_node]
#[derive(Eq, Hash, Is, EqIgnoreSpan)]
#[allow(variant_size_differences)]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
pub enum EsEntityName {
    #[tag("EsQualifiedName")]
    EsQualifiedName(Box<EsQualifiedName>),

    #[tag("Identifier")]
    Ident(Ident),
}

// ================
// TypeScript type members (for type literal / interface / class)
// ================

#[ast_node]
#[derive(Eq, Hash, Is, EqIgnoreSpan)]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
pub enum EsTypeElement {
    #[tag("EsCallSignatureDeclaration")]
    EsCallSignatureDecl(EsCallSignatureDecl),

    #[tag("EsConstructSignatureDeclaration")]
    EsConstructSignatureDecl(EsConstructSignatureDecl),

    #[tag("EsPropertySignature")]
    EsPropertySignature(EsPropertySignature),

    #[tag("EsGetterSignature")]
    EsGetterSignature(EsGetterSignature),

    #[tag("EsSetterSignature")]
    EsSetterSignature(EsSetterSignature),

    #[tag("EsMethodSignature")]
    EsMethodSignature(EsMethodSignature),

    #[tag("EsIndexSignature")]
    EsIndexSignature(EsIndexSignature),
}

#[ast_node("EsCallSignatureDeclaration")]
#[derive(Eq, Hash, EqIgnoreSpan)]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
pub struct EsCallSignatureDecl {
    pub span: Span,
    pub params: Vec<EsFnParam>,
    #[serde(default, rename = "typeAnnotation")]
    pub type_ann: Option<EsTypeAnn>,
    #[serde(default)]
    pub type_params: Option<EsTypeParamDecl>,
}

#[ast_node("EsConstructSignatureDeclaration")]
#[derive(Eq, Hash, EqIgnoreSpan)]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
pub struct EsConstructSignatureDecl {
    pub span: Span,
    pub params: Vec<EsFnParam>,
    #[serde(default, rename = "typeAnnotation")]
    pub type_ann: Option<EsTypeAnn>,
    #[serde(default)]
    pub type_params: Option<EsTypeParamDecl>,
}

#[ast_node("EsPropertySignature")]
#[derive(Eq, Hash, EqIgnoreSpan)]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
pub struct EsPropertySignature {
    pub span: Span,
    pub readonly: bool,
    pub key: Box<Expr>,
    pub computed: bool,
    pub optional: bool,
    #[serde(default)]
    pub init: Option<Box<Expr>>,
    pub params: Vec<EsFnParam>,
    #[serde(default, rename = "typeAnnotation")]
    pub type_ann: Option<EsTypeAnn>,
    #[serde(default)]
    pub type_params: Option<EsTypeParamDecl>,
}

#[ast_node("EsGetterSignature")]
#[derive(Eq, Hash, EqIgnoreSpan)]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
pub struct EsGetterSignature {
    pub span: Span,
    pub readonly: bool,
    pub key: Box<Expr>,
    pub computed: bool,
    pub optional: bool,
    #[serde(default, rename = "typeAnnotation")]
    pub type_ann: Option<EsTypeAnn>,
}

#[ast_node("EsSetterSignature")]
#[derive(Eq, Hash, EqIgnoreSpan)]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
pub struct EsSetterSignature {
    pub span: Span,
    pub readonly: bool,
    pub key: Box<Expr>,
    pub computed: bool,
    pub optional: bool,
    pub param: EsFnParam,
}

#[ast_node("EsMethodSignature")]
#[derive(Eq, Hash, EqIgnoreSpan)]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
pub struct EsMethodSignature {
    pub span: Span,
    pub readonly: bool,
    pub key: Box<Expr>,
    pub computed: bool,
    pub optional: bool,
    pub params: Vec<EsFnParam>,
    #[serde(default)]
    pub type_ann: Option<EsTypeAnn>,
    #[serde(default)]
    pub type_params: Option<EsTypeParamDecl>,
}

#[ast_node("EsIndexSignature")]
#[derive(Eq, Hash, EqIgnoreSpan)]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
pub struct EsIndexSignature {
    pub params: Vec<EsFnParam>,
    #[serde(default, rename = "typeAnnotation")]
    pub type_ann: Option<EsTypeAnn>,

    pub readonly: bool,
    #[serde(rename = "static")]
    pub is_static: bool,
    pub span: Span,
}

// ================
// TypeScript types
// ================

#[ast_node(no_clone)]
#[derive(Eq, Hash, Is, EqIgnoreSpan)]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
pub enum EsType {
    #[tag("EsKeywordType")]
    EsKeywordType(EsKeywordType),

    #[tag("EsThisType")]
    EsThisType(EsThisType),

    #[tag("EsFunctionType")]
    #[tag("EsConstructorType")]
    EsFnOrConstructorType(EsFnOrConstructorType),

    #[tag("EsTypeReference")]
    EsTypeRef(EsTypeRef),

    #[tag("EsTypeQuery")]
    EsTypeQuery(EsTypeQuery),

    #[tag("EsTypeLiteral")]
    EsTypeLit(EsTypeLit),

    #[tag("EsArrayType")]
    EsArrayType(EsArrayType),

    #[tag("EsTupleType")]
    EsTupleType(EsTupleType),

    #[tag("EsOptionalType")]
    EsOptionalType(EsOptionalType),

    #[tag("EsRestType")]
    EsRestType(EsRestType),

    #[tag("EsUnionType")]
    #[tag("EsIntersectionType")]
    EsUnionOrIntersectionType(EsUnionOrIntersectionType),

    #[tag("EsConditionalType")]
    EsConditionalType(EsConditionalType),

    #[tag("EsInferType")]
    EsInferType(EsInferType),

    #[tag("EsParenthesizedType")]
    EsParenthesizedType(EsParenthesizedType),

    #[tag("EsTypeOperator")]
    EsTypeOperator(EsTypeOperator),

    #[tag("EsIndexedAccessType")]
    EsIndexedAccessType(EsIndexedAccessType),

    #[tag("EsMappedType")]
    EsMappedType(EsMappedType),

    #[tag("EsLiteralType")]
    EsLitType(EsLitType),

    #[tag("EsTypePredicate")]
    EsTypePredicate(EsTypePredicate),

    #[tag("EsImportType")]
    EsImportType(EsImportType),
}

// Implement Clone without inline to avoid multiple copies of the
// implementation.
impl Clone for EsType {
    fn clone(&self) -> Self {
        use EsType::*;
        match self {
            EsKeywordType(t) => EsKeywordType(t.clone()),
            EsThisType(t) => EsThisType(t.clone()),
            EsFnOrConstructorType(t) => EsFnOrConstructorType(t.clone()),
            EsTypeRef(t) => EsTypeRef(t.clone()),
            EsTypeQuery(t) => EsTypeQuery(t.clone()),
            EsTypeLit(t) => EsTypeLit(t.clone()),
            EsArrayType(t) => EsArrayType(t.clone()),
            EsTupleType(t) => EsTupleType(t.clone()),
            EsOptionalType(t) => EsOptionalType(t.clone()),
            EsRestType(t) => EsRestType(t.clone()),
            EsUnionOrIntersectionType(t) => EsUnionOrIntersectionType(t.clone()),
            EsConditionalType(t) => EsConditionalType(t.clone()),
            EsInferType(t) => EsInferType(t.clone()),
            EsParenthesizedType(t) => EsParenthesizedType(t.clone()),
            EsTypeOperator(t) => EsTypeOperator(t.clone()),
            EsIndexedAccessType(t) => EsIndexedAccessType(t.clone()),
            EsMappedType(t) => EsMappedType(t.clone()),
            EsLitType(t) => EsLitType(t.clone()),
            EsTypePredicate(t) => EsTypePredicate(t.clone()),
            EsImportType(t) => EsImportType(t.clone()),
        }
    }
}

#[ast_node]
#[derive(Eq, Hash, Is, EqIgnoreSpan)]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
pub enum EsFnOrConstructorType {
    #[tag("EsFunctionType")]
    EsFnType(EsFnType),
    #[tag("EsConstructorType")]
    EsConstructorType(EsConstructorType),
}

impl From<EsFnType> for EsType {
    fn from(t: EsFnType) -> Self {
        EsFnOrConstructorType::EsFnType(t).into()
    }
}

impl From<EsConstructorType> for EsType {
    fn from(t: EsConstructorType) -> Self {
        EsFnOrConstructorType::EsConstructorType(t).into()
    }
}

impl From<EsUnionType> for EsType {
    fn from(t: EsUnionType) -> Self {
        EsUnionOrIntersectionType::EsUnionType(t).into()
    }
}

impl From<EsIntersectionType> for EsType {
    fn from(t: EsIntersectionType) -> Self {
        EsUnionOrIntersectionType::EsIntersectionType(t).into()
    }
}

#[ast_node("EsKeywordType")]
#[derive(Eq, Hash, EqIgnoreSpan)]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
pub struct EsKeywordType {
    pub span: Span,
    pub kind: EsKeywordTypeKind,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, EqIgnoreSpan)]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)
)]
#[cfg_attr(
    feature = "rkyv",
    archive_attr(repr(u32), derive(bytecheck::CheckBytes))
)]
pub enum EsKeywordTypeKind {
    #[serde(rename = "any")]
    EsAnyKeyword,

    #[serde(rename = "unknown")]
    EsUnknownKeyword,

    #[serde(rename = "number")]
    EsNumberKeyword,

    #[serde(rename = "object")]
    EsObjectKeyword,

    #[serde(rename = "boolean")]
    EsBooleanKeyword,

    #[serde(rename = "bigint")]
    EsBigIntKeyword,

    #[serde(rename = "string")]
    EsStringKeyword,

    #[serde(rename = "symbol")]
    EsSymbolKeyword,

    #[serde(rename = "void")]
    EsVoidKeyword,

    #[serde(rename = "undefined")]
    EsUndefinedKeyword,

    #[serde(rename = "null")]
    EsNullKeyword,

    #[serde(rename = "never")]
    EsNeverKeyword,

    #[serde(rename = "intrinsic")]
    EsIntrinsicKeyword,
}

#[ast_node("EsThisType")]
#[derive(Copy, Eq, Hash, EqIgnoreSpan)]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
pub struct EsThisType {
    pub span: Span,
}

#[ast_node]
#[derive(Eq, Hash, Is, EqIgnoreSpan)]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
pub enum EsFnParam {
    #[tag("Identifier")]
    Ident(BindingIdent),

    #[tag("ArrayPattern")]
    Array(ArrayPat),

    #[tag("RestElement")]
    Rest(RestPat),

    #[tag("ObjectPattern")]
    Object(ObjectPat),
}

#[ast_node("EsFunctionType")]
#[derive(Eq, Hash, EqIgnoreSpan)]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
pub struct EsFnType {
    pub span: Span,
    pub params: Vec<EsFnParam>,

    #[serde(default)]
    pub type_params: Option<EsTypeParamDecl>,
    #[serde(rename = "typeAnnotation")]
    pub type_ann: EsTypeAnn,
}

#[ast_node("EsConstructorType")]
#[derive(Eq, Hash, EqIgnoreSpan)]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
pub struct EsConstructorType {
    pub span: Span,
    pub params: Vec<EsFnParam>,
    #[serde(default)]
    pub type_params: Option<EsTypeParamDecl>,
    #[serde(rename = "typeAnnotation")]
    pub type_ann: EsTypeAnn,
    pub is_abstract: bool,
}

#[ast_node("EsTypeReference")]
#[derive(Eq, Hash, EqIgnoreSpan)]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
pub struct EsTypeRef {
    pub span: Span,
    pub type_name: EsEntityName,
    #[serde(default)]
    pub type_params: Option<EsTypeParamInstantiation>,
}

#[ast_node("EsTypePredicate")]
#[derive(Eq, Hash, EqIgnoreSpan)]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
pub struct EsTypePredicate {
    pub span: Span,
    pub asserts: bool,
    pub param_name: EsThisTypeOrIdent,
    #[serde(rename = "typeAnnotation")]
    pub type_ann: Option<EsTypeAnn>,
}

#[ast_node]
#[derive(Eq, Hash, Is, EqIgnoreSpan)]
#[allow(variant_size_differences)]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
pub enum EsThisTypeOrIdent {
    #[tag("EsThisType")]
    EsThisType(EsThisType),

    #[tag("Identifier")]
    Ident(Ident),
}

/// `typeof` operator
#[ast_node("EsTypeQuery")]
#[derive(Eq, Hash, EqIgnoreSpan)]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
pub struct EsTypeQuery {
    pub span: Span,
    pub expr_name: EsTypeQueryExpr,
    #[serde(default, rename = "typeArguments")]
    pub type_args: Option<EsTypeParamInstantiation>,
}

#[ast_node]
#[derive(Eq, Hash, Is, EqIgnoreSpan)]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
pub enum EsTypeQueryExpr {
    #[tag("EsQualifiedName")]
    #[tag("Identifier")]
    EsEntityName(EsEntityName),
    #[tag("EsImportType")]
    Import(EsImportType),
}

#[ast_node("EsImportType")]
#[derive(Eq, Hash, EqIgnoreSpan)]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
pub struct EsImportType {
    pub span: Span,
    #[serde(rename = "argument")]
    pub arg: Str,
    pub qualifier: Option<EsEntityName>,
    #[serde(rename = "typeArguments")]
    pub type_args: Option<EsTypeParamInstantiation>,
}

#[ast_node("EsTypeLiteral")]
#[derive(Eq, Hash, EqIgnoreSpan)]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
pub struct EsTypeLit {
    pub span: Span,
    pub members: Vec<EsTypeElement>,
}

#[ast_node("EsArrayType")]
#[derive(Eq, Hash, EqIgnoreSpan)]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
pub struct EsArrayType {
    pub span: Span,
    pub elem_type: Box<EsType>,
}

#[ast_node("EsTupleType")]
#[derive(Eq, Hash, EqIgnoreSpan)]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
pub struct EsTupleType {
    pub span: Span,
    pub elem_types: Vec<EsTupleElement>,
}

#[ast_node("EsTupleElement")]
#[derive(Eq, Hash, EqIgnoreSpan)]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
pub struct EsTupleElement {
    pub span: Span,
    /// `Ident` or `RestPat { arg: Ident }`
    pub label: Option<Pat>,
    pub ty: EsType,
}

#[ast_node("EsOptionalType")]
#[derive(Eq, Hash, EqIgnoreSpan)]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
pub struct EsOptionalType {
    pub span: Span,
    #[serde(rename = "typeAnnotation")]
    pub type_ann: Box<EsType>,
}

#[ast_node("EsRestType")]
#[derive(Eq, Hash, EqIgnoreSpan)]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
pub struct EsRestType {
    pub span: Span,
    #[serde(rename = "typeAnnotation")]
    pub type_ann: Box<EsType>,
}

#[ast_node]
#[derive(Eq, Hash, Is, EqIgnoreSpan)]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
pub enum EsUnionOrIntersectionType {
    #[tag("EsUnionType")]
    EsUnionType(EsUnionType),

    #[tag("EsIntersectionType")]
    EsIntersectionType(EsIntersectionType),
}

#[ast_node("EsUnionType")]
#[derive(Eq, Hash, EqIgnoreSpan)]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
pub struct EsUnionType {
    pub span: Span,
    pub types: Vec<Box<EsType>>,
}

#[ast_node("EsIntersectionType")]
#[derive(Eq, Hash, EqIgnoreSpan)]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
pub struct EsIntersectionType {
    pub span: Span,
    pub types: Vec<Box<EsType>>,
}

#[ast_node("EsConditionalType")]
#[derive(Eq, Hash, EqIgnoreSpan)]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
pub struct EsConditionalType {
    pub span: Span,
    pub check_type: Box<EsType>,
    pub extends_type: Box<EsType>,
    pub true_type: Box<EsType>,
    pub false_type: Box<EsType>,
}

#[ast_node("EsInferType")]
#[derive(Eq, Hash, EqIgnoreSpan)]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
pub struct EsInferType {
    pub span: Span,
    pub type_param: EsTypeParam,
}

#[ast_node("EsParenthesizedType")]
#[derive(Eq, Hash, EqIgnoreSpan)]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
pub struct EsParenthesizedType {
    pub span: Span,
    #[serde(rename = "typeAnnotation")]
    pub type_ann: Box<EsType>,
}

#[ast_node("EsTypeOperator")]
#[derive(Eq, Hash, EqIgnoreSpan)]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
pub struct EsTypeOperator {
    pub span: Span,
    pub op: EsTypeOperatorOp,
    #[serde(rename = "typeAnnotation")]
    pub type_ann: Box<EsType>,
}

#[derive(StringEnum, Clone, Copy, PartialEq, Eq, Hash, EqIgnoreSpan)]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)
)]
#[cfg_attr(
    feature = "rkyv",
    archive_attr(repr(u32), derive(bytecheck::CheckBytes))
)]
pub enum EsTypeOperatorOp {
    /// `keyof`
    KeyOf,
    /// `unique`
    Unique,
    /// `readonly`
    ReadOnly,
}

#[ast_node("EsIndexedAccessType")]
#[derive(Eq, Hash, EqIgnoreSpan)]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
pub struct EsIndexedAccessType {
    pub span: Span,
    pub readonly: bool,
    #[serde(rename = "objectType")]
    pub obj_type: Box<EsType>,
    pub index_type: Box<EsType>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EqIgnoreSpan)]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)
)]
#[cfg_attr(
    feature = "rkyv",
    archive_attr(repr(u32), derive(bytecheck::CheckBytes))
)]
pub enum TruePlusMinus {
    True,
    Plus,
    Minus,
}

impl Serialize for TruePlusMinus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        match *self {
            TruePlusMinus::True => serializer.serialize_bool(true),
            TruePlusMinus::Plus => serializer.serialize_str("+"),
            TruePlusMinus::Minus => serializer.serialize_str("-"),
        }
    }
}

impl<'de> Deserialize<'de> for TruePlusMinus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct TruePlusMinusVisitor;

        impl<'de> Visitor<'de> for TruePlusMinusVisitor {
            type Value = TruePlusMinus;

            fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str("one of '+', '-', true")
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                match value {
                    "+" => Ok(TruePlusMinus::Plus),
                    "-" => Ok(TruePlusMinus::Minus),
                    "true" => Ok(TruePlusMinus::True),
                    _ => Err(de::Error::invalid_value(Unexpected::Str(value), &self)),
                }
            }

            fn visit_bool<E>(self, value: bool) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                if value {
                    Ok(TruePlusMinus::True)
                } else {
                    Err(de::Error::invalid_value(Unexpected::Bool(value), &self))
                }
            }
        }

        deserializer.deserialize_any(TruePlusMinusVisitor)
    }
}

#[ast_node("EsMappedType")]
#[derive(Eq, Hash, EqIgnoreSpan)]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
pub struct EsMappedType {
    pub span: Span,
    #[serde(default)]
    pub readonly: Option<TruePlusMinus>,
    pub type_param: EsTypeParam,
    #[serde(default, rename = "nameType")]
    pub name_type: Option<Box<EsType>>,
    #[serde(default)]
    pub optional: Option<TruePlusMinus>,
    #[serde(default, rename = "typeAnnotation")]
    pub type_ann: Option<Box<EsType>>,
}

#[ast_node("EsLiteralType")]
#[derive(Eq, Hash, EqIgnoreSpan)]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
pub struct EsLitType {
    pub span: Span,
    #[serde(rename = "literal")]
    pub lit: EsLit,
}

#[ast_node]
#[derive(Eq, Hash, Is, EqIgnoreSpan)]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
pub enum EsLit {
    #[tag("NumericLiteral")]
    Number(Number),

    #[tag("StringLiteral")]
    Str(Str),

    #[tag("BooleanLiteral")]
    Bool(Bool),

    #[tag("BigIntLiteral")]
    BigInt(BigInt),

    #[tag("TemplateLiteral")]
    Tpl(EsTplLitType),
}

#[ast_node("TemplateLiteral")]
#[derive(Eq, Hash, EqIgnoreSpan)]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
pub struct EsTplLitType {
    pub span: Span,

    pub types: Vec<Box<EsType>>,

    pub quasis: Vec<TplElement>,
}

// // ================
// // TypeScript declarations
// // ================

#[ast_node("EsInterfaceDeclaration")]
#[derive(Eq, Hash, EqIgnoreSpan)]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
pub struct EsInterfaceDecl {
    pub span: Span,
    pub id: Ident,
    pub declare: bool,
    #[serde(default)]
    pub type_params: Option<EsTypeParamDecl>,
    pub extends: Vec<EsExprWithTypeArgs>,
    pub body: EsInterfaceBody,
}

#[ast_node("EsInterfaceBody")]
#[derive(Eq, Hash, EqIgnoreSpan)]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
pub struct EsInterfaceBody {
    pub span: Span,
    pub body: Vec<EsTypeElement>,
}

#[ast_node("EsExpressionWithTypeArguments")]
#[derive(Eq, Hash, EqIgnoreSpan)]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
pub struct EsExprWithTypeArgs {
    pub span: Span,
    #[serde(rename = "expression")]
    pub expr: Box<Expr>,
    #[serde(default, rename = "typeArguments")]
    pub type_args: Option<EsTypeParamInstantiation>,
}

#[ast_node("EsTypeAliasDeclaration")]
#[derive(Eq, Hash, EqIgnoreSpan)]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
pub struct EsTypeAliasDecl {
    pub span: Span,
    pub declare: bool,
    pub id: Ident,
    #[serde(default)]
    pub type_params: Option<EsTypeParamDecl>,
    #[serde(rename = "typeAnnotation")]
    pub type_ann: Box<EsType>,
}

#[ast_node("EsEnumDeclaration")]
#[derive(Eq, Hash, EqIgnoreSpan)]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
pub struct EsEnumDecl {
    pub span: Span,
    pub declare: bool,
    pub is_const: bool,
    pub id: Ident,
    pub members: Vec<EsEnumMember>,
}

#[ast_node("EsEnumMember")]
#[derive(Eq, Hash, EqIgnoreSpan)]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
pub struct EsEnumMember {
    pub span: Span,
    pub id: EsEnumMemberId,
    #[serde(default)]
    pub init: Option<Box<Expr>>,
}

///
/// - Invalid: [Ident] with empty symbol.
#[ast_node]
#[derive(Eq, Hash, Is, EqIgnoreSpan)]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
pub enum EsEnumMemberId {
    #[tag("Identifier")]
    Ident(Ident),

    #[tag("StringLiteral")]
    Str(Str),
}

impl AsRef<JsWord> for EsEnumMemberId {
    fn as_ref(&self) -> &JsWord {
        match &self {
            EsEnumMemberId::Str(Str { value: ref sym, .. })
            | EsEnumMemberId::Ident(Ident { ref sym, .. }) => sym,
        }
    }
}

#[ast_node("EsModuleDeclaration")]
#[derive(Eq, Hash, EqIgnoreSpan)]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
pub struct EsModuleDecl {
    pub span: Span,
    pub declare: bool,
    /// In TypeScript, this is only available through`node.flags`.
    pub global: bool,
    pub id: EsModuleName,
    #[serde(default)]
    pub body: Option<EsNamespaceBody>,
}

/// `namespace A.B { }` is a namespace named `A` with another EsNamespaceDecl as
/// its body.
#[ast_node]
#[derive(Eq, Hash, Is, EqIgnoreSpan)]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
pub enum EsNamespaceBody {
    #[tag("EsModuleBlock")]
    EsModuleBlock(EsModuleBlock),

    #[tag("EsNamespaceDeclaration")]
    EsNamespaceDecl(EsNamespaceDecl),
}

#[ast_node("EsModuleBlock")]
#[derive(Eq, Hash, EqIgnoreSpan)]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
pub struct EsModuleBlock {
    pub span: Span,
    pub body: Vec<ModuleItem>,
}

#[ast_node("EsNamespaceDeclaration")]
#[derive(Eq, Hash, EqIgnoreSpan)]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
pub struct EsNamespaceDecl {
    pub span: Span,
    pub declare: bool,
    /// In TypeScript, this is only available through`node.flags`.
    pub global: bool,
    pub id: Ident,
    pub body: Box<EsNamespaceBody>,
}

#[ast_node]
#[derive(Eq, Hash, Is, EqIgnoreSpan)]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
pub enum EsModuleName {
    #[tag("Identifier")]
    Ident(Ident),

    #[tag("StringLiteral")]
    Str(Str),
}

#[ast_node("EsImportEqualsDeclaration")]
#[derive(Eq, Hash, EqIgnoreSpan)]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
pub struct EsImportEqualsDecl {
    pub span: Span,
    pub declare: bool,
    pub is_export: bool,
    pub is_type_only: bool,
    pub id: Ident,
    pub module_ref: EsModuleRef,
}

#[ast_node]
#[derive(Eq, Hash, Is, EqIgnoreSpan)]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
pub enum EsModuleRef {
    #[tag("EsQualifiedName")]
    #[tag("Identifier")]
    EsEntityName(EsEntityName),

    #[tag("EsExternalModuleReference")]
    EsExternalModuleRef(EsExternalModuleRef),
}

#[ast_node("EsExternalModuleReference")]
#[derive(Eq, Hash, EqIgnoreSpan)]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
pub struct EsExternalModuleRef {
    pub span: Span,
    #[serde(rename = "expression")]
    pub expr: Str,
}

/// TypeScript's own parser uses ExportAssignment for both `export default` and
/// `export =`. But for @babel/parser, `export default` is an ExportDefaultDecl,
/// so a EsExportAssignment is always `export =`.
#[ast_node("EsExportAssignment")]
#[derive(Eq, Hash, EqIgnoreSpan)]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
pub struct EsExportAssignment {
    pub span: Span,
    #[serde(rename = "expression")]
    pub expr: Box<Expr>,
}

#[ast_node("EsNamespaceExportDeclaration")]
#[derive(Eq, Hash, EqIgnoreSpan)]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
pub struct EsNamespaceExportDecl {
    pub span: Span,
    pub id: Ident,
}

// // ================
// // TypeScript exprs
// // ================

#[ast_node("EsAsExpression")]
#[derive(Eq, Hash, EqIgnoreSpan)]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
pub struct EsAsExpr {
    pub span: Span,
    #[serde(rename = "expression")]
    pub expr: Box<Expr>,
    #[serde(rename = "typeAnnotation")]
    pub type_ann: Box<EsType>,
}

#[ast_node("EsTypeAssertion")]
#[derive(Eq, Hash, EqIgnoreSpan)]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
pub struct EsTypeAssertion {
    pub span: Span,
    #[serde(rename = "expression")]
    pub expr: Box<Expr>,
    #[serde(rename = "typeAnnotation")]
    pub type_ann: Box<EsType>,
}

#[ast_node("EsNonNullExpression")]
#[derive(Eq, Hash, EqIgnoreSpan)]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
pub struct EsNonNullExpr {
    pub span: Span,
    #[serde(rename = "expression")]
    pub expr: Box<Expr>,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, Eq, Hash, EqIgnoreSpan)]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)
)]
#[cfg_attr(
    feature = "rkyv",
    archive_attr(repr(u32), derive(bytecheck::CheckBytes))
)]
pub enum Accessibility {
    #[serde(rename = "public")]
    Public,
    #[serde(rename = "protected")]
    Protected,
    #[serde(rename = "private")]
    Private,
}

#[ast_node("EsConstAssertion")]
#[derive(Eq, Hash, EqIgnoreSpan)]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
pub struct EsConstAssertion {
    pub span: Span,
    #[serde(rename = "expression")]
    pub expr: Box<Expr>,
}

#[ast_node("EsInstantiation")]
#[derive(Eq, Hash, EqIgnoreSpan)]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
pub struct EsInstantiation {
    pub span: Span,
    #[serde(rename = "expression")]
    pub expr: Box<Expr>,
    #[serde(rename = "typeArguments")]
    pub type_args: EsTypeParamInstantiation,
}
