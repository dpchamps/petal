use crate::{class::Decorator, expr::Expr, ident::Ident, lit::{Bool, Number, Str}, module::ModuleItem, pat::{ArrayPat, AssignPat, ObjectPat, Pat, RestPat}, BigInt, BindingIdent, TplElement, TsTypeRef, TsArrayType, Tpl, TsTypeQuery, TsImportType, TsTypePredicate, TsThisType, TsFnParam};


#[ast_node(no_clone)]
#[derive(Eq, Hash, Is, EqIgnoreSpan)]
pub enum EsType {
    EsConditionalType(EsConditionalType),
    EsUnionType(EsUnionType),
    EsFunctionType(EsFunctionType),
    EsConstructorType(EsConstructorType),
    EsIntersectionType(EsIntersectionType),

    EsTypeOperatorType(EsTypeOperatorType),

    /// Primary Types
    #[tag("EsParenthesizedType")]
    EsParenthesizedType(EsBracketedType),

    #[tag("EsSquareBracketedType")]
    EsSquareBracketedType(EsBracketedType),

    #[tag("EsCurlyBracketedType")]
    EsCurlyBracketedType(EsBracketedType),

    #[tag("EsTypeReference")]
    EsTypeReference(TsTypeRef),

    #[tag("EsArrayType")]
    EsArrayType(TsArrayType),

    #[tag("EsLiteralType")]
    EsLiteralType(EsLiteralType),

    #[tag("EsTypeQuery")]
    EsTypeQuery(TsTypeQuery),

    #[tag("EsImportType")]
    EsImportType(TsImportType),

    #[tag("EsTypePredicate")]
    EsTypePredicate(TsTypePredicate),

    #[tag("EsThisType")]
    EsThisType(TsThisType),

    #[tag("EsVoidType")]
    EsVoidType(EsVoidType),
}

#[ast_node("EsFunctionType")]
#[derive(Eq, Hash, EqIgnoreSpan)]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
pub struct EsFunctionType {
    pub span: Span,
    pub params: Vec<TsFnParam>,

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
    pub params: Vec<TsFnParam>,
    #[serde(default)]
    pub type_params: Option<EsTypeParamDecl>,
    #[serde(rename = "typeAnnotation")]
    pub type_ann: EsTypeAnn,
    pub is_abstract: bool,
}

#[ast_node("EsTypeAnnotation")]
#[derive(Eq, Hash, EqIgnoreSpan)]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
pub struct TsTypeAnn {
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
    pub params: Vec<EsTokenBodyEl>,
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
#[derive(Eq, Hash, Is, EqIgnoreSpan)]
pub struct EsUnionType {
    pub span: Span,
    pub op: EsTypeOperatorOp,
    pub types: Vec<Box<EsType>>,
}

#[ast_node("EsIntersectionType")]
#[derive(Eq, Hash, Is, EqIgnoreSpan)]
pub struct EsIntersectionType {
    pub span: Span,
    pub op: EsTypeOperatorOp,
    pub types: Vec<Box<EsType>>,
}

#[derive(StringEnum, Clone, Copy, PartialEq, Eq, Hash, EqIgnoreSpan)]
pub enum EsTypeOperatorOp {
    ReadOnly,
    KeyOf,
    Unique,
    infer,
    not
}

#[ast_node("EsTypeOperatorType")]
#[derive(Eq, Hash, Is, EqIgnoreSpan)]
pub struct EsTypeOperatorType {
    pub span: Span,
    pub op: EsTypeOperatorOp,
    pub es_type: Box<EsType>
}

#[ast_node("EsBracketedType")]
#[derive(Eq, Hash, EqIgnoreSpan)]
pub struct EsBracketedType {
    pub span: Span,
    pub token_body: Vec<EsTokenBodyEl>,
}

#[ast_node("EsTypeReference")]
#[derive(Eq, Hash, EqIgnoreSpan)]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
pub struct EsTypeRef {
    pub span: Span,
    pub type_name: TsEntityName,
    #[serde(default)]
    pub type_arguments: Option<EsBracketedType>,
}


#[ast_node]
#[ast_node(no_clone)]
#[derive(Eq, Hash, Is, EqIgnoreSpan)]
pub enum EsTokenBodyEl {
    #[tag("Identifier")]
    Ident(Ident),
}

#[ast_node]
#[ast_node(no_clone)]
#[derive(Eq, Hash, Is, EqIgnoreSpan)]
pub enum EsLiteralType {
    #[tag("NumbericLiteral")]
    Number(Number),

    #[tag("StringLiteral")]
    Str(Str),

    #[tag("TemplateLiteral")]
    Template(Tpl),

    #[tag("TrueLiteral")]
    True,

    #[tag("FalseLiteral")]
    False,

    #[tag("NullLiteral")]
    Null
}

#[ast_node("EsVoidType")]
#[derive(Copy, Eq, Hash, EqIgnoreSpan)]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
pub struct EsVoidType {
    pub span: Span,
}
