<br><br>
#### TypeArguments :
&ensp;&ensp;AngleBracketedTokens
<br><br>
#### TypeDeclaration :
&ensp;&ensp;type BindingIdentifier TypeParameters<sub>opt</sub> = Type
<br><br>
#### TypeParameters :
&ensp;&ensp;AngleBracketedTokens
<br><br>
#### Type :
&ensp;&ensp;ConditionalType

&ensp;&ensp;NonConditionalType
<br><br>
#### ConditionalType :
&ensp;&ensp;NonConditionalType [no LineTerminator here] extends NonConditionalType ? Type : Type
<br><br>
#### NonConditionalType :
&ensp;&ensp;UnionType

&ensp;&ensp;FunctionType

&ensp;&ensp;ConstructorType
<br><br>
#### UnionType :
&ensp;&ensp;|<sub>opt</sub> IntersectionType

&ensp;&ensp;UnionType | IntersectionType
<br><br>
#### IntersectionType :
&ensp;&ensp;&<sub>opt</sub> TypeOperatorType

&ensp;&ensp;IntersectionType & TypeOperatorType
<br><br>
#### TypeOperatorType :
&ensp;&ensp;readonly TypeOperatorType

&ensp;&ensp;keyof TypeOperatorType

&ensp;&ensp;unique TypeOperatorType

&ensp;&ensp;infer TypeOperatorType

&ensp;&ensp;not TypeOperatorType

&ensp;&ensp;PrimaryType
<br><br>
#### PrimaryType :
&ensp;&ensp;ParenthesizedType

&ensp;&ensp;SquareBracketedType

&ensp;&ensp;CurlyBracketedType

&ensp;&ensp;TypeReference

&ensp;&ensp;ArrayType

&ensp;&ensp;LiteralType

&ensp;&ensp;TypeQuery

&ensp;&ensp;ImportType

&ensp;&ensp;TypePredicate

&ensp;&ensp;this

&ensp;&ensp;void
<br><br>
#### ParenthesizedType :
&ensp;&ensp;ParenthesizedTokens
<br><br>
#### SquareBracketedType :
&ensp;&ensp;SquareBracketedTokens
<br><br>
#### CurlyBracketedType :
&ensp;&ensp;CurlyBracketedTokens
<br><br>
#### TypeReference :
&ensp;&ensp;TypeName [no LineTerminator here] TypeArguments<sub>opt</sub>
<br><br>
#### TypeName :
&ensp;&ensp;Identifier

&ensp;&ensp;TypeName . Identifier
<br><br>
#### ArrayType :
&ensp;&ensp;PrimaryType [no LineTerminator here] [ ]
<br><br>
#### LiteralType :
&ensp;&ensp;NumericLiteralType

&ensp;&ensp;StringLiteral

&ensp;&ensp;TemplateLiteralType

&ensp;&ensp;true

&ensp;&ensp;false

&ensp;&ensp;null
<br><br>
#### TemplateLiteralType :
&ensp;&ensp;NoSubstitutionTemplate

&ensp;&ensp;TemplateBracketedTokens
<br><br>
#### NumericLiteralType :
&ensp;&ensp;NumericLiteral

&ensp;&ensp;- [no LineTerminator here] NumericLiteral
<br><br>
####   TypeQuery :
&ensp;&ensp;  typeof [no LineTerminator here] EntityName
<br><br>
####   EntityName :
&ensp;&ensp;  IdentifierName

&ensp;&ensp;  ImportSpecifier

&ensp;&ensp;  EntityName . IdentifierName

&ensp;&ensp;  EntityName :: TypeArguments
<br><br>
####   ImportSpecifier :
&ensp;&ensp;  import [no LineTerminator here] ( ModuleSpecifier )
<br><br>
####   ImportType :
&ensp;&ensp;  ImportSpecifier

&ensp;&ensp;  ImportSpecifier . TypeName
<br><br>
####   TypePredicate :
&ensp;&ensp;  IdentifierOrThis [no LineTerminator here] is Type

&ensp;&ensp;  asserts IdentifierOrThis

&ensp;&ensp;  asserts IdentifierOrThis [no LineTerminator here] is Type
<br><br>
####   IdentifierOrThis :
&ensp;&ensp;  Identifier

&ensp;&ensp;  this
<br><br>
####   FunctionType :
&ensp;&ensp;  TypeParameters<sub>opt</sub> ParameterList => Type
<br><br>
####   ParameterList :
&ensp;&ensp;  ParenthesizedTokens
<br><br>
####   TypeAnnotation :
&ensp;&ensp;  : Type
<br><br>
####   IndexSignature :
&ensp;&ensp;  [ BindingIdentifier TypeAnnotation ] TypeAnnotation
<br><br>
####   BracketedTokens :
&ensp;&ensp;  ParenthesizedTokens

&ensp;&ensp;  SquareBracketedTokens

&ensp;&ensp;  CurlyBracketedTokens

&ensp;&ensp;  AngleBracketedTokens

&ensp;&ensp;  TemplateBracketedTokens
<br><br>
####   ParenthesizedTokens :
&ensp;&ensp;  ( TokenBody<sub>opt</sub> )
<br><br>
####   SquareBracketedTokens :
&ensp;&ensp;  [ TokenBody<sub>opt</sub> ]
<br><br>
####   CurlyBracketedTokens :
&ensp;&ensp;  { TokenBody<sub>opt</sub> }
<br><br>
####   AngleBracketedTokens :
&ensp;&ensp;  < TokenBody<sub>opt</sub> >
<br><br>
####   TemplateBracketedTokens :
&ensp;&ensp;  TemplateHead TemplateTokenBody TemplateTail
<br><br>
####   TemplateTokenBody :
&ensp;&ensp;  TokenBody

&ensp;&ensp;  TokenBody TemplateMiddle TemplateTokenBody
<br><br>
####   TokenBody :
&ensp;&ensp;  TokenOrBracketedTokens TokenBody<sub>opt</sub>
<br><br>
####   TokenOrBracketedTokens :
&ensp;&ensp;  NonBracketedToken

&ensp;&ensp;  BracketedTokens
<br><br>
####   NonBracketedToken :
&ensp;&ensp;  Token but not one of ( or ) or [ or ] or { or } or < or > or TemplateHead or TemplateMiddle or TemplateTail