use std::ops::Try;
use crate::{EsBracketBody, EsConstructorType, EsEntityName, EsFunctionType, EsNormalBracketBody, EsQualifiedName, EsThisType, EsToken, EsType, EsTypeAnn, EsTypeRef, Ident, TokenOrBracketedTokens, TsConstructorType, TsEntityName, TsFnOrConstructorType, TsFnType, TsKeywordType, TsKeywordTypeKind, TsThisType, TsType, TsTypeAnn, TsTypeParam, TsTypeParamDecl, TsTypeParamInstantiation, TsTypeRef};
use swc_atoms::{js_word, JsWord};
use swc_common::DUMMY_SP;

/**
Notes to self:

This will at somepoint, need to be parsed into yet another ast representing actual
petal semantics.

EsTypes being a superset here, I'll have to circle back to see whether or not it makes sense
to parse straight to it, or enrich the ast with a second pass.
 */

#[derive(Copy, Clone)]
pub enum InteropErr {
    NoKnownConversion
}

impl TryFrom<TsType> for EsType {
    type Error = InteropErr;

    fn try_from(t: TsType) -> Result<Self, Self::Error> {
        match t {
            TsType::TsKeywordType(t) => Ok( EsType::EsTypeReference(t.into())),
            TsType::TsThisType(t) => Ok(EsType::EsThisType(t.into())),
            TsType::TsFnOrConstructorType(t) => match t {
                TsFnOrConstructorType::TsFnType(fn_type) => Ok(EsType::EsFunctionType(fn_type.try_into()?)),
                TsFnOrConstructorType::TsConstructorType(cn_typed) => Ok(EsType::EsConstructorType(cn_typed.try_into()?))
            },
            TsType::TsTypeRef(t) => Err(InteropErr::NoKnownConversion),
            TsType::TsTypeQuery(_) => Err(InteropErr::NoKnownConversion),
            TsType::TsTypeLit(_) => Err(InteropErr::NoKnownConversion),
            TsType::TsArrayType(_) => Err(InteropErr::NoKnownConversion),
            TsType::TsTupleType(_) => Err(InteropErr::NoKnownConversion),
            TsType::TsOptionalType(_) => Err(InteropErr::NoKnownConversion),
            TsType::TsRestType(_) => Err(InteropErr::NoKnownConversion),
            TsType::TsUnionOrIntersectionType(_) => Err(InteropErr::NoKnownConversion),
            TsType::TsConditionalType(_) => Err(InteropErr::NoKnownConversion),
            TsType::TsInferType(_) => Err(InteropErr::NoKnownConversion),
            TsType::TsParenthesizedType(_) => Err(InteropErr::NoKnownConversion),
            TsType::TsTypeOperator(_) => Err(InteropErr::NoKnownConversion),
            TsType::TsIndexedAccessType(_) => Err(InteropErr::NoKnownConversion),
            TsType::TsMappedType(_) => Err(InteropErr::NoKnownConversion),
            TsType::TsLitType(_) => Err(InteropErr::NoKnownConversion),
            TsType::TsTypePredicate(_) => Err(InteropErr::NoKnownConversion),
            TsType::TsImportType(_) => Err(InteropErr::NoKnownConversion),
        }
    }
}

impl From<TsKeywordType> for EsTypeRef {
    fn from(value: TsKeywordType) -> Self {
        EsTypeRef {
            span: value.span,
            type_arguments: None,
            type_name: EsEntityName::Ident(Ident {
                span: DUMMY_SP,
                sym: value.kind.into(),
                optional: false
            })
        }
    }
}

impl TryFrom<TsTypeRef> for EsTypeRef {
    type Error = InteropErr;

    fn try_from(t: TsTypeRef) -> Result<Self, Self::Error> {
        Ok(EsTypeRef {
            span: t.span,
            type_name: t.type_name.try_into()?,
            type_arguments: t.type_params.map(|x| x.into()),
        })
    }
}

impl TryFrom<TsTypeParamInstantiation> for EsBracketBody {
    type Error = InteropErr;

    fn try_from(t: TsTypeParamInstantiation) -> Result<Self, Self::Error> {
        let token_body = t.params.into_iter().map(|x| {
            x.try_into()?
        })
            .collect::<Result<Vec<TokenOrBracketedTokens>, InteropErr>>()?;

        Ok(EsBracketBody::EsNormalBracketBody(EsNormalBracketBody {
            span: t.span,
            token_body
        }))
    }
}

impl TryFrom<EsType> for TokenOrBracketedTokens {
    type Error = InteropErr;

    fn try_from(t: EsType) -> Result<Self, Self::Error> {
        match t {
            EsType::EsConditionalType(_) => Err(InteropErr::NoKnownConversion),
            EsType::EsUnionType(_) => Err(InteropErr::NoKnownConversion),
            EsType::EsFunctionType(_) => Err(InteropErr::NoKnownConversion),
            EsType::EsConstructorType(_) => Err(InteropErr::NoKnownConversion),
            EsType::EsIntersectionType(_) => Err(InteropErr::NoKnownConversion),
            EsType::EsTypeOperatorType(_) => Err(InteropErr::NoKnownConversion),
            EsType::EsParenthesizedType(_) => Err(InteropErr::NoKnownConversion),
            EsType::EsSquareBracketedType(_) => Err(InteropErr::NoKnownConversion),
            EsType::EsCurlyBracketedType(_) => Err(InteropErr::NoKnownConversion),
            EsType::EsAngleBracketedType(_) => Err(InteropErr::NoKnownConversion),
            EsType::EsTemplateBracketedType(_) => Err(InteropErr::NoKnownConversion),
            EsType::EsTypeReference(_) => Err(InteropErr::NoKnownConversion),
            EsType::EsArrayType(_) => Err(InteropErr::NoKnownConversion),
            EsType::EsLiteralType(_) => Err(InteropErr::NoKnownConversion),
            EsType::EsTypeQuery(_) => Err(InteropErr::NoKnownConversion),
            EsType::EsImportType(_) => Err(InteropErr::NoKnownConversion),
            EsType::EsTypePredicate(_) => Err(InteropErr::NoKnownConversion),
            EsType::EsThisType(_) => Err(InteropErr::NoKnownConversion),
            EsType::EsVoidType(_) => Err(InteropErr::NoKnownConversion),
        }
    }
}


impl From<TsThisType> for EsThisType {
    fn from(t: TsThisType) -> Self {
        EsThisType {
            span: t.span
        }
    }
}

impl From<TsEntityName> for EsEntityName {

    fn from(t: TsEntityName) -> Self {
        match t {
            TsEntityName::TsQualifiedName(es) => EsEntityName::EsQualifiedName(Box::new(EsQualifiedName {
                left: es.left.into(),
                right: es.right
            })),
            TsEntityName::Ident(i) => EsEntityName::Ident(i)
        }
    }
}

impl TryFrom<TsFnType> for EsFunctionType {
    type Error = InteropErr;

    fn try_from(t: TsFnType) -> Result<Self, Self::Error> {
        Ok(EsFunctionType {
            span: t.span,
            params: t.params,
            type_params: t.type_params.map(|x| x.into()),
            type_ann: t.type_ann.try_into()?
        })
    }
}

impl TryFrom<TsConstructorType> for EsConstructorType {
    type Error = InteropErr;

    fn try_from(t: TsConstructorType) -> Result<Self, Self::Error> {
        Ok(
            EsConstructorType {
                span: t.span,
                params: t.params,
                type_params: t.type_params.map(|x| x.into()),
                type_ann: t.type_ann.try_into()?,
                is_abstract: t.is_abstract
            }
        )
    }
}

impl TryFrom<TsTypeAnn> for EsTypeAnn {
    type Error = InteropErr;

    fn try_from(t: TsTypeAnn) -> Result<Self, Self::Error> {
        let es_type_ann: EsType = (*t.type_ann).try_into()?;
        Ok(Self {
            span: t.span,
            type_ann: Box::new(es_type_ann)
        })
    }
}

impl From<TsKeywordTypeKind> for JsWord {
    fn from(t: TsKeywordTypeKind) -> Self {
        match t {
            TsKeywordTypeKind::TsAnyKeyword => "any".into(),
            TsKeywordTypeKind::TsUnknownKeyword => "unknown".into(),
            TsKeywordTypeKind::TsNumberKeyword => "number".into(),
            TsKeywordTypeKind::TsObjectKeyword => "object".into(),
            TsKeywordTypeKind::TsBooleanKeyword => "boolean".into(),
            TsKeywordTypeKind::TsBigIntKeyword => "BigInt".into(),
            TsKeywordTypeKind::TsStringKeyword => "string".into(),
            TsKeywordTypeKind::TsSymbolKeyword => "Symbol".into(),
            TsKeywordTypeKind::TsVoidKeyword => "void".into(),
            TsKeywordTypeKind::TsUndefinedKeyword => "undefined".into(),
            TsKeywordTypeKind::TsNullKeyword => "null".into(),
            TsKeywordTypeKind::TsNeverKeyword => "never".into(),
            TsKeywordTypeKind::TsIntrinsicKeyword => "intrinsic".into()
        }
    }
}

impl From<TsTypeParamDecl> for EsBracketBody {
    fn from(t: TsTypeParamDecl) -> Self {
        EsBracketBody::EsNormalBracketBody(EsNormalBracketBody {
            span: t.span,
            token_body: t.params.into_iter().map(|x| TokenOrBracketedTokens::from(x)).collect()
        })
    }
}

impl From<TsTypeParam> for TokenOrBracketedTokens {
    fn from(t: TsTypeParam) -> Self {
        TokenOrBracketedTokens::Token(EsToken {
            span: t.span,
            value: t.name.to_string()
        })
    }
}