use std::string::ParseError;
use crate::TokenOrBracketedTokens::BracketBody;
use crate::{BindingIdent, EsAngleBracketedType, EsBindingIdent, EsBracketBody, EsConstructorType, EsEntityName, EsFunctionType, EsIndexSignature, EsNormalBracketBody, EsQualifiedName, EsThisType, EsToken, EsType, EsTypeAnn, EsTypeRef, Ident, TokenOrBracketedTokens, TsConstructorType, TsEntityName, TsFnOrConstructorType, TsFnParam, TsFnType, TsIndexSignature, TsKeywordType, TsKeywordTypeKind, TsThisType, TsType, TsTypeAnn, TsTypeParam, TsTypeParamDecl, TsTypeParamInstantiation, TsTypeRef};
use swc_atoms::{js_word, JsWord};
use swc_common::DUMMY_SP;

/**
Notes to self:

This file defines some interop between the existing AST and the EsTypesAsComments AST. It allows
for the reuse of existing ts types.

This will at somepoint, need to be converted into yet another ast representing actual
petal semantics.

EsTypes being a superset here, I'll have to circle back to see whether or not it makes sense
to parse straight to it, or enrich the ast with a second pass.

An elegant method might be to define another interop file with more try_intos.
 */

#[derive(Copy, Clone, Debug)]
pub enum InteropErr {
    NoKnownConversion,
}

impl TryFrom<TsType> for EsType {
    type Error = InteropErr;

    fn try_from(t: TsType) -> Result<Self, Self::Error> {
        match t {
            TsType::TsKeywordType(t) => Ok(EsType::EsTypeReference(t.into())),
            TsType::TsThisType(t) => Ok(EsType::EsThisType(t.into())),
            TsType::TsFnOrConstructorType(t) => match t {
                TsFnOrConstructorType::TsFnType(fn_type) => {
                    Ok(EsType::EsFunctionType(fn_type.try_into()?))
                }
                TsFnOrConstructorType::TsConstructorType(cn_typed) => {
                    Ok(EsType::EsConstructorType(cn_typed.try_into()?))
                }
            },
            TsType::TsTypeRef(t) => Ok(EsType::EsTypeReference(t.try_into()?)),
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
                optional: false,
            }),
        }
    }
}

impl From<TsKeywordType> for EsToken {
    fn from(t: TsKeywordType) -> Self {
        EsToken {
            span: t.span,
            value: t.kind.into(),
        }
    }
}

impl TryFrom<TsTypeRef> for EsTypeRef {
    type Error = InteropErr;

    fn try_from(t: TsTypeRef) -> Result<Self, Self::Error> {
        Ok(EsTypeRef {
            span: t.span,
            type_name: t.type_name.into(),
            type_arguments: match t.type_params {
                Some(_) => None,
                None => None,
            },
        })
    }
}

impl From<TsTypeParam> for TokenOrBracketedTokens {
    fn from(t: TsTypeParam) -> Self {
        TokenOrBracketedTokens::Token(EsToken {
            span: t.span,
            value: t.name.to_string(),
        })
    }
}

impl TryFrom<TsTypeParamInstantiation> for EsBracketBody {
    type Error = InteropErr;

    fn try_from(t: TsTypeParamInstantiation) -> Result<Self, Self::Error> {
        let token_body = t
            .params
            .into_iter()
            .flat_map(|x| match *x {
                TsType::TsKeywordType(t) => vec![Ok(TokenOrBracketedTokens::Token(t.into()))],
                TsType::TsTypeRef(t) => {
                    let mut initial_args: Vec<Result<TokenOrBracketedTokens, InteropErr>> = vec![
                        Ok(t.type_name.into()),
                    ];
                    // This transposition is a little hairy...
                    // We're at the farthest from similarities between the two ASTs at this point

                    if let Some(tps) = t.type_params {
                        match EsBracketBody::try_from(tps) {
                            Ok(EsBracketBody::EsNormalBracketBody(n)) => {
                                initial_args.extend(n.token_body.into_iter().map(|x|Ok(x)));
                            },
                            _ => return vec![Err(InteropErr::NoKnownConversion)]
                        }
                    }

                    return initial_args;
                },
                _ => return vec![Err(InteropErr::NoKnownConversion)],
            })
            .collect::<Result<Vec<TokenOrBracketedTokens>, InteropErr>>()?;

        Ok(EsBracketBody::EsNormalBracketBody(EsNormalBracketBody {
            span: t.span,
            token_body,
        }))
    }
}

impl From<TsThisType> for EsThisType {
    fn from(t: TsThisType) -> Self {
        EsThisType { span: t.span }
    }
}

impl From<TsEntityName> for EsEntityName {
    fn from(t: TsEntityName) -> Self {
        match t {
            TsEntityName::TsQualifiedName(es) => {
                EsEntityName::EsQualifiedName(Box::new(EsQualifiedName {
                    left: es.left.into(),
                    right: es.right,
                }))
            }
            TsEntityName::Ident(i) => EsEntityName::Ident(i),
        }
    }
}

impl From<TsEntityName> for TokenOrBracketedTokens {
    fn from(t: TsEntityName) -> Self {
        match t {
            TsEntityName::TsQualifiedName(es) => TokenOrBracketedTokens::BracketBody(
                EsType::EsAngleBracketedType(EsAngleBracketedType {
                    span: DUMMY_SP,
                    body: EsBracketBody::EsNormalBracketBody(EsNormalBracketBody {
                        span: DUMMY_SP,
                        token_body: vec![es.right.into(), es.left.into()],
                    }),
                }),
            ),
            TsEntityName::Ident(i) => i.into(),
        }
    }
}

impl From<Ident> for TokenOrBracketedTokens {
    fn from(t: Ident) -> Self {
        TokenOrBracketedTokens::Token(EsToken {
            span: t.span,
            value: t.sym.to_string(),
        })
    }
}

impl TryFrom<TsFnType> for EsFunctionType {
    type Error = InteropErr;

    fn try_from(t: TsFnType) -> Result<Self, Self::Error> {
        Ok(EsFunctionType {
            span: t.span,
            type_params: None,
            params: vec![],
            return_type: Box::new((*t.type_ann.type_ann).try_into()?),
        })
    }
}

impl TryFrom<TsConstructorType> for EsConstructorType {
    type Error = InteropErr;

    fn try_from(t: TsConstructorType) -> Result<Self, Self::Error> {
        Ok(EsConstructorType {
            span: t.span,
            params: t.params,
            type_params: t.type_params.map(|x| x.into()),
            type_ann: t.type_ann.try_into()?,
            is_abstract: t.is_abstract,
        })
    }
}

impl TryFrom<TsTypeAnn> for EsTypeAnn {
    type Error = InteropErr;

    fn try_from(t: TsTypeAnn) -> Result<Self, Self::Error> {
        let es_type_ann: EsType = (*t.type_ann).try_into()?;
        Ok(Self {
            span: t.span,
            type_ann: Box::new(es_type_ann),
        })
    }
}

impl From<TsKeywordTypeKind> for String {
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
            TsKeywordTypeKind::TsIntrinsicKeyword => "intrinsic".into(),
        }
    }
}

impl From<TsKeywordTypeKind> for JsWord {
    fn from(t: TsKeywordTypeKind) -> Self {
        String::from(t).into()
    }
}

impl From<TsTypeParamDecl> for EsBracketBody {
    fn from(t: TsTypeParamDecl) -> Self {
        EsBracketBody::EsNormalBracketBody(EsNormalBracketBody {
            span: t.span,
            token_body: t
                .params
                .into_iter()
                .map(|x| TokenOrBracketedTokens::from(x))
                .collect(),
        })
    }
}

impl TryFrom<TsIndexSignature> for EsIndexSignature {
    type Error = InteropErr;

    fn try_from(t: TsIndexSignature) -> Result<Self, Self::Error> {
        // From what I can see this should only ever have one param.
        let binding_id = match &t.params[0] {
            TsFnParam::Ident(b) => b.clone(),
            _ => unreachable!()
        };
        let type_ann: Option<EsTypeAnn> = match t.type_ann {
            Some( t) => {
                Some(t.try_into()?)
            },
            None => None
        };

        Ok(EsIndexSignature {
            binding_id: binding_id.try_into()?,
            type_ann,
            readonly: false,
            is_static: false,
            span: t.span,

        })
    }
}

impl TryFrom<BindingIdent> for EsBindingIdent {
    type Error = InteropErr;

    fn try_from(t: BindingIdent) -> Result<Self, Self::Error> {
        let type_ann: Option<EsTypeAnn> = match t.type_ann {
            Some(ref t) => {
                Some(t.clone().try_into()?)
            },
            None => None
        };

        Ok(EsBindingIdent {
            span: t.span,
            id: t.id,
            type_ann
        })
    }
}