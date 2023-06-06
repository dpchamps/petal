## Expressions
**IdentifierReference[Yield, Await] :**

&ensp;&ensp;Identifier

&ensp;&ensp;[~Yield] yield

&ensp;&ensp;[~Await] await
<br><br>
**BindingIdentifier[Yield, Await] :**

&ensp;&ensp;Identifier

&ensp;&ensp;yield

&ensp;&ensp;await
<br><br>
**LabelIdentifier[Yield, Await] :**

&ensp;&ensp;Identifier

&ensp;&ensp;[~Yield] yield

&ensp;&ensp;[~Await] await
<br><br>
**Identifier :**

&ensp;&ensp;IdentifierName but not ReservedWord
<br><br>
**PrimaryExpression[Yield, Await] :**

&ensp;&ensp;this

&ensp;&ensp;IdentifierReference[?Yield, ?Await]

&ensp;&ensp;Literal

&ensp;&ensp;ArrayLiteral[?Yield, ?Await]

&ensp;&ensp;ObjectLiteral[?Yield, ?Await]

&ensp;&ensp;FunctionExpression

&ensp;&ensp;ClassExpression[?Yield, ?Await]

&ensp;&ensp;GeneratorExpression

&ensp;&ensp;AsyncFunctionExpression

&ensp;&ensp;AsyncGeneratorExpression

&ensp;&ensp;RegularExpressionLiteral

&ensp;&ensp;TemplateLiteral[?Yield, ?Await, ~Tagged]

&ensp;&ensp;CoverParenthesizedExpressionAndArrowParameterList[?Yield, ?Await]
<br><br>
**CoverParenthesizedExpressionAndArrowParameterList[Yield, Await] :**

&ensp;&ensp;( Expression[+In, ?Yield, ?Await] )

&ensp;&ensp;( Expression[+In, ?Yield, ?Await] , )

&ensp;&ensp;( )

&ensp;&ensp;( ... BindingIdentifier[?Yield, ?Await] )

&ensp;&ensp;( ... BindingPattern[?Yield, ?Await] )

&ensp;&ensp;( Expression[+In, ?Yield, ?Await] , ... BindingIdentifier[?Yield, ?Await] )

&ensp;&ensp;( Expression[+In, ?Yield, ?Await] , ... BindingPattern[?Yield, ?Await] )

&ensp;&ensp;When processing an instance of the production

&ensp;&ensp;PrimaryExpression[Yield, Await] : CoverParenthesizedExpressionAndArrowParameterList[?Yield, ?Await]
<br><br>
**the interpretation of CoverParenthesizedExpressionAndArrowParameterList is refined using the following grammar:**

&ensp;&ensp;
<br><br>
**ParenthesizedExpression[Yield, Await] :**

&ensp;&ensp;( Expression[+In, ?Yield, ?Await] )

&ensp;&ensp;

&ensp;&ensp;
<br><br>
**Literal :**

&ensp;&ensp;NullLiteral

&ensp;&ensp;BooleanLiteral

&ensp;&ensp;NumericLiteral

&ensp;&ensp;StringLiteral
<br><br>
**ArrayLiteral[Yield, Await] :**

&ensp;&ensp;[ Elisionopt ]

&ensp;&ensp;[ ElementList[?Yield, ?Await] ]

&ensp;&ensp;[ ElementList[?Yield, ?Await] , Elisionopt ]
<br><br>
**ElementList[Yield, Await] :**

&ensp;&ensp;Elisionopt AssignmentExpression[+In, ?Yield, ?Await]

&ensp;&ensp;Elisionopt SpreadElement[?Yield, ?Await]

&ensp;&ensp;ElementList[?Yield, ?Await] , Elisionopt AssignmentExpression[+In, ?Yield, ?Await]

&ensp;&ensp;ElementList[?Yield, ?Await] , Elisionopt SpreadElement[?Yield, ?Await]
<br><br>
**Elision :**

&ensp;&ensp;,

&ensp;&ensp;Elision ,
<br><br>
**SpreadElement[Yield, Await] :**

&ensp;&ensp;... AssignmentExpression[+In, ?Yield, ?Await]
<br><br>
**ObjectLiteral[Yield, Await] :**

&ensp;&ensp;{ }

&ensp;&ensp;{ PropertyDefinitionList[?Yield, ?Await] }

&ensp;&ensp;{ PropertyDefinitionList[?Yield, ?Await] , }
<br><br>
**PropertyDefinitionList[Yield, Await] :**

&ensp;&ensp;PropertyDefinition[?Yield, ?Await]

&ensp;&ensp;PropertyDefinitionList[?Yield, ?Await] , PropertyDefinition[?Yield, ?Await]
<br><br>
**PropertyDefinition[Yield, Await] :**

&ensp;&ensp;IdentifierReference[?Yield, ?Await]

&ensp;&ensp;CoverInitializedName[?Yield, ?Await]

&ensp;&ensp;PropertyName[?Yield, ?Await] : AssignmentExpression[+In, ?Yield, ?Await]

&ensp;&ensp;MethodDefinition[?Yield, ?Await]

&ensp;&ensp;... AssignmentExpression[+In, ?Yield, ?Await]
<br><br>
**PropertyName[Yield, Await] :**

&ensp;&ensp;LiteralPropertyName

&ensp;&ensp;ComputedPropertyName[?Yield, ?Await]
<br><br>
**LiteralPropertyName :**

&ensp;&ensp;IdentifierName

&ensp;&ensp;StringLiteral

&ensp;&ensp;NumericLiteral
<br><br>
**ComputedPropertyName[Yield, Await] :**

&ensp;&ensp;[ AssignmentExpression[+In, ?Yield, ?Await] ]
<br><br>
**CoverInitializedName[Yield, Await] :**

&ensp;&ensp;IdentifierReference[?Yield, ?Await] Initializer[+In, ?Yield, ?Await]
<br><br>
**Initializer[In, Yield, Await] :**

&ensp;&ensp;= AssignmentExpression[?In, ?Yield, ?Await]
<br><br>
**TemplateLiteral[Yield, Await, Tagged] :**

&ensp;&ensp;NoSubstitutionTemplate

&ensp;&ensp;SubstitutionTemplate[?Yield, ?Await, ?Tagged]
<br><br>
**SubstitutionTemplate[Yield, Await, Tagged] :**

&ensp;&ensp;TemplateHead Expression[+In, ?Yield, ?Await] TemplateSpans[?Yield, ?Await, ?Tagged]
<br><br>
**TemplateSpans[Yield, Await, Tagged] :**

&ensp;&ensp;TemplateTail

&ensp;&ensp;TemplateMiddleList[?Yield, ?Await, ?Tagged] TemplateTail
<br><br>
**TemplateMiddleList[Yield, Await, Tagged] :**

&ensp;&ensp;TemplateMiddle Expression[+In, ?Yield, ?Await]

&ensp;&ensp;TemplateMiddleList[?Yield, ?Await, ?Tagged] TemplateMiddle Expression[+In, ?Yield, ?Await]
<br><br>
**MemberExpression[Yield, Await] :**

&ensp;&ensp;PrimaryExpression[?Yield, ?Await]

&ensp;&ensp;MemberExpression[?Yield, ?Await] [ Expression[+In, ?Yield, ?Await] ]

&ensp;&ensp;MemberExpression[?Yield, ?Await] . IdentifierName

&ensp;&ensp;MemberExpression[?Yield, ?Await] TemplateLiteral[?Yield, ?Await, +Tagged]

&ensp;&ensp;SuperProperty[?Yield, ?Await]

&ensp;&ensp;MetaProperty

&ensp;&ensp;new MemberExpression[?Yield, ?Await] Arguments[?Yield, ?Await]

&ensp;&ensp;MemberExpression[?Yield, ?Await] . PrivateIdentifier
<br><br>
**SuperProperty[Yield, Await] :**

&ensp;&ensp;super [ Expression[+In, ?Yield, ?Await] ]

&ensp;&ensp;super . IdentifierName
<br><br>
**MetaProperty :**

&ensp;&ensp;NewTarget

&ensp;&ensp;ImportMeta
<br><br>
**NewTarget :**

&ensp;&ensp;new . target
<br><br>
**ImportMeta :**

&ensp;&ensp;import . meta
<br><br>
**NewExpression[Yield, Await] :**

&ensp;&ensp;MemberExpression[?Yield, ?Await]

&ensp;&ensp;new NewExpression[?Yield, ?Await]
<br><br>
**CallExpression[Yield, Await] :**

&ensp;&ensp;CoverCallExpressionAndAsyncArrowHead[?Yield, ?Await]

&ensp;&ensp;SuperCall[?Yield, ?Await]

&ensp;&ensp;ImportCall[?Yield, ?Await]

&ensp;&ensp;CallExpression[?Yield, ?Await] Arguments[?Yield, ?Await]

&ensp;&ensp;CallExpression[?Yield, ?Await] [ Expression[+In, ?Yield, ?Await] ]

&ensp;&ensp;CallExpression[?Yield, ?Await] . IdentifierName

&ensp;&ensp;CallExpression[?Yield, ?Await] TemplateLiteral[?Yield, ?Await, +Tagged]

&ensp;&ensp;CallExpression[?Yield, ?Await] . PrivateIdentifier

&ensp;&ensp;When processing an instance of the production

&ensp;&ensp;CallExpression[Yield, Await] : CoverCallExpressionAndAsyncArrowHead[?Yield, ?Await]
<br><br>
**the interpretation of CoverCallExpressionAndAsyncArrowHead is refined using the following grammar:**

&ensp;&ensp;
<br><br>
**CallMemberExpression[Yield, Await] :**

&ensp;&ensp;MemberExpression[?Yield, ?Await] Arguments[?Yield, ?Await]

&ensp;&ensp;

&ensp;&ensp;
<br><br>
**SuperCall[Yield, Await] :**

&ensp;&ensp;super Arguments[?Yield, ?Await]
<br><br>
**ImportCall[Yield, Await] :**

&ensp;&ensp;import ( AssignmentExpression[+In, ?Yield, ?Await] )
<br><br>
**Arguments[Yield, Await] :**

&ensp;&ensp;( )

&ensp;&ensp;( ArgumentList[?Yield, ?Await] )

&ensp;&ensp;( ArgumentList[?Yield, ?Await] , )
<br><br>
**ArgumentList[Yield, Await] :**

&ensp;&ensp;AssignmentExpression[+In, ?Yield, ?Await]

&ensp;&ensp;... AssignmentExpression[+In, ?Yield, ?Await]

&ensp;&ensp;ArgumentList[?Yield, ?Await] , AssignmentExpression[+In, ?Yield, ?Await]

&ensp;&ensp;ArgumentList[?Yield, ?Await] , ... AssignmentExpression[+In, ?Yield, ?Await]
<br><br>
**OptionalExpression[Yield, Await] :**

&ensp;&ensp;MemberExpression[?Yield, ?Await] OptionalChain[?Yield, ?Await]

&ensp;&ensp;CallExpression[?Yield, ?Await] OptionalChain[?Yield, ?Await]

&ensp;&ensp;OptionalExpression[?Yield, ?Await] OptionalChain[?Yield, ?Await]
<br><br>
**OptionalChain[Yield, Await] :**

&ensp;&ensp;?. Arguments[?Yield, ?Await]

&ensp;&ensp;?. [ Expression[+In, ?Yield, ?Await] ]

&ensp;&ensp;?. IdentifierName

&ensp;&ensp;?. TemplateLiteral[?Yield, ?Await, +Tagged]

&ensp;&ensp;?. PrivateIdentifier

&ensp;&ensp;OptionalChain[?Yield, ?Await] Arguments[?Yield, ?Await]

&ensp;&ensp;OptionalChain[?Yield, ?Await] [ Expression[+In, ?Yield, ?Await] ]

&ensp;&ensp;OptionalChain[?Yield, ?Await] . IdentifierName

&ensp;&ensp;OptionalChain[?Yield, ?Await] TemplateLiteral[?Yield, ?Await, +Tagged]

&ensp;&ensp;OptionalChain[?Yield, ?Await] . PrivateIdentifier
<br><br>
**LeftHandSideExpression[Yield, Await] :**

&ensp;&ensp;NewExpression[?Yield, ?Await]

&ensp;&ensp;CallExpression[?Yield, ?Await]

&ensp;&ensp;OptionalExpression[?Yield, ?Await]
<br><br>
**UpdateExpression[Yield, Await] :**

&ensp;&ensp;LeftHandSideExpression[?Yield, ?Await]

&ensp;&ensp;LeftHandSideExpression[?Yield, ?Await] [no LineTerminator here] ++

&ensp;&ensp;LeftHandSideExpression[?Yield, ?Await] [no LineTerminator here] --

&ensp;&ensp;++ UnaryExpression[?Yield, ?Await]

&ensp;&ensp;-- UnaryExpression[?Yield, ?Await]
<br><br>
**UnaryExpression[Yield, Await] :**

&ensp;&ensp;UpdateExpression[?Yield, ?Await]

&ensp;&ensp;delete UnaryExpression[?Yield, ?Await]

&ensp;&ensp;void UnaryExpression[?Yield, ?Await]

&ensp;&ensp;typeof UnaryExpression[?Yield, ?Await]

&ensp;&ensp;+ UnaryExpression[?Yield, ?Await]

&ensp;&ensp;- UnaryExpression[?Yield, ?Await]

&ensp;&ensp;  ~ UnaryExpression[?Yield, ?Await]

&ensp;&ensp;  ! UnaryExpression[?Yield, ?Await]

&ensp;&ensp;  [+Await] AwaitExpression[?Yield]
<br><br>
**ExponentiationExpression[Yield, Await] :**

&ensp;&ensp;  UnaryExpression[?Yield, ?Await]

&ensp;&ensp;  UpdateExpression[?Yield, ?Await] ** ExponentiationExpression[?Yield, ?Await]
<br><br>
**MultiplicativeExpression[Yield, Await] :**

&ensp;&ensp;  ExponentiationExpression[?Yield, ?Await]

&ensp;&ensp;  MultiplicativeExpression[?Yield, ?Await] MultiplicativeOperator ExponentiationExpression[?Yield, ?Await]

&ensp;&ensp;  MultiplicativeOperator : one of

* / %
<br><br>
**AdditiveExpression[Yield, Await] :**

&ensp;&ensp;  MultiplicativeExpression[?Yield, ?Await]

&ensp;&ensp;  AdditiveExpression[?Yield, ?Await] + MultiplicativeExpression[?Yield, ?Await]

&ensp;&ensp;  AdditiveExpression[?Yield, ?Await] - MultiplicativeExpression[?Yield, ?Await]
<br><br>
**ShiftExpression[Yield, Await] :**

&ensp;&ensp;  AdditiveExpression[?Yield, ?Await]

&ensp;&ensp;  ShiftExpression[?Yield, ?Await] << AdditiveExpression[?Yield, ?Await]

&ensp;&ensp;  ShiftExpression[?Yield, ?Await] >> AdditiveExpression[?Yield, ?Await]

&ensp;&ensp;  ShiftExpression[?Yield, ?Await] >>> AdditiveExpression[?Yield, ?Await]
<br><br>
**RelationalExpression[In, Yield, Await] :**

&ensp;&ensp;  ShiftExpression[?Yield, ?Await]

&ensp;&ensp;  RelationalExpression[?In, ?Yield, ?Await] < ShiftExpression[?Yield, ?Await]

&ensp;&ensp;  RelationalExpression[?In, ?Yield, ?Await] > ShiftExpression[?Yield, ?Await]

&ensp;&ensp;  RelationalExpression[?In, ?Yield, ?Await] <= ShiftExpression[?Yield, ?Await]

&ensp;&ensp;  RelationalExpression[?In, ?Yield, ?Await] >= ShiftExpression[?Yield, ?Await]

&ensp;&ensp;  RelationalExpression[?In, ?Yield, ?Await] instanceof ShiftExpression[?Yield, ?Await]

&ensp;&ensp;  [+In] RelationalExpression[+In, ?Yield, ?Await] in ShiftExpression[?Yield, ?Await]

&ensp;&ensp;  [+In] PrivateIdentifier in ShiftExpression[?Yield, ?Await]
<br><br>
**EqualityExpression[In, Yield, Await] :**

&ensp;&ensp;  RelationalExpression[?In, ?Yield, ?Await]

&ensp;&ensp;  EqualityExpression[?In, ?Yield, ?Await] == RelationalExpression[?In, ?Yield, ?Await]

&ensp;&ensp;  EqualityExpression[?In, ?Yield, ?Await] != RelationalExpression[?In, ?Yield, ?Await]

&ensp;&ensp;  EqualityExpression[?In, ?Yield, ?Await] === RelationalExpression[?In, ?Yield, ?Await]

&ensp;&ensp;  EqualityExpression[?In, ?Yield, ?Await] !== RelationalExpression[?In, ?Yield, ?Await]
<br><br>
**BitwiseANDExpression[In, Yield, Await] :**

&ensp;&ensp;  EqualityExpression[?In, ?Yield, ?Await]

&ensp;&ensp;  BitwiseANDExpression[?In, ?Yield, ?Await] & EqualityExpression[?In, ?Yield, ?Await]
<br><br>
**BitwiseXORExpression[In, Yield, Await] :**

&ensp;&ensp;  BitwiseANDExpression[?In, ?Yield, ?Await]

&ensp;&ensp;  BitwiseXORExpression[?In, ?Yield, ?Await] ^ BitwiseANDExpression[?In, ?Yield, ?Await]
<br><br>
**BitwiseORExpression[In, Yield, Await] :**

&ensp;&ensp;  BitwiseXORExpression[?In, ?Yield, ?Await]

&ensp;&ensp;  BitwiseORExpression[?In, ?Yield, ?Await] | BitwiseXORExpression[?In, ?Yield, ?Await]
<br><br>
**LogicalANDExpression[In, Yield, Await] :**

&ensp;&ensp;  BitwiseORExpression[?In, ?Yield, ?Await]

&ensp;&ensp;  LogicalANDExpression[?In, ?Yield, ?Await] && BitwiseORExpression[?In, ?Yield, ?Await]
<br><br>
**LogicalORExpression[In, Yield, Await] :**

&ensp;&ensp;  LogicalANDExpression[?In, ?Yield, ?Await]

&ensp;&ensp;  LogicalORExpression[?In, ?Yield, ?Await] || LogicalANDExpression[?In, ?Yield, ?Await]
<br><br>
**CoalesceExpression[In, Yield, Await] :**

&ensp;&ensp;  CoalesceExpressionHead[?In, ?Yield, ?Await] ?? BitwiseORExpression[?In, ?Yield, ?Await]
<br><br>
**CoalesceExpressionHead[In, Yield, Await] :**

&ensp;&ensp;  CoalesceExpression[?In, ?Yield, ?Await]

&ensp;&ensp;  BitwiseORExpression[?In, ?Yield, ?Await]
<br><br>
**ShortCircuitExpression[In, Yield, Await] :**

&ensp;&ensp;  LogicalORExpression[?In, ?Yield, ?Await]

&ensp;&ensp;  CoalesceExpression[?In, ?Yield, ?Await]
<br><br>
**ConditionalExpression[In, Yield, Await] :**

&ensp;&ensp;  ShortCircuitExpression[?In, ?Yield, ?Await]

&ensp;&ensp;  ShortCircuitExpression[?In, ?Yield, ?Await] ? AssignmentExpression[+In, ?Yield, ?Await] : AssignmentExpression[?In, ?Yield, ?Await]
<br><br>
**AssignmentExpression[In, Yield, Await] :**

&ensp;&ensp;  ConditionalExpression[?In, ?Yield, ?Await]

&ensp;&ensp;  [+Yield] YieldExpression[?In, ?Await]

&ensp;&ensp;  ArrowFunction[?In, ?Yield, ?Await]

&ensp;&ensp;  AsyncArrowFunction[?In, ?Yield, ?Await]

&ensp;&ensp;  LeftHandSideExpression[?Yield, ?Await] = AssignmentExpression[?In, ?Yield, ?Await]

&ensp;&ensp;  LeftHandSideExpression[?Yield, ?Await] AssignmentOperator AssignmentExpression[?In, ?Yield, ?Await]

&ensp;&ensp;  LeftHandSideExpression[?Yield, ?Await] &&= AssignmentExpression[?In, ?Yield, ?Await]

&ensp;&ensp;  LeftHandSideExpression[?Yield, ?Await] ||= AssignmentExpression[?In, ?Yield, ?Await]

&ensp;&ensp;  LeftHandSideExpression[?Yield, ?Await] ??= AssignmentExpression[?In, ?Yield, ?Await]

&ensp;&ensp;  AssignmentOperator : one of

&ensp;&ensp;  *= /= %= += -= <<= >>= >>>= &= ^= |= **=

&ensp;&ensp;  In certain circumstances when processing an instance of the production

&ensp;&ensp;  AssignmentExpression[In, Yield, Await] : LeftHandSideExpression[?Yield, ?Await] = AssignmentExpression[?In, ?Yield, ?Await]
<br><br>
**the interpretation of LeftHandSideExpression is refined using the following grammar:**

&ensp;&ensp;
<br><br>
**AssignmentPattern[Yield, Await] :**

&ensp;&ensp;ObjectAssignmentPattern[?Yield, ?Await]

&ensp;&ensp;ArrayAssignmentPattern[?Yield, ?Await]
<br><br>
**ObjectAssignmentPattern[Yield, Await] :**

&ensp;&ensp;{ }

&ensp;&ensp;{ AssignmentRestProperty[?Yield, ?Await] }

&ensp;&ensp;{ AssignmentPropertyList[?Yield, ?Await] }

&ensp;&ensp;{ AssignmentPropertyList[?Yield, ?Await] , AssignmentRestProperty[?Yield, ?Await]opt }
<br><br>
**ArrayAssignmentPattern[Yield, Await] :**

&ensp;&ensp;[ Elisionopt AssignmentRestElement[?Yield, ?Await]opt ]

&ensp;&ensp;[ AssignmentElementList[?Yield, ?Await] ]

&ensp;&ensp;[ AssignmentElementList[?Yield, ?Await] , Elisionopt AssignmentRestElement[?Yield, ?Await]opt ]
<br><br>
**AssignmentRestProperty[Yield, Await] :**

&ensp;&ensp;... DestructuringAssignmentTarget[?Yield, ?Await]
<br><br>
**AssignmentPropertyList[Yield, Await] :**

&ensp;&ensp;AssignmentProperty[?Yield, ?Await]

&ensp;&ensp;AssignmentPropertyList[?Yield, ?Await] , AssignmentProperty[?Yield, ?Await]
<br><br>
**AssignmentElementList[Yield, Await] :**

&ensp;&ensp;AssignmentElisionElement[?Yield, ?Await]

&ensp;&ensp;AssignmentElementList[?Yield, ?Await] , AssignmentElisionElement[?Yield, ?Await]
<br><br>
**AssignmentElisionElement[Yield, Await] :**

&ensp;&ensp;Elisionopt AssignmentElement[?Yield, ?Await]
<br><br>
**AssignmentProperty[Yield, Await] :**

&ensp;&ensp;IdentifierReference[?Yield, ?Await] Initializer[+In, ?Yield, ?Await]opt

&ensp;&ensp;PropertyName[?Yield, ?Await] : AssignmentElement[?Yield, ?Await]
<br><br>
**AssignmentElement[Yield, Await] :**

&ensp;&ensp;DestructuringAssignmentTarget[?Yield, ?Await] Initializer[+In, ?Yield, ?Await]opt
<br><br>
**AssignmentRestElement[Yield, Await] :**

&ensp;&ensp;... DestructuringAssignmentTarget[?Yield, ?Await]
<br><br>
**DestructuringAssignmentTarget[Yield, Await] :**

&ensp;&ensp;LeftHandSideExpression[?Yield, ?Await]

&ensp;&ensp;

&ensp;&ensp;
<br><br>
**Expression[In, Yield, Await] :**

&ensp;&ensp;AssignmentExpression[?In, ?Yield, ?Await]

&ensp;&ensp;Expression[?In, ?Yield, ?Await] , AssignmentExpression[?In, ?Yield, ?Await]


## Statements

**Statement[Yield, Await, Return] :**

&ensp;&ensp;BlockStatement[?Yield, ?Await, ?Return]

&ensp;&ensp;VariableStatement[?Yield, ?Await]

&ensp;&ensp;EmptyStatement

&ensp;&ensp;ExpressionStatement[?Yield, ?Await]

&ensp;&ensp;IfStatement[?Yield, ?Await, ?Return]

&ensp;&ensp;BreakableStatement[?Yield, ?Await, ?Return]

&ensp;&ensp;ContinueStatement[?Yield, ?Await]

&ensp;&ensp;BreakStatement[?Yield, ?Await]

&ensp;&ensp;[+Return] ReturnStatement[?Yield, ?Await]

&ensp;&ensp;WithStatement[?Yield, ?Await, ?Return]

&ensp;&ensp;LabelledStatement[?Yield, ?Await, ?Return]

&ensp;&ensp;ThrowStatement[?Yield, ?Await]

&ensp;&ensp;TryStatement[?Yield, ?Await, ?Return]

&ensp;&ensp;DebuggerStatement
<br><br>
**Declaration[Yield, Await] :**

&ensp;&ensp;HoistableDeclaration[?Yield, ?Await, ~Default]

&ensp;&ensp;ClassDeclaration[?Yield, ?Await, ~Default]

&ensp;&ensp;LexicalDeclaration[+In, ?Yield, ?Await]
<br><br>
**HoistableDeclaration[Yield, Await, Default] :**

&ensp;&ensp;FunctionDeclaration[?Yield, ?Await, ?Default]

&ensp;&ensp;GeneratorDeclaration[?Yield, ?Await, ?Default]

&ensp;&ensp;AsyncFunctionDeclaration[?Yield, ?Await, ?Default]

&ensp;&ensp;AsyncGeneratorDeclaration[?Yield, ?Await, ?Default]
<br><br>
**BreakableStatement[Yield, Await, Return] :**

&ensp;&ensp;IterationStatement[?Yield, ?Await, ?Return]

&ensp;&ensp;SwitchStatement[?Yield, ?Await, ?Return]
<br><br>
**BlockStatement[Yield, Await, Return] :**

&ensp;&ensp;Block[?Yield, ?Await, ?Return]
<br><br>
**Block[Yield, Await, Return] :**

&ensp;&ensp;{ StatementList[?Yield, ?Await, ?Return]opt }
<br><br>
**StatementList[Yield, Await, Return] :**

&ensp;&ensp;StatementListItem[?Yield, ?Await, ?Return]

&ensp;&ensp;StatementList[?Yield, ?Await, ?Return] StatementListItem[?Yield, ?Await, ?Return]
<br><br>
**StatementListItem[Yield, Await, Return] :**

&ensp;&ensp;Statement[?Yield, ?Await, ?Return]

&ensp;&ensp;Declaration[?Yield, ?Await]
<br><br>
**LexicalDeclaration[In, Yield, Await] :**

&ensp;&ensp;LetOrConst BindingList[?In, ?Yield, ?Await] ;
<br><br>
**LetOrConst :**

&ensp;&ensp;let

&ensp;&ensp;const
<br><br>
**BindingList[In, Yield, Await] :**

&ensp;&ensp;LexicalBinding[?In, ?Yield, ?Await]

&ensp;&ensp;BindingList[?In, ?Yield, ?Await] , LexicalBinding[?In, ?Yield, ?Await]
<br><br>
**LexicalBinding[In, Yield, Await] :**

&ensp;&ensp;BindingIdentifier[?Yield, ?Await] Initializer[?In, ?Yield, ?Await]opt

&ensp;&ensp;BindingPattern[?Yield, ?Await] Initializer[?In, ?Yield, ?Await]
<br><br>
**VariableStatement[Yield, Await] :**

&ensp;&ensp;var VariableDeclarationList[+In, ?Yield, ?Await] ;
<br><br>
**VariableDeclarationList[In, Yield, Await] :**

&ensp;&ensp;VariableDeclaration[?In, ?Yield, ?Await]

&ensp;&ensp;VariableDeclarationList[?In, ?Yield, ?Await] , VariableDeclaration[?In, ?Yield, ?Await]
<br><br>
**VariableDeclaration[In, Yield, Await] :**

&ensp;&ensp;BindingIdentifier[?Yield, ?Await] Initializer[?In, ?Yield, ?Await]opt

&ensp;&ensp;BindingPattern[?Yield, ?Await] Initializer[?In, ?Yield, ?Await]
<br><br>
**BindingPattern[Yield, Await] :**

&ensp;&ensp;ObjectBindingPattern[?Yield, ?Await]

&ensp;&ensp;ArrayBindingPattern[?Yield, ?Await]
<br><br>
**ObjectBindingPattern[Yield, Await] :**

&ensp;&ensp;{ }

&ensp;&ensp;{ BindingRestProperty[?Yield, ?Await] }

&ensp;&ensp;{ BindingPropertyList[?Yield, ?Await] }

&ensp;&ensp;{ BindingPropertyList[?Yield, ?Await] , BindingRestProperty[?Yield, ?Await]opt }
<br><br>
**ArrayBindingPattern[Yield, Await] :**

&ensp;&ensp;[ Elisionopt BindingRestElement[?Yield, ?Await]opt ]

&ensp;&ensp;[ BindingElementList[?Yield, ?Await] ]

&ensp;&ensp;[ BindingElementList[?Yield, ?Await] , Elisionopt BindingRestElement[?Yield, ?Await]opt ]
<br><br>
**BindingRestProperty[Yield, Await] :**

&ensp;&ensp;... BindingIdentifier[?Yield, ?Await]
<br><br>
**BindingPropertyList[Yield, Await] :**

&ensp;&ensp;BindingProperty[?Yield, ?Await]

&ensp;&ensp;BindingPropertyList[?Yield, ?Await] , BindingProperty[?Yield, ?Await]
<br><br>
**BindingElementList[Yield, Await] :**

&ensp;&ensp;BindingElisionElement[?Yield, ?Await]

&ensp;&ensp;BindingElementList[?Yield, ?Await] , BindingElisionElement[?Yield, ?Await]
<br><br>
**BindingElisionElement[Yield, Await] :**

&ensp;&ensp;Elisionopt BindingElement[?Yield, ?Await]
<br><br>
**BindingProperty[Yield, Await] :**

&ensp;&ensp;SingleNameBinding[?Yield, ?Await]

&ensp;&ensp;PropertyName[?Yield, ?Await] : BindingElement[?Yield, ?Await]
<br><br>
**BindingElement[Yield, Await] :**

&ensp;&ensp;SingleNameBinding[?Yield, ?Await]

&ensp;&ensp;BindingPattern[?Yield, ?Await] Initializer[+In, ?Yield, ?Await]opt
<br><br>
**SingleNameBinding[Yield, Await] :**

&ensp;&ensp;BindingIdentifier[?Yield, ?Await] Initializer[+In, ?Yield, ?Await]opt
<br><br>
**BindingRestElement[Yield, Await] :**

&ensp;&ensp;... BindingIdentifier[?Yield, ?Await]

&ensp;&ensp;... BindingPattern[?Yield, ?Await]
<br><br>
**EmptyStatement :**

&ensp;&ensp;;
<br><br>
**ExpressionStatement[Yield, Await] :**

&ensp;&ensp;[lookahead ∉ { {, function, async [no LineTerminator here] function, class, let [ }] Expression[+In, ?Yield, ?Await] ;
<br><br>
**IfStatement[Yield, Await, Return] :**

&ensp;&ensp;if ( Expression[+In, ?Yield, ?Await] ) Statement[?Yield, ?Await, ?Return] else Statement[?Yield, ?Await, ?Return]

&ensp;&ensp;if ( Expression[+In, ?Yield, ?Await] ) Statement[?Yield, ?Await, ?Return] [lookahead ≠ else]
<br><br>
**IterationStatement[Yield, Await, Return] :**

&ensp;&ensp;DoWhileStatement[?Yield, ?Await, ?Return]

&ensp;&ensp;WhileStatement[?Yield, ?Await, ?Return]

&ensp;&ensp;ForStatement[?Yield, ?Await, ?Return]

&ensp;&ensp;ForInOfStatement[?Yield, ?Await, ?Return]
<br><br>
**DoWhileStatement[Yield, Await, Return] :**

&ensp;&ensp;do Statement[?Yield, ?Await, ?Return] while ( Expression[+In, ?Yield, ?Await] ) ;
<br><br>
**WhileStatement[Yield, Await, Return] :**

&ensp;&ensp;while ( Expression[+In, ?Yield, ?Await] ) Statement[?Yield, ?Await, ?Return]
<br><br>
**ForStatement[Yield, Await, Return] :**

&ensp;&ensp;for ( [lookahead ≠ let [] Expression[~In, ?Yield, ?Await]opt ; Expression[+In, ?Yield, ?Await]opt ; Expression[+In, ?Yield, ?Await]opt ) Statement[?Yield, ?Await, ?Return]

&ensp;&ensp;for ( var VariableDeclarationList[~In, ?Yield, ?Await] ; Expression[+In, ?Yield, ?Await]opt ; Expression[+In, ?Yield, ?Await]opt ) Statement[?Yield, ?Await, ?Return]

&ensp;&ensp;for ( LexicalDeclaration[~In, ?Yield, ?Await] Expression[+In, ?Yield, ?Await]opt ; Expression[+In, ?Yield, ?Await]opt ) Statement[?Yield, ?Await, ?Return]
<br><br>
**ForInOfStatement[Yield, Await, Return] :**

&ensp;&ensp;for ( [lookahead ≠ let [] LeftHandSideExpression[?Yield, ?Await] in Expression[+In, ?Yield, ?Await] ) Statement[?Yield, ?Await, ?Return]

&ensp;&ensp;for ( var ForBinding[?Yield, ?Await] in Expression[+In, ?Yield, ?Await] ) Statement[?Yield, ?Await, ?Return]

&ensp;&ensp;for ( ForDeclaration[?Yield, ?Await] in Expression[+In, ?Yield, ?Await] ) Statement[?Yield, ?Await, ?Return]

&ensp;&ensp;for ( [lookahead ∉ { let, async of }] LeftHandSideExpression[?Yield, ?Await] of AssignmentExpression[+In, ?Yield, ?Await] ) Statement[?Yield, ?Await, ?Return]

&ensp;&ensp;for ( var ForBinding[?Yield, ?Await] of AssignmentExpression[+In, ?Yield, ?Await] ) Statement[?Yield, ?Await, ?Return]

&ensp;&ensp;for ( ForDeclaration[?Yield, ?Await] of AssignmentExpression[+In, ?Yield, ?Await] ) Statement[?Yield, ?Await, ?Return]

&ensp;&ensp;[+Await] for await ( [lookahead ≠ let] LeftHandSideExpression[?Yield, ?Await] of AssignmentExpression[+In, ?Yield, ?Await] ) Statement[?Yield, ?Await, ?Return]

&ensp;&ensp;[+Await] for await ( var ForBinding[?Yield, ?Await] of AssignmentExpression[+In, ?Yield, ?Await] ) Statement[?Yield, ?Await, ?Return]

&ensp;&ensp;[+Await] for await ( ForDeclaration[?Yield, ?Await] of AssignmentExpression[+In, ?Yield, ?Await] ) Statement[?Yield, ?Await, ?Return]
<br><br>
**ForDeclaration[Yield, Await] :**

&ensp;&ensp;LetOrConst ForBinding[?Yield, ?Await]
<br><br>
**ForBinding[Yield, Await] :**

&ensp;&ensp;BindingIdentifier[?Yield, ?Await]

&ensp;&ensp;BindingPattern[?Yield, ?Await]
<br><br>
**ContinueStatement[Yield, Await] :**

&ensp;&ensp;continue ;

&ensp;&ensp;continue [no LineTerminator here] LabelIdentifier[?Yield, ?Await] ;
<br><br>
**BreakStatement[Yield, Await] :**

&ensp;&ensp;break ;

&ensp;&ensp;break [no LineTerminator here] LabelIdentifier[?Yield, ?Await] ;
<br><br>
**ReturnStatement[Yield, Await] :**

&ensp;&ensp;return ;

&ensp;&ensp;return [no LineTerminator here] Expression[+In, ?Yield, ?Await] ;
<br><br>
**WithStatement[Yield, Await, Return] :**

&ensp;&ensp;with ( Expression[+In, ?Yield, ?Await] ) Statement[?Yield, ?Await, ?Return]
<br><br>
**SwitchStatement[Yield, Await, Return] :**

&ensp;&ensp;switch ( Expression[+In, ?Yield, ?Await] ) CaseBlock[?Yield, ?Await, ?Return]
<br><br>
**CaseBlock[Yield, Await, Return] :**

&ensp;&ensp;{ CaseClauses[?Yield, ?Await, ?Return]opt }

&ensp;&ensp;{ CaseClauses[?Yield, ?Await, ?Return]opt DefaultClause[?Yield, ?Await, ?Return] CaseClauses[?Yield, ?Await, ?Return]opt }

&ensp;&ensp;CaseClauses[Yield, Await, Return] :
<br><br>
**CaseClause[?Yield, ?Await, ?Return]**

&ensp;&ensp;CaseClauses[?Yield, ?Await, ?Return] CaseClause[?Yield, ?Await, ?Return]
<br><br>
**CaseClause[Yield, Await, Return] :**

&ensp;&ensp;case Expression[+In, ?Yield, ?Await] : StatementList[?Yield, ?Await, ?Return]opt
<br><br>
**DefaultClause[Yield, Await, Return] :**

&ensp;&ensp;default : StatementList[?Yield, ?Await, ?Return]opt
<br><br>
**LabelledStatement[Yield, Await, Return] :**

&ensp;&ensp;LabelIdentifier[?Yield, ?Await] : LabelledItem[?Yield, ?Await, ?Return]
<br><br>
**LabelledItem[Yield, Await, Return] :**

&ensp;&ensp;Statement[?Yield, ?Await, ?Return]

&ensp;&ensp;FunctionDeclaration[?Yield, ?Await, ~Default]
<br><br>
**ThrowStatement[Yield, Await] :**

&ensp;&ensp;throw [no LineTerminator here] Expression[+In, ?Yield, ?Await] ;
<br><br>
**TryStatement[Yield, Await, Return] :**

&ensp;&ensp;try Block[?Yield, ?Await, ?Return] Catch[?Yield, ?Await, ?Return]

&ensp;&ensp;try Block[?Yield, ?Await, ?Return] Finally[?Yield, ?Await, ?Return]

&ensp;&ensp;try Block[?Yield, ?Await, ?Return] Catch[?Yield, ?Await, ?Return] Finally[?Yield, ?Await, ?Return]
<br><br>
**Catch[Yield, Await, Return] :**

&ensp;&ensp;catch ( CatchParameter[?Yield, ?Await] ) Block[?Yield, ?Await, ?Return]

&ensp;&ensp;catch Block[?Yield, ?Await, ?Return]
<br><br>
**Finally[Yield, Await, Return] :**

&ensp;&ensp;finally Block[?Yield, ?Await, ?Return]
<br><br>
**CatchParameter[Yield, Await] :**

&ensp;&ensp;BindingIdentifier[?Yield, ?Await]

&ensp;&ensp;BindingPattern[?Yield, ?Await]
<br><br>
**DebuggerStatement :**

&ensp;&ensp; debugger ;


## Functions and Classes

**UniqueFormalParameters[Yield, Await] :**

&ensp;&ensp;FormalParameters[?Yield, ?Await]
<br><br>
**FormalParameters[Yield, Await] :**

&ensp;&ensp; [empty]

&ensp;&ensp; FunctionRestParameter[?Yield, ?Await]

&ensp;&ensp; FormalParameterList[?Yield, ?Await]

&ensp;&ensp; FormalParameterList[?Yield, ?Await] ,

&ensp;&ensp; FormalParameterList[?Yield, ?Await] , FunctionRestParameter[?Yield, ?Await]

&ensp;&ensp; FormalParameterList[Yield, Await] :

&ensp;&ensp; FormalParameter[?Yield, ?Await]
<br><br>
**FormalParameterList[?Yield, ?Await] , FormalParameter[?Yield, ?Await]**

&ensp;&ensp; FunctionRestParameter[Yield, Await] :

&ensp;&ensp; BindingRestElement[?Yield, ?Await]
<br><br>
**FormalParameter[Yield, Await] :**

&ensp;&ensp; BindingElement[?Yield, ?Await]
<br><br>
**FunctionDeclaration[Yield, Await, Default] :**

&ensp;&ensp; function BindingIdentifier[?Yield, ?Await] ( FormalParameters[~Yield, ~Await] ) { FunctionBody[~Yield, ~Await] }

&ensp;&ensp; [+Default] function ( FormalParameters[~Yield, ~Await] ) { FunctionBody[~Yield, ~Await] }
<br><br>
**FunctionExpression :**

&ensp;&ensp; function BindingIdentifier[~Yield, ~Await]opt ( FormalParameters[~Yield, ~Await] ) { FunctionBody[~Yield, ~Await] }
<br><br>
**FunctionBody[Yield, Await] :**

&ensp;&ensp; FunctionStatementList[?Yield, ?Await]
<br><br>
**FunctionStatementList[Yield, Await] :**

&ensp;&ensp; StatementList[?Yield, ?Await, +Return]opt
<br><br>
**ArrowFunction[In, Yield, Await] :**

&ensp;&ensp; ArrowParameters[?Yield, ?Await] [no LineTerminator here] => ConciseBody[?In]
<br><br>
**ArrowParameters[Yield, Await] :**

&ensp;&ensp; BindingIdentifier[?Yield, ?Await]

&ensp;&ensp; CoverParenthesizedExpressionAndArrowParameterList[?Yield, ?Await]
<br><br>
**ConciseBody[In] :**

&ensp;&ensp; [lookahead ≠ {] ExpressionBody[?In, ~Await]

&ensp;&ensp; { FunctionBody[~Yield, ~Await] }
<br><br>
**ExpressionBody[In, Await] :**

&ensp;&ensp; AssignmentExpression[?In, ~Yield, ?Await]



When processing an instance of the production

ArrowParameters[Yield, Await] : CoverParenthesizedExpressionAndArrowParameterList[?Yield, ?Await]

the interpretation of CoverParenthesizedExpressionAndArrowParameterList is refined using the following grammar:
<br><br>
**ArrowFormalParameters[Yield, Await] :**

&ensp;&ensp; ( UniqueFormalParameters[?Yield, ?Await] )
<br><br>
**AsyncArrowFunction[In, Yield, Await] :**

&ensp;&ensp; async [no LineTerminator here] AsyncArrowBindingIdentifier[?Yield] [no LineTerminator here] => AsyncConciseBody[?In]

&ensp;&ensp; CoverCallExpressionAndAsyncArrowHead[?Yield, ?Await] [no LineTerminator here] => AsyncConciseBody[?In]
<br><br>
**AsyncConciseBody[In] :**

&ensp;&ensp; [lookahead ≠ {] ExpressionBody[?In, +Await]

&ensp;&ensp; { AsyncFunctionBody }
<br><br>
**AsyncArrowBindingIdentifier[Yield] :**

&ensp;&ensp; BindingIdentifier[?Yield, +Await]
<br><br>
**CoverCallExpressionAndAsyncArrowHead[Yield, Await] :**

&ensp;&ensp; MemberExpression[?Yield, ?Await] Arguments[?Yield, ?Await]



When processing an instance of the production

AsyncArrowFunction[In, Yield, Await] : CoverCallExpressionAndAsyncArrowHead[?Yield, ?Await] [no LineTerminator here] => AsyncConciseBody[?In]

the interpretation of CoverCallExpressionAndAsyncArrowHead is refined using the following grammar:
<br><br>
**AsyncArrowHead :**

&ensp;&ensp; async [no LineTerminator here] ArrowFormalParameters[~Yield, +Await]
<br><br>
**MethodDefinition[Yield, Await] :**

&ensp;&ensp; ClassElementName[?Yield, ?Await] ( UniqueFormalParameters[~Yield, ~Await] ) { FunctionBody[~Yield, ~Await] }

&ensp;&ensp; GeneratorMethod[?Yield, ?Await]

&ensp;&ensp; AsyncMethod[?Yield, ?Await]

&ensp;&ensp; AsyncGeneratorMethod[?Yield, ?Await]

&ensp;&ensp; get ClassElementName[?Yield, ?Await] ( ) { FunctionBody[~Yield, ~Await] }

&ensp;&ensp; set ClassElementName[?Yield, ?Await] ( PropertySetParameterList ) { FunctionBody[~Yield, ~Await] }
<br><br>
**PropertySetParameterList :**

&ensp;&ensp; FormalParameter[~Yield, ~Await]
<br><br>
**GeneratorDeclaration[Yield, Await, Default] :**

&ensp;&ensp; function * BindingIdentifier[?Yield, ?Await] ( FormalParameters[+Yield, ~Await] ) { GeneratorBody }

[+Default] function * ( FormalParameters[+Yield, ~Await] ) { GeneratorBody }

&ensp;&ensp; **GeneratorExpression :**

&ensp;&ensp; function * BindingIdentifier[+Yield, ~Await]opt ( FormalParameters[+Yield, ~Await] ) { GeneratorBody }
<br><br>
**GeneratorMethod[Yield, Await] :**

&ensp;&ensp; * ClassElementName[?Yield, ?Await] ( UniqueFormalParameters[+Yield, ~Await] ) { GeneratorBody }
<br><br>
**GeneratorBody :**

&ensp;&ensp; FunctionBody[+Yield, ~Await]
<br><br>
**YieldExpression[In, Await] :**

&ensp;&ensp; yield

&ensp;&ensp; yield [no LineTerminator here] AssignmentExpression[?In, +Yield, ?Await]

&ensp;&ensp; yield [no LineTerminator here] * AssignmentExpression[?In, +Yield, ?Await]

&ensp;&ensp; **AsyncGeneratorDeclaration[Yield, Await, Default] :**

&ensp;&ensp; async [no LineTerminator here] function * BindingIdentifier[?Yield, ?Await] ( FormalParameters[+Yield, +Await] ) { AsyncGeneratorBody }

&ensp;&ensp; [+Default] async [no LineTerminator here] function * ( FormalParameters[+Yield, +Await] ) { AsyncGeneratorBody }
<br><br>
**AsyncGeneratorExpression :**

&ensp;&ensp; async [no LineTerminator here] function * BindingIdentifier[+Yield, +Await]opt ( FormalParameters[+Yield, +Await] ) { AsyncGeneratorBody }
<br><br>
**AsyncGeneratorMethod[Yield, Await] :**

&ensp;&ensp; async [no LineTerminator here] * ClassElementName[?Yield, ?Await] ( UniqueFormalParameters[+Yield, +Await] ) { AsyncGeneratorBody }
<br><br>
**AsyncGeneratorBody :**

&ensp;&ensp; FunctionBody[+Yield, +Await]
<br><br>
**AsyncFunctionDeclaration[Yield, Await, Default] :**

&ensp;&ensp; async [no LineTerminator here] function BindingIdentifier[?Yield, ?Await] ( FormalParameters[~Yield, +Await] ) { AsyncFunctionBody }

&ensp;&ensp; [+Default] async [no LineTerminator here] function ( FormalParameters[~Yield, +Await] ) { AsyncFunctionBody }
<br><br>
**AsyncFunctionExpression :**

&ensp;&ensp; async [no LineTerminator here] function BindingIdentifier[~Yield, +Await]opt ( FormalParameters[~Yield, +Await] ) { AsyncFunctionBody }
<br><br>
**AsyncMethod[Yield, Await] :**

&ensp;&ensp; async [no LineTerminator here] ClassElementName[?Yield, ?Await] ( UniqueFormalParameters[~Yield, +Await] ) { AsyncFunctionBody }
<br><br>
**AsyncFunctionBody :**

&ensp;&ensp; FunctionBody[~Yield, +Await]
<br><br>
**AwaitExpression[Yield] :**

&ensp;&ensp; await UnaryExpression[?Yield, +Await]
<br><br>
**ClassDeclaration[Yield, Await, Default] :**

&ensp;&ensp; class BindingIdentifier[?Yield, ?Await] ClassTail[?Yield, ?Await]

&ensp;&ensp; [+Default] class ClassTail[?Yield, ?Await]
<br><br>
**ClassExpression[Yield, Await] :**

&ensp;&ensp; class BindingIdentifier[?Yield, ?Await]opt ClassTail[?Yield, ?Await]
<br><br>
**ClassTail[Yield, Await] :**

&ensp;&ensp; ClassHeritage[?Yield, ?Await]opt { ClassBody[?Yield, ?Await]opt }
<br><br>
**ClassHeritage[Yield, Await] :**

&ensp;&ensp; extends LeftHandSideExpression[?Yield, ?Await]
<br><br>
**ClassBody[Yield, Await] :**

&ensp;&ensp; ClassElementList[?Yield, ?Await]
<br><br>
**ClassElementList[Yield, Await] :**

&ensp;&ensp; ClassElement[?Yield, ?Await]

&ensp;&ensp; ClassElementList[?Yield, ?Await] ClassElement[?Yield, ?Await]
<br><br>
**ClassElement[Yield, Await] :**

&ensp;&ensp; MethodDefinition[?Yield, ?Await]

&ensp;&ensp; static MethodDefinition[?Yield, ?Await]

&ensp;&ensp; FieldDefinition[?Yield, ?Await] ;

&ensp;&ensp; static FieldDefinition[?Yield, ?Await] ;

&ensp;&ensp; ClassStaticBlock

&ensp;&ensp; ;
<br><br>
**FieldDefinition[Yield, Await] :**

&ensp;&ensp; ClassElementName[?Yield, ?Await] Initializer[+In, ?Yield, ?Await]opt
<br><br>
**ClassElementName[Yield, Await] :**

&ensp;&ensp; PropertyName[?Yield, ?Await]

&ensp;&ensp; PrivateIdentifier
<br><br>
**ClassStaticBlock :**

&ensp;&ensp; static { ClassStaticBlockBody }
<br><br>
**ClassStaticBlockBody :**

&ensp;&ensp; ClassStaticBlockStatementList
<br><br>
**ClassStaticBlockStatementList :**

&ensp;&ensp; StatementList[~Yield, +Await, ~Return]opt
