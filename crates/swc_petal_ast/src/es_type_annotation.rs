use crate::{
    ident::Ident,
    lit::{Bool, Number, Str},
    BindingIdent, Null, Tpl, TplElement, TsFnParam, TsTypeAnn,
};
use is_macro::Is;
use string_enum::StringEnum;
use swc_common::{ast_node, EqIgnoreSpan, Span};

#[ast_node("EsTypeAnnotation")]
#[derive(Eq, Hash, EqIgnoreSpan)]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
pub struct EsTypeAnn {
    pub span: Span,
    #[serde(rename = "typeAnnotation")]
    pub type_ann: Box<EsType>,
}

#[ast_node(no_clone)]
#[derive(Eq, Hash, Is, EqIgnoreSpan)]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
pub enum EsType {
    #[tag("EsConditionalType")]
    EsConditionalType(EsConditionalType),
    #[tag("EsUnionType")]
    EsUnionType(EsUnionType),
    #[tag("EsFunctionType")]
    EsFunctionType(EsFunctionType),
    #[tag("EsConstructorType")]
    EsConstructorType(EsConstructorType),
    #[tag("EsIntersectionType")]
    EsIntersectionType(EsIntersectionType),
    #[tag("EsTypeOperatorType")]
    EsTypeOperatorType(EsTypeOperatorType),

    /// Primary Types
    #[tag("EsParenthesizedType")]
    EsParenthesizedType(EsParenthesizedType),

    #[tag("EsSquareBracketedType")]
    EsSquareBracketedType(EsSquareBracketedType),

    #[tag("EsCurlyBracketedType")]
    EsCurlyBracketedType(EsCurlyBracketedType),

    #[tag("EsAngleBracketedType")]
    EsAngleBracketedType(EsAngleBracketedType),

    #[tag("EsTemplateBracketedType")]
    EsTemplateBracketedType(EsTemplateBracketedType),

    #[tag("EsTypeReference")]
    EsTypeReference(EsTypeRef),

    #[tag("EsArrayType")]
    EsArrayType(EsArrayType),

    #[tag("EsLiteralType")]
    EsLiteralType(EsLiteralType),

    #[tag("EsTypeQuery")]
    EsTypeQuery(EsTypeQuery),

    #[tag("EsImportType")]
    EsImportType(EsImportType),

    #[tag("EsTypePredicate")]
    EsTypePredicate(EsTypePredicate),

    #[tag("EsThisType")]
    EsThisType(EsThisType),

    #[tag("EsVoidType")]
    EsVoidType(EsVoidType),
}

// Implement Clone without inline to avoid multiple copies of the
// implementation.
impl Clone for EsType {
    fn clone(&self) -> Self {
        use EsType::*;
        match self {
            EsConditionalType(t) => EsConditionalType(t.clone()),
            EsUnionType(t) => EsUnionType(t.clone()),
            EsFunctionType(t) => EsFunctionType(t.clone()),
            EsConstructorType(t) => EsConstructorType(t.clone()),
            EsIntersectionType(t) => EsIntersectionType(t.clone()),
            EsTypeOperatorType(t) => EsTypeOperatorType(t.clone()),
            EsParenthesizedType(t) => EsParenthesizedType(t.clone()),
            EsSquareBracketedType(t) => EsSquareBracketedType(t.clone()),
            EsCurlyBracketedType(t) => EsCurlyBracketedType(t.clone()),
            EsAngleBracketedType(t) => EsAngleBracketedType(t.clone()),
            EsTemplateBracketedType(t) => EsTemplateBracketedType(t.clone()),
            EsTypeReference(t) => EsTypeReference(t.clone()),
            EsArrayType(t) => EsArrayType(t.clone()),
            EsLiteralType(t) => EsLiteralType(t.clone()),
            EsTypeQuery(t) => EsTypeQuery(t.clone()),
            EsImportType(t) => EsImportType(t.clone()),
            EsTypePredicate(t) => EsTypePredicate(t.clone()),
            EsThisType(t) => EsThisType(t.clone()),
            EsVoidType(t) => EsVoidType(t.clone()),
        }
    }
}

#[ast_node("EsFunctionType")]
#[derive(Eq, Hash, EqIgnoreSpan)]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
pub struct EsFunctionType {
    pub span: Span,
    pub params: Vec<TsFnParam>,

    #[serde(default)]
    pub type_params: Option<EsBracketBody>,
    #[serde(rename = "typeAnnotation")]
    pub type_ann: EsTypeAnn,
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

#[ast_node("EsConstructorType")]
#[derive(Eq, Hash, EqIgnoreSpan)]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
pub struct EsConstructorType {
    pub span: Span,
    pub params: Vec<TsFnParam>,
    #[serde(default)]
    pub type_params: Option<EsBracketBody>,
    #[serde(rename = "typeAnnotation")]
    pub type_ann: EsTypeAnn,
    pub is_abstract: bool,
}

#[ast_node("EsConditionalType")]
#[derive(Eq, Hash, EqIgnoreSpan)]
pub struct EsConditionalType {
    pub span: Span,
    pub check_type: Box<EsType>,
    pub extends_type: Box<EsType>,
    pub true_type: Box<EsType>,
    pub false_type: Box<EsType>,
}

#[ast_node("EsUnionType")]
#[derive(Eq, Hash, EqIgnoreSpan)]
pub struct EsUnionType {
    pub span: Span,
    pub op: EsTypeOperatorOp,
    pub types: Vec<Box<EsType>>,
}

#[ast_node("EsIntersectionType")]
#[derive(Eq, Hash, EqIgnoreSpan)]
pub struct EsIntersectionType {
    pub span: Span,
    pub op: EsTypeOperatorOp,
    pub types: Vec<Box<EsType>>,
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
    /// `readonly`
    ReadOnly,
    /// `keyof`
    KeyOf,
    /// `unique`
    Unique,
    /// `infer`
    Infer,
    /// `not`
    Not,
}

#[ast_node("EsTypeOperatorType")]
#[derive(Eq, Hash, EqIgnoreSpan)]
pub struct EsTypeOperatorType {
    pub span: Span,
    pub op: EsTypeOperatorOp,
    pub es_type: Box<EsType>,
}

// pub enum

#[ast_node("EsBracketedType")]
#[derive(Eq, Hash, EqIgnoreSpan)]
pub struct EsParenthesizedType {
    pub span: Span,
    pub body: EsBracketBody,
}

#[ast_node("EsBracketedType")]
#[derive(Eq, Hash, EqIgnoreSpan)]
pub struct EsSquareBracketedType {
    pub span: Span,
    pub body: EsBracketBody,
}

#[ast_node("EsBracketedType")]
#[derive(Eq, Hash, EqIgnoreSpan)]
pub struct EsCurlyBracketedType {
    pub span: Span,
    pub body: EsBracketBody,
}

#[ast_node("EsBracketedType")]
#[derive(Eq, Hash, EqIgnoreSpan)]
pub struct EsAngleBracketedType {
    pub span: Span,
    pub body: EsBracketBody,
}

#[ast_node("EsBracketedType")]
#[derive(Eq, Hash, EqIgnoreSpan)]
pub struct EsTemplateBracketedType {
    pub span: Span,
    pub body: EsBracketBody,
}

#[ast_node]
#[derive(Eq, Hash, EqIgnoreSpan)]
pub enum EsBracketBody {
    #[tag("EsNormalBracketBody")]
    EsNormalBracketBody(EsNormalBracketBody),
    #[tag("EsNormalTemplateBody")]
    EsTemplateBracketBody(EsTemplateBracketBody),
}

#[ast_node("EsNormalBracketBody")]
#[derive(Eq, Hash, EqIgnoreSpan)]
pub struct EsNormalBracketBody {
    pub span: Span,
    pub token_body: Vec<TokenOrBracketedTokens>,
}

#[ast_node("EsTemplateBracketBody")]
#[derive(Eq, Hash, EqIgnoreSpan)]
pub struct EsTemplateBracketBody {
    pub span: Span,
    pub exprs: Vec<TokenOrBracketedTokens>,
    pub token_body: Vec<TplElement>,
}

#[ast_node("EsTypeReference")]
#[derive(Eq, Hash, EqIgnoreSpan)]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
pub struct EsTypeRef {
    pub span: Span,
    pub type_name: EsEntityName,
    #[serde(default)]
    pub type_arguments: Option<EsBracketBody>,
}

#[ast_node("EsArrayType")]
#[derive(Eq, Hash, EqIgnoreSpan)]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
pub struct EsArrayType {
    pub span: Span,
    pub elem_type: Box<EsType>,
}

/// `typeof` operator
#[ast_node("EsTypeQuery")]
#[derive(Eq, Hash, EqIgnoreSpan)]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
pub struct EsTypeQuery {
    pub span: Span,
    pub expr_name: EsTypeQueryExpr,
    #[serde(default, rename = "typeArguments")]
    pub type_args: Option<EsBracketBody>,
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

#[ast_node("EsImportType")]
#[derive(Eq, Hash, EqIgnoreSpan)]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
pub struct EsImportType {
    pub span: Span,
    #[serde(rename = "argument")]
    pub arg: Str,
    pub qualifier: Option<EsEntityName>,
    #[serde(rename = "typeArguments")]
    pub type_args: Option<EsBracketBody>,
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
pub enum TokenOrBracketedTokens {

    #[tag("Identifier")]
    Token(EsToken),
    #[tag("String")]
    BracketBody(EsType),
}

#[ast_node("EsToken")]
#[derive(Eq, Hash, EqIgnoreSpan)]
pub struct EsToken {
    pub span: Span,
    /// TODO: For ease making this a string,
    /// however, the typesystem will define this. So either a second pass further decorates the ast,
    /// Or the specification just lives here
    pub value: String,
}

#[ast_node]
#[derive(Eq, Hash, Is, EqIgnoreSpan)]
pub enum EsLiteralType {
    #[tag("NumericLiteral")]
    Number(Number),

    #[tag("StringLiteral")]
    Str(Str),

    #[tag("TemplateLiteral")]
    Template(Tpl),

    #[tag("BoolLiteral")]
    Bool(Bool),

    #[tag("NullLiteral")]
    Null(Null),
}

#[ast_node("EsVoidType")]
#[derive(Copy, Eq, Hash, EqIgnoreSpan)]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
pub struct EsVoidType {
    pub span: Span,
}

#[ast_node("EsThisType")]
#[derive(Copy, Eq, Hash, EqIgnoreSpan)]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
pub struct EsThisType {
    pub span: Span,
}

#[ast_node("EsIndexSignature")]
#[derive(Eq, Hash, EqIgnoreSpan)]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
pub struct EsIndexSignature {
    pub binding_id: EsBindingIdent,
    #[serde(default, rename = "typeAnnotation")]
    pub type_ann: Option<EsTypeAnn>,

    pub readonly: bool,
    #[serde(rename = "static")]
    pub is_static: bool,
    pub span: Span,
}

#[ast_node("EsBindingIdent")]
#[derive(Eq, Hash, EqIgnoreSpan)]
pub struct EsBindingIdent {
    pub span: Span,
    #[serde(flatten)]
    #[cfg_attr(feature = "rkyv", omit_bounds)]
    pub id: Ident,
    #[serde(default, rename = "typeAnnotation")]
    #[cfg_attr(feature = "rkyv", omit_bounds)]
    pub type_ann: Option<EsTypeAnn>,
}