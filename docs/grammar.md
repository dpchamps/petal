# Modified Grammar Productions

Here lists modified only grammar productions for https://tc39.es/ecma262.

Modifications must strictly be in the form of a subset of ECMAScript

----

### Scripts and modules

Replaces productions for https://tc39.es/ecma262/#sec-scripts-and-modules


Module :

&nbsp;&nbsp;&nbsp;&nbsp;ModuleBody <sub>[opt]</sub>

ModuleBody :

&nbsp;&nbsp;&nbsp;&nbsp;ModuleItemList

ModuleItemList :

&nbsp;&nbsp;&nbsp;&nbsp;ModuleItem

&nbsp;&nbsp;&nbsp;&nbsp;ModuleItemList ModuleItem

ModuleItem :

&nbsp;&nbsp;&nbsp;&nbsp;ImportDeclaration

&nbsp;&nbsp;&nbsp;&nbsp;ExportDeclaration

&nbsp;&nbsp;&nbsp;&nbsp;StatementListItem <sub>[~Yield, +Await, ~Return]</sub>

ModuleExportName :

&nbsp;&nbsp;&nbsp;&nbsp;IdentifierName

ImportDeclaration :

&nbsp;&nbsp;&nbsp;&nbsp;`import` ImportClause FromClause `;`


ImportClause :

&nbsp;&nbsp;&nbsp;&nbsp;NameSpaceImport

&nbsp;&nbsp;&nbsp;&nbsp;NamedImports

NameSpaceImport :

&nbsp;&nbsp;&nbsp;&nbsp;`*` `as` ImportedBinding
 
NamedImports :

&nbsp;&nbsp;&nbsp;&nbsp;`{` `}`

&nbsp;&nbsp;&nbsp;&nbsp;`{` ImportsList `}`

&nbsp;&nbsp;&nbsp;&nbsp;`{` ImportsList `,` `}`

FromClause :

&nbsp;&nbsp;&nbsp;&nbsp;`from` ModuleSpecifier

ImportsList :

&nbsp;&nbsp;&nbsp;&nbsp;ImportSpecifier

&nbsp;&nbsp;&nbsp;&nbsp;ImportsList `,` ImportSpecifier

ImportSpecifier :

&nbsp;&nbsp;&nbsp;&nbsp;ImportedBinding

&nbsp;&nbsp;&nbsp;&nbsp;ModuleExportName `as` ImportedBinding

ModuleSpecifier :

&nbsp;&nbsp;&nbsp;&nbsp;StringLiteral

ImportedBinding :

&nbsp;&nbsp;&nbsp;&nbsp;BindingIdentifier <sub>[~Yield, +Await]</sub>

ExportDeclaration :

&nbsp;&nbsp;&nbsp;&nbsp;`export` ExportFromClause FromClause `;`

&nbsp;&nbsp;&nbsp;&nbsp;`export` NamedExports `;`

&nbsp;&nbsp;&nbsp;&nbsp;`export` Declaration <sub>[~Yield, +Await]</sub>

ExportFromClause :

&nbsp;&nbsp;&nbsp;&nbsp; `*`

&nbsp;&nbsp;&nbsp;&nbsp; `*` `as` ModuleExportName

&nbsp;&nbsp;&nbsp;&nbsp;NamedExports

NamedExports :

&nbsp;&nbsp;&nbsp;&nbsp;`{` `}`

&nbsp;&nbsp;&nbsp;&nbsp;`{` ExportsList `}`

&nbsp;&nbsp;&nbsp;&nbsp;`{` ExportsList `,` `}`

ExportsList :

&nbsp;&nbsp;&nbsp;&nbsp;ExportSpecifier

&nbsp;&nbsp;&nbsp;&nbsp;ExportsList `,` ExportSpecifier

ExportSpecifier :

&nbsp;&nbsp;&nbsp;&nbsp;ModuleExportName

&nbsp;&nbsp;&nbsp;&nbsp;ModuleExportName `as` ModuleExportName

### Functions and Classes

Replaces productions for https://tc39.es/ecma262/#sec-functions-and-classes

UniqueFormalParameters<sub>[Yield, Await]</sub> :

&nbsp;&nbsp;&nbsp;&nbsp;FormalParameters<sub>[?Yield, ?Await]</sub>

FormalParameters<sub>[Yield, Await]</sub> :

&nbsp;&nbsp;&nbsp;&nbsp;[empty]

&nbsp;&nbsp;&nbsp;&nbsp;FunctionRestParameter<sub>[?Yield, ?Await]</sub>

&nbsp;&nbsp;&nbsp;&nbsp;FormalParameterList<sub>[?Yield, ?Await]</sub>

&nbsp;&nbsp;&nbsp;&nbsp;FormalParameterList<sub>[?Yield, ?Await]</sub> `,`

&nbsp;&nbsp;&nbsp;&nbsp;FormalParameterList<sub>[?Yield, ?Await]</sub> `,` FunctionRestParameter<sub>[?Yield, ?Await]</sub>

FormalParameterList<sub>[Yield, Await]</sub> :

&nbsp;&nbsp;&nbsp;&nbsp;FormalParameter<sub>[?Yield, ?Await]</sub>

&nbsp;&nbsp;&nbsp;&nbsp;FormalParameterList<sub>[?Yield, ?Await]</sub> `,` FormalParameter<sub>[?Yield, ?Await]</sub>

FunctionRestParameter<sub>[Yield, Await]</sub> :

&nbsp;&nbsp;&nbsp;&nbsp;BindingRestElement<sub>[?Yield, ?Await]</sub>

FormalParameter<sub>[Yield, Await]</sub> :

&nbsp;&nbsp;&nbsp;&nbsp;BindingElement<sub>[?Yield, ?Await]</sub>

FunctionDeclaration<sub>[Yield, Await, Default]</sub> :

&nbsp;&nbsp;&nbsp;&nbsp;`function` BindingIdentifier<sub>[?Yield, ?Await]</sub> ( FormalParameters<sub>[~Yield, ~Await]</sub> ) `{` FunctionBody<sub>[~Yield, ~Await]</sub> `}`

&nbsp;&nbsp;&nbsp;&nbsp;<sub>[+Default]</sub> `function` ( FormalParameters<sub>[~Yield, ~Await]</sub> ) `{` FunctionBody<sub>[~Yield, ~Await]</sub> `}`

FunctionExpression :

&nbsp;&nbsp;&nbsp;&nbsp;`function` BindingIdentifier<sub>[~Yield, ~Await]</sub>opt ( FormalParameters<sub>[~Yield, ~Await]</sub> ) `{` FunctionBody<sub>[~Yield, ~Await]</sub> `}`

FunctionBody<sub>[Yield, Await]</sub> :

&nbsp;&nbsp;&nbsp;&nbsp;FunctionStatementList<sub>[?Yield, ?Await]</sub>

FunctionStatementList<sub>[Yield, Await]</sub> :

&nbsp;&nbsp;&nbsp;&nbsp;StatementList<sub>[?Yield, ?Await, +Return]</sub>opt

ArrowFunction<sub>[In, Yield, Await]</sub> :

&nbsp;&nbsp;&nbsp;&nbsp;ArrowParameters<sub>[?Yield, ?Await]</sub> <sub>[no LineTerminator here]</sub> => ConciseBody<sub>[?In]</sub>

ArrowParameters<sub>[Yield, Await]</sub> :

&nbsp;&nbsp;&nbsp;&nbsp;BindingIdentifier<sub>[?Yield, ?Await]</sub>

&nbsp;&nbsp;&nbsp;&nbsp;CoverParenthesizedExpressionAndArrowParameterList<sub>[?Yield, ?Await]</sub>

ConciseBody<sub>[In]</sub> :

&nbsp;&nbsp;&nbsp;&nbsp;<sub>[lookahead ≠ {]</sub> ExpressionBody<sub>[?In, ~Await]</sub>

&nbsp;&nbsp;&nbsp;&nbsp;`{` FunctionBody<sub>[~Yield, ~Await]</sub> `}`

ExpressionBody<sub>[In, Await]</sub> :

&nbsp;&nbsp;&nbsp;&nbsp;AssignmentExpression<sub>[?In, ~Yield, ?Await]</sub>

When processing an instance of the production

ArrowParameters<sub>[Yield, Await]</sub> `:` CoverParenthesizedExpressionAndArrowParameterList<sub>[?Yield, ?Await]</sub>

the interpretation of CoverParenthesizedExpressionAndArrowParameterList is refined using the following grammar:


ArrowFormalParameters<sub>[Yield, Await]</sub> :

&nbsp;&nbsp;&nbsp;&nbsp;`(` UniqueFormalParameters<sub>[?Yield, ?Await]</sub> `)`


AsyncArrowFunction<sub>[In, Yield, Await]</sub> :

&nbsp;&nbsp;&nbsp;&nbsp;`async` <sub>[no LineTerminator here]</sub> AsyncArrowBindingIdentifier<sub>[?Yield]</sub> <sub>[no LineTerminator here]</sub> `=>` AsyncConciseBody<sub>[?In]</sub>

&nbsp;&nbsp;&nbsp;&nbsp;CoverCallExpressionAndAsyncArrowHead<sub>[?Yield, ?Await]</sub> <sub>[no LineTerminator here]</sub> `=>` AsyncConciseBody<sub>[?In]</sub>

AsyncConciseBody<sub>[In]</sub> :

&nbsp;&nbsp;&nbsp;&nbsp;<sub>[lookahead ≠ {]</sub> ExpressionBody<sub>[?In, +Await]</sub>

&nbsp;&nbsp;&nbsp;&nbsp;`{` AsyncFunctionBody `}`

AsyncArrowBindingIdentifier<sub>[Yield]</sub> :

&nbsp;&nbsp;&nbsp;&nbsp;BindingIdentifier<sub>[?Yield, +Await]</sub>

CoverCallExpressionAndAsyncArrowHead<sub>[Yield, Await]</sub> :

&nbsp;&nbsp;&nbsp;&nbsp;MemberExpression<sub>[?Yield, ?Await]</sub> Arguments<sub>[?Yield, ?Await]</sub>

When processing an instance of the production

AsyncArrowFunction<sub>[In, Yield, Await]</sub> `:` CoverCallExpressionAndAsyncArrowHead<sub>[?Yield, ?Await]</sub> <sub>[no LineTerminator here]</sub> `=>` AsyncConciseBody<sub>[?In]</sub>

the interpretation of CoverCallExpressionAndAsyncArrowHead is refined using the following grammar:



AsyncArrowHead :

&nbsp;&nbsp;&nbsp;&nbsp;`async` <sub>[no LineTerminator here]</sub> ArrowFormalParameters<sub>[~Yield, +Await]</sub>

PropertySetParameterList :

&nbsp;&nbsp;&nbsp;&nbsp;FormalParameter<sub>[~Yield, ~Await]</sub>

GeneratorDeclaration<sub>[Yield, Await, Default]</sub> :

&nbsp;&nbsp;&nbsp;&nbsp;`function` `*` BindingIdentifier<sub>[?Yield, ?Await]</sub> `(` FormalParameters<sub>[+Yield, ~Await]</sub> `)` `{` GeneratorBody `}`

&nbsp;&nbsp;&nbsp;&nbsp;<sub>[+Default]</sub> `function` `*` `(` FormalParameters<sub>[+Yield, ~Await]</sub> `)` `{` GeneratorBody `}`

GeneratorExpression :

&nbsp;&nbsp;&nbsp;&nbsp;`function` `*` BindingIdentifier<sub><sub>[+Yield, ~Await]</sub>opt</sub> `(` FormalParameters<sub>[+Yield, ~Await]</sub> `)` `{` GeneratorBody `}`

GeneratorBody :

&nbsp;&nbsp;&nbsp;&nbsp;FunctionBody<sub>[+Yield, ~Await]</sub>

YieldExpression<sub>[In, Await]</sub> :

&nbsp;&nbsp;&nbsp;&nbsp;`yield`

&nbsp;&nbsp;&nbsp;&nbsp;`yield` <sub>[no LineTerminator here]</sub> AssignmentExpression<sub>[?In, +Yield, ?Await]</sub>

&nbsp;&nbsp;&nbsp;&nbsp;`yield` <sub>[no LineTerminator here]</sub> `*` AssignmentExpression<sub>[?In, +Yield, ?Await]</sub>

AsyncGeneratorDeclaration<sub>[Yield, Await, Default]</sub> :

&nbsp;&nbsp;&nbsp;&nbsp;`async` <sub>[no LineTerminator here]</sub> `function` `*` BindingIdentifier<sub>[?Yield, ?Await]</sub> `(` FormalParameters<sub>[+Yield, +Await]</sub> `)` `{` AsyncGeneratorBody `}`

&nbsp;&nbsp;&nbsp;&nbsp;<sub>[+Default]</sub> `async` <sub>[no LineTerminator here]</sub> `function` `*` `(` FormalParameters<sub>[+Yield, +Await]</sub> `)` `{` AsyncGeneratorBody `}`

AsyncGeneratorExpression :

&nbsp;&nbsp;&nbsp;&nbsp;`async` <sub>[no LineTerminator here]</sub> `function` `*` BindingIdentifier<sub>[+Yield, +Await]</sub>opt `(` FormalParameters<sub>[+Yield, +Await]</sub> `)` `{` AsyncGeneratorBody `}`

AsyncGeneratorBody :

&nbsp;&nbsp;&nbsp;&nbsp;FunctionBody<sub>[+Yield, +Await]</sub>

AsyncFunctionDeclaration<sub>[Yield, Await, Default]</sub> :

&nbsp;&nbsp;&nbsp;&nbsp;`async` <sub>[no LineTerminator here]</sub> `function` BindingIdentifier<sub>[?Yield, ?Await]</sub> `(` FormalParameters<sub>[~Yield, +Await]</sub> `)` `{` AsyncFunctionBody `}`

&nbsp;&nbsp;&nbsp;&nbsp;<sub>[+Default]</sub> `async` <sub>[no LineTerminator here]</sub> `function` `(` FormalParameters<sub>[~Yield, +Await]</sub> `)` `{` AsyncFunctionBody `}`

AsyncFunctionExpression :

&nbsp;&nbsp;&nbsp;&nbsp;`async` <sub>[no LineTerminator here]</sub> `function` BindingIdentifier<sub>[~Yield, +Await]</sub>opt `(` FormalParameters<sub>[~Yield, +Await]</sub> `)` `{` AsyncFunctionBody `}`

AsyncFunctionBody :

&nbsp;&nbsp;&nbsp;&nbsp;FunctionBody<sub>[~Yield, +Await]</sub>

AwaitExpression<sub>[Yield]</sub> :

&nbsp;&nbsp;&nbsp;&nbsp;`await` UnaryExpression<sub>[?Yield, +Await]</sub>

### Statements


Statement<sub>[Yield, Await, Return]</sub> :

&nbsp;&nbsp;&nbsp;&nbsp;BlockStatement<sub>[?Yield, ?Await, ?Return]</sub>

&nbsp;&nbsp;&nbsp;&nbsp;VariableStatement<sub>[?Yield, ?Await]</sub>

&nbsp;&nbsp;&nbsp;&nbsp;EmptyStatement

&nbsp;&nbsp;&nbsp;&nbsp;ExpressionStatement<sub>[?Yield, ?Await]</sub>

&nbsp;&nbsp;&nbsp;&nbsp;IfStatement<sub>[?Yield, ?Await, ?Return]</sub>

&nbsp;&nbsp;&nbsp;&nbsp;BreakableStatement<sub>[?Yield, ?Await, ?Return]</sub>

&nbsp;&nbsp;&nbsp;&nbsp;ContinueStatement<sub>[?Yield, ?Await]</sub>

&nbsp;&nbsp;&nbsp;&nbsp;BreakStatement<sub>[?Yield, ?Await]</sub>

&nbsp;&nbsp;&nbsp;&nbsp;<sub>[+Return]</sub> ReturnStatement<sub>[?Yield, ?Await]</sub>

&nbsp;&nbsp;&nbsp;&nbsp;WithStatement<sub>[?Yield, ?Await, ?Return]</sub>

&nbsp;&nbsp;&nbsp;&nbsp;LabelledStatement<sub>[?Yield, ?Await, ?Return]</sub>

&nbsp;&nbsp;&nbsp;&nbsp;ThrowStatement<sub>[?Yield, ?Await]</sub>

&nbsp;&nbsp;&nbsp;&nbsp;TryStatement<sub>[?Yield, ?Await, ?Return]</sub>

&nbsp;&nbsp;&nbsp;&nbsp;DebuggerStatement

Declaration<sub>[Yield, Await]</sub> :

&nbsp;&nbsp;&nbsp;&nbsp;HoistableDeclaration<sub>[?Yield, ?Await, ~Default]</sub>

&nbsp;&nbsp;&nbsp;&nbsp;LexicalDeclaration<sub>[+In, ?Yield, ?Await]</sub>

HoistableDeclaration<sub>[Yield, Await, Default]</sub> :

&nbsp;&nbsp;&nbsp;&nbsp;FunctionDeclaration<sub>[?Yield, ?Await, ?Default]</sub>

&nbsp;&nbsp;&nbsp;&nbsp;GeneratorDeclaration<sub>[?Yield, ?Await, ?Default]</sub>

&nbsp;&nbsp;&nbsp;&nbsp;AsyncFunctionDeclaration<sub>[?Yield, ?Await, ?Default]</sub>

&nbsp;&nbsp;&nbsp;&nbsp;AsyncGeneratorDeclaration<sub>[?Yield, ?Await, ?Default]</sub>

BreakableStatement<sub>[Yield, Await, Return]</sub> :

&nbsp;&nbsp;&nbsp;&nbsp;IterationStatement<sub>[?Yield, ?Await, ?Return]</sub>

&nbsp;&nbsp;&nbsp;&nbsp;SwitchStatement<sub>[?Yield, ?Await, ?Return]</sub>

BlockStatement<sub>[Yield, Await, Return]</sub> :

&nbsp;&nbsp;&nbsp;&nbsp;Block<sub>[?Yield, ?Await, ?Return]</sub>

Block<sub>[Yield, Await, Return]</sub> :

&nbsp;&nbsp;&nbsp;&nbsp;`{` StatementList<sub>[?Yield, ?Await, ?Return]</sub>opt `}`

StatementList<sub>[Yield, Await, Return]</sub> :

&nbsp;&nbsp;&nbsp;&nbsp;StatementListItem<sub>[?Yield, ?Await, ?Return]</sub>

&nbsp;&nbsp;&nbsp;&nbsp;StatementList<sub>[?Yield, ?Await, ?Return]</sub> StatementListItem<sub>[?Yield, ?Await, ?Return]</sub>

StatementListItem<sub>[Yield, Await, Return]</sub> :

&nbsp;&nbsp;&nbsp;&nbsp;Statement<sub>[?Yield, ?Await, ?Return]</sub>

&nbsp;&nbsp;&nbsp;&nbsp;Declaration<sub>[?Yield, ?Await]</sub>

LexicalDeclaration<sub>[In, Yield, Await]</sub> :

&nbsp;&nbsp;&nbsp;&nbsp;LetOrConst BindingList<sub>[?In, ?Yield, ?Await]</sub> ;

LetOrConst :

&nbsp;&nbsp;&nbsp;&nbsp;`let`

&nbsp;&nbsp;&nbsp;&nbsp;`const`

BindingList<sub>[In, Yield, Await]</sub> :

&nbsp;&nbsp;&nbsp;&nbsp;LexicalBinding<sub>[?In, ?Yield, ?Await]</sub>

&nbsp;&nbsp;&nbsp;&nbsp;BindingList<sub>[?In, ?Yield, ?Await]</sub> , LexicalBinding<sub>[?In, ?Yield, ?Await]</sub>

LexicalBinding<sub>[In, Yield, Await]</sub> :

&nbsp;&nbsp;&nbsp;&nbsp;BindingIdentifier<sub>[?Yield, ?Await]</sub> Initializer<sub>[?In, ?Yield, ?Await]</sub>opt

&nbsp;&nbsp;&nbsp;&nbsp;BindingPattern<sub>[?Yield, ?Await]</sub> Initializer<sub>[?In, ?Yield, ?Await]</sub>

BindingPattern<sub>[Yield, Await]</sub> :

&nbsp;&nbsp;&nbsp;&nbsp;ObjectBindingPattern<sub>[?Yield, ?Await]</sub>

&nbsp;&nbsp;&nbsp;&nbsp;ArrayBindingPattern<sub>[?Yield, ?Await]</sub>

ObjectBindingPattern<sub>[Yield, Await]</sub> :

&nbsp;&nbsp;&nbsp;&nbsp;`{` `}`

&nbsp;&nbsp;&nbsp;&nbsp;`{` BindingRestProperty<sub>[?Yield, ?Await]</sub> `}`

&nbsp;&nbsp;&nbsp;&nbsp;`{` BindingPropertyList<sub>[?Yield, ?Await]</sub> `}`

&nbsp;&nbsp;&nbsp;&nbsp;`{` BindingPropertyList<sub>[?Yield, ?Await]</sub> , BindingRestProperty<sub>[?Yield, ?Await]</sub>opt `}`

ArrayBindingPattern<sub>[Yield, Await]</sub> :

&nbsp;&nbsp;&nbsp;&nbsp;`[` Elisionopt BindingRestElement[?Yield, ?Await]opt `]`

&nbsp;&nbsp;&nbsp;&nbsp;`[` BindingElementList[?Yield, ?Await] `]`

&nbsp;&nbsp;&nbsp;&nbsp;`[` BindingElementList[?Yield, ?Await] , Elisionopt BindingRestElement<sub>[?Yield, ?Await]</sub>opt `]`

BindingRestProperty<sub>[Yield, Await]</sub> :

&nbsp;&nbsp;&nbsp;&nbsp;`...` BindingIdentifier<sub>[?Yield, ?Await]</sub>

BindingPropertyList<sub>[Yield, Await]</sub> :

&nbsp;&nbsp;&nbsp;&nbsp;BindingProperty<sub>[?Yield, ?Await]</sub>

&nbsp;&nbsp;&nbsp;&nbsp;BindingPropertyList<sub>[?Yield, ?Await]</sub> `,` BindingProperty<sub>[?Yield, ?Await]</sub>

BindingElementList<sub>[Yield, Await]</sub> :

&nbsp;&nbsp;&nbsp;&nbsp;BindingElisionElement<sub>[?Yield, ?Await]</sub>

&nbsp;&nbsp;&nbsp;&nbsp;BindingElementList<sub>[?Yield, ?Await]</sub> `,` BindingElisionElement<sub>[?Yield, ?Await]</sub>

BindingElisionElement<sub>[Yield, Await]</sub> :

&nbsp;&nbsp;&nbsp;&nbsp;Elisionopt BindingElement<sub>[?Yield, ?Await]</sub>

BindingProperty<sub>[Yield, Await]</sub> :

&nbsp;&nbsp;&nbsp;&nbsp;SingleNameBinding<sub>[?Yield, ?Await]</sub>

&nbsp;&nbsp;&nbsp;&nbsp;PropertyName<sub>[?Yield, ?Await]</sub> `:` BindingElement<sub>[?Yield, ?Await]</sub>

BindingElement<sub>[Yield, Await]</sub> :

&nbsp;&nbsp;&nbsp;&nbsp;SingleNameBinding<sub>[?Yield, ?Await]</sub>

&nbsp;&nbsp;&nbsp;&nbsp;BindingPattern<sub>[?Yield, ?Await]</sub> Initializer<sub>[+In, ?Yield, ?Await]</sub>opt

SingleNameBinding<sub>[Yield, Await]</sub> :

&nbsp;&nbsp;&nbsp;&nbsp;BindingIdentifier<sub>[?Yield, ?Await]</sub> Initializer<sub>[+In, ?Yield, ?Await]</sub>opt

BindingRestElement<sub>[Yield, Await]</sub> :

&nbsp;&nbsp;&nbsp;&nbsp;`...` BindingIdentifier<sub>[?Yield, ?Await]</sub>

&nbsp;&nbsp;&nbsp;&nbsp;`...` BindingPattern<sub>[?Yield, ?Await]</sub>

EmptyStatement :

&nbsp;&nbsp;&nbsp;&nbsp;`;`

ExpressionStatement<sub>[Yield, Await]</sub> :

&nbsp;&nbsp;&nbsp;&nbsp;[lookahead ∉ { {, function, async [no LineTerminator here] function, class, let [ }] Expression<sub>[+In, ?Yield, ?Await]</sub> ;

IfStatement<sub>[Yield, Await, Return]</sub> :

&nbsp;&nbsp;&nbsp;&nbsp;`if` ( Expression<sub>[+In, ?Yield, ?Await]</sub> ) Statement<sub>[?Yield, ?Await, ?Return]</sub> `else` Statement<sub>[?Yield, ?Await, ?Return]</sub>

&nbsp;&nbsp;&nbsp;&nbsp;`if` ( Expression<sub>[+In, ?Yield, ?Await]</sub> ) Statement<sub>[?Yield, ?Await, ?Return]</sub> <sub>[lookahead ≠ else]</sub>

IterationStatement<sub>[Yield, Await, Return]</sub> :

&nbsp;&nbsp;&nbsp;&nbsp;DoWhileStatement<sub>[?Yield, ?Await, ?Return]</sub>

&nbsp;&nbsp;&nbsp;&nbsp;WhileStatement<sub>[?Yield, ?Await, ?Return]</sub>

&nbsp;&nbsp;&nbsp;&nbsp;ForStatement<sub>[?Yield, ?Await, ?Return]</sub>

&nbsp;&nbsp;&nbsp;&nbsp;ForInOfStatement<sub>[?Yield, ?Await, ?Return]</sub>

DoWhileStatement<sub>[Yield, Await, Return]</sub> :

&nbsp;&nbsp;&nbsp;&nbsp;`do` Statement<sub>[?Yield, ?Await, ?Return]</sub> `while` ( Expression<sub>[+In, ?Yield, ?Await]</sub> ) `;`

WhileStatement<sub>[Yield, Await, Return]</sub> :

&nbsp;&nbsp;&nbsp;&nbsp;`while` ( Expression<sub>[+In, ?Yield, ?Await]</sub> ) Statement<sub>[?Yield, ?Await, ?Return]</sub>

ForStatement<sub>[Yield, Await, Return]</sub> :

&nbsp;&nbsp;&nbsp;&nbsp;`for` `(` [lookahead ≠ let [] Expression<sub>[~In, ?Yield, ?Await]</sub>opt `;` Expression<sub>[+In, ?Yield, ?Await]</sub>opt `;` Expression<sub>[+In, ?Yield, ?Await]</sub>opt `)` Statement<sub>[?Yield, ?Await, ?Return]</sub>

&nbsp;&nbsp;&nbsp;&nbsp;`for` `(` LexicalDeclaration<sub>[~In, ?Yield, ?Await]</sub> Expression<sub>[+In, ?Yield, ?Await]</sub>opt `;` Expression<sub>[+In, ?Yield, ?Await]</sub>opt `)` Statement<sub>[?Yield, ?Await, ?Return]</sub>

ForInOfStatement<sub>[Yield, Await, Return]</sub> :

&nbsp;&nbsp;&nbsp;&nbsp;`for` `(` <sub>[lookahead ≠ let []</sub> LeftHandSideExpression<sub>[?Yield, ?Await]</sub> `in` Expression<sub>[+In, ?Yield, ?Await]</sub> `)` Statement<sub>[?Yield, ?Await, ?Return]</sub>

&nbsp;&nbsp;&nbsp;&nbsp;`for` `(` ForDeclaration<sub>[?Yield, ?Await]</sub> `in` Expression<sub>[+In, ?Yield, ?Await]</sub> `)` Statement<sub>[?Yield, ?Await, ?Return]</sub>

&nbsp;&nbsp;&nbsp;&nbsp;`for` `(` <sub>[lookahead ∉ { let, async of }]</sub> LeftHandSideExpression<sub>[?Yield, ?Await]</sub> `of` AssignmentExpression<sub>[+In, ?Yield, ?Await]</sub> `)` Statement<sub>[?Yield, ?Await, ?Return]</sub>

&nbsp;&nbsp;&nbsp;&nbsp;`for` `(` ForDeclaration<sub>[?Yield, ?Await]</sub> `of` AssignmentExpression<sub>[+In, ?Yield, ?Await]</sub> `)` Statement<sub>[?Yield, ?Await, ?Return]</sub>

&nbsp;&nbsp;&nbsp;&nbsp;<sub>[+Await]</sub> `for` `await` `(` <sub>[lookahead ≠ let]</sub> LeftHandSideExpression<sub>[?Yield, ?Await]</sub> `of` AssignmentExpression<sub>[+In, ?Yield, ?Await]</sub> `)` Statement<sub>[?Yield, ?Await, ?Return]</sub>

&nbsp;&nbsp;&nbsp;&nbsp;<sub>[+Await]</sub> `for` `await` `(` var ForBinding<sub>[?Yield, ?Await]</sub> `of` AssignmentExpression<sub>[+In, ?Yield, ?Await]</sub> `)` Statement<sub>[?Yield, ?Await, ?Return]</sub>

&nbsp;&nbsp;&nbsp;&nbsp;<sub>[+Await]</sub> `for` `await` `(` ForDeclaration<sub>[?Yield, ?Await]</sub> `of` AssignmentExpression<sub>[+In, ?Yield, ?Await]</sub> `)` Statement<sub>[?Yield, ?Await, ?Return]</sub>

ForDeclaration<sub>[Yield, Await]</sub> :

&nbsp;&nbsp;&nbsp;&nbsp;LetOrConst ForBinding<sub>[?Yield, ?Await]</sub>

ForBinding<sub>[Yield, Await]</sub> :

&nbsp;&nbsp;&nbsp;&nbsp;BindingIdentifier<sub>[?Yield, ?Await]</sub>

&nbsp;&nbsp;&nbsp;&nbsp;BindingPattern<sub>[?Yield, ?Await]</sub>

ContinueStatement<sub>[Yield, Await]</sub> :

&nbsp;&nbsp;&nbsp;&nbsp;`continue` `;`

&nbsp;&nbsp;&nbsp;&nbsp;`continue` <sub>[no LineTerminator here]</sub> LabelIdentifier<sub>[?Yield, ?Await]</sub> `;`

BreakStatement<sub>[Yield, Await]</sub> :

&nbsp;&nbsp;&nbsp;&nbsp;`break` `;`

&nbsp;&nbsp;&nbsp;&nbsp;`break` <sub>[no LineTerminator here]</sub> LabelIdentifier<sub>[?Yield, ?Await]</sub> ;

ReturnStatement<sub>[Yield, Await]</sub> :

&nbsp;&nbsp;&nbsp;&nbsp;`return` `;`

&nbsp;&nbsp;&nbsp;&nbsp;`return` <sub>[no LineTerminator here]</sub> Expression<sub>[+In, ?Yield, ?Await]</sub> `;`

SwitchStatement<sub>[Yield, Await, Return]</sub> :

&nbsp;&nbsp;&nbsp;&nbsp;`switch` `(` Expression<sub>[+In, ?Yield, ?Await]</sub> `)` CaseBlock<sub>[?Yield, ?Await, ?Return]</sub>

CaseBlock<sub>[Yield, Await, Return]</sub> :

&nbsp;&nbsp;&nbsp;&nbsp;`{` CaseClauses<sub>[?Yield, ?Await, ?Return]</sub>opt `}`

&nbsp;&nbsp;&nbsp;&nbsp;`{` CaseClauses<sub>[?Yield, ?Await, ?Return]</sub>opt DefaultClause<sub>[?Yield, ?Await, ?Return]</sub> CaseClauses<sub>[?Yield, ?Await, ?Return]</sub>opt `}`

CaseClauses<sub>[Yield, Await, Return]</sub> :

&nbsp;&nbsp;&nbsp;&nbsp;CaseClause<sub>[?Yield, ?Await, ?Return]</sub>

&nbsp;&nbsp;&nbsp;&nbsp;CaseClauses<sub>[?Yield, ?Await, ?Return]</sub> CaseClause<sub>[?Yield, ?Await, ?Return]</sub>

CaseClause<sub>[Yield, Await, Return]</sub> :

&nbsp;&nbsp;&nbsp;&nbsp;`case` Expression<sub>[+In, ?Yield, ?Await]</sub> `:` StatementList<sub>[?Yield, ?Await, ?Return]</sub>opt

DefaultClause<sub>[Yield, Await, Return]</sub> :

&nbsp;&nbsp;&nbsp;&nbsp;`default` `:` StatementList<sub>[?Yield, ?Await, ?Return]</sub>opt

LabelledStatement<sub>[Yield, Await, Return]</sub> :

&nbsp;&nbsp;&nbsp;&nbsp;LabelIdentifier<sub>[?Yield, ?Await]</sub> `:` LabelledItem<sub>[?Yield, ?Await, ?Return]</sub>

LabelledItem<sub>[Yield, Await, Return]</sub> :

&nbsp;&nbsp;&nbsp;&nbsp;Statement<sub>[?Yield, ?Await, ?Return]</sub>

&nbsp;&nbsp;&nbsp;&nbsp;FunctionDeclaration<sub>[?Yield, ?Await, ~Default]</sub>

ThrowStatement<sub>[Yield, Await]</sub> :

&nbsp;&nbsp;&nbsp;&nbsp;`throw` <sub>[no LineTerminator here]</sub> Expression<sub>[+In, ?Yield, ?Await]</sub> `;`

TryStatement<sub>[Yield, Await, Return]</sub> :

&nbsp;&nbsp;&nbsp;&nbsp;`try` Block<sub>[?Yield, ?Await, ?Return]</sub> Catch<sub>[?Yield, ?Await, ?Return]</sub>

&nbsp;&nbsp;&nbsp;&nbsp;`try` Block<sub>[?Yield, ?Await, ?Return]</sub> Finally<sub>[?Yield, ?Await, ?Return]</sub>

&nbsp;&nbsp;&nbsp;&nbsp;`try` Block<sub>[?Yield, ?Await, ?Return]</sub> Catch<sub>[?Yield, ?Await, ?Return]</sub> Finally<sub>[?Yield, ?Await, ?Return]</sub>

Catch<sub>[Yield, Await, Return]</sub> :

&nbsp;&nbsp;&nbsp;&nbsp;`catch` `(` CatchParameter<sub>[?Yield, ?Await]</sub> `)` Block<sub>[?Yield, ?Await, ?Return]</sub>

&nbsp;&nbsp;&nbsp;&nbsp;`catch` Block<sub>[?Yield, ?Await, ?Return]</sub>

Finally<sub>[Yield, Await, Return]</sub> :

&nbsp;&nbsp;&nbsp;&nbsp;`finally` Block<sub>[?Yield, ?Await, ?Return]</sub>

CatchParameter<sub>[Yield, Await]</sub> :

&nbsp;&nbsp;&nbsp;&nbsp;BindingIdentifier<sub>[?Yield, ?Await]</sub>

&nbsp;&nbsp;&nbsp;&nbsp;BindingPattern<sub>[?Yield, ?Await]</sub>

DebuggerStatement :

&nbsp;&nbsp;&nbsp;&nbsp;`debugger` `;`

### Expressions

IdentifierReference<sub>[Yield, Await]</sub> :

&nbsp;&nbsp;&nbsp;&nbsp;Identifier

&nbsp;&nbsp;&nbsp;&nbsp;<sub>[~Yield]</sub> `yield`

&nbsp;&nbsp;&nbsp;&nbsp;<sub>[~Await]</sub> `await`

BindingIdentifier<sub>[Yield, Await]</sub> :

&nbsp;&nbsp;&nbsp;&nbsp;Identifier

&nbsp;&nbsp;&nbsp;&nbsp;`yield`

&nbsp;&nbsp;&nbsp;&nbsp;`await`

LabelIdentifier<sub>[Yield, Await]</sub> :

&nbsp;&nbsp;&nbsp;&nbsp;Identifier

&nbsp;&nbsp;&nbsp;&nbsp;<sub>[~Yield]</sub> `yield`

&nbsp;&nbsp;&nbsp;&nbsp;<sub>[~Await]</sub> `await`

Identifier :

&nbsp;&nbsp;&nbsp;&nbsp;IdentifierName but not ReservedWord

PrimaryExpression<sub>[Yield, Await]</sub> :

&nbsp;&nbsp;&nbsp;&nbsp;IdentifierReference<sub>[?Yield, ?Await]</sub>

&nbsp;&nbsp;&nbsp;&nbsp;Literal

&nbsp;&nbsp;&nbsp;&nbsp;ArrayLiteral<sub>[?Yield, ?Await]</sub>

&nbsp;&nbsp;&nbsp;&nbsp;ObjectLiteral<sub>[?Yield, ?Await]</sub>

&nbsp;&nbsp;&nbsp;&nbsp;FunctionExpression

&nbsp;&nbsp;&nbsp;&nbsp;GeneratorExpression

&nbsp;&nbsp;&nbsp;&nbsp;AsyncFunctionExpression

&nbsp;&nbsp;&nbsp;&nbsp;AsyncGeneratorExpression

&nbsp;&nbsp;&nbsp;&nbsp;RegularExpressionLiteral

&nbsp;&nbsp;&nbsp;&nbsp;TemplateLiteral<sub>[?Yield, ?Await, ~Tagged]</sub>

&nbsp;&nbsp;&nbsp;&nbsp;CoverParenthesizedExpressionAndArrowParameterList<sub>[?Yield, ?Await]</sub>

CoverParenthesizedExpressionAndArrowParameterList<sub>[Yield, Await]</sub> :

&nbsp;&nbsp;&nbsp;&nbsp;`(` Expression<sub>[+In, ?Yield, ?Await]</sub> `)`

&nbsp;&nbsp;&nbsp;&nbsp;`(` Expression<sub>[+In, ?Yield, ?Await]</sub> `,` `)`

&nbsp;&nbsp;&nbsp;&nbsp;`(` `)`

&nbsp;&nbsp;&nbsp;&nbsp;`(` `...` BindingIdentifier<sub>[?Yield, ?Await]</sub> `)`

&nbsp;&nbsp;&nbsp;&nbsp;`(` `...` BindingPattern<sub>[?Yield, ?Await]</sub> `)`

&nbsp;&nbsp;&nbsp;&nbsp;`(` Expression<sub>[+In, ?Yield, ?Await]</sub> `,` `...` BindingIdentifier<sub>[?Yield, ?Await]</sub> `)`

&nbsp;&nbsp;&nbsp;&nbsp;`(` Expression<sub>[+In, ?Yield, ?Await]</sub> `,` `...` BindingPattern<sub>[?Yield, ?Await]</sub> `)`

When processing an instance of the production

PrimaryExpression<sub>[Yield, Await]</sub> `:` CoverParenthesizedExpressionAndArrowParameterList<sub>[?Yield, ?Await]</sub>



the interpretation of CoverParenthesizedExpressionAndArrowParameterList is refined using the following grammar:



ParenthesizedExpression<sub>[Yield, Await]</sub> :

&nbsp;&nbsp;&nbsp;&nbsp;`(` Expression<sub>[+In, ?Yield, ?Await]</sub> `)`



Literal :

&nbsp;&nbsp;&nbsp;&nbsp;NullLiteral

&nbsp;&nbsp;&nbsp;&nbsp;BooleanLiteral

&nbsp;&nbsp;&nbsp;&nbsp;NumericLiteral

&nbsp;&nbsp;&nbsp;&nbsp;StringLiteral

ArrayLiteral<sub>[Yield, Await]</sub> :

&nbsp;&nbsp;&nbsp;&nbsp;`[` Elisionopt `]`

&nbsp;&nbsp;&nbsp;&nbsp;`[` ElementList[?Yield, ?Await] `]`

&nbsp;&nbsp;&nbsp;&nbsp;`[` ElementList[?Yield, ?Await] , Elisionopt `]`

ElementList<sub>[Yield, Await]</sub> :

&nbsp;&nbsp;&nbsp;&nbsp;Elisionopt AssignmentExpression<sub>[+In, ?Yield, ?Await]</sub>

&nbsp;&nbsp;&nbsp;&nbsp;Elisionopt SpreadElement<sub>[?Yield, ?Await]</sub>

&nbsp;&nbsp;&nbsp;&nbsp;ElementList<sub>[?Yield, ?Await]</sub> `,` Elisionopt AssignmentExpression<sub>[+In, ?Yield, ?Await]</sub>

&nbsp;&nbsp;&nbsp;&nbsp;ElementList<sub>[?Yield, ?Await]</sub> `,` Elisionopt SpreadElement<sub>[?Yield, ?Await]</sub>

Elision :

&nbsp;&nbsp;&nbsp;&nbsp;`,`

&nbsp;&nbsp;&nbsp;&nbsp;Elision `,`

SpreadElement<sub>[Yield, Await]</sub> :

&nbsp;&nbsp;&nbsp;&nbsp;`...` AssignmentExpression<sub>[+In, ?Yield, ?Await]</sub>

ObjectLiteral<sub>[Yield, Await]</sub> :

&nbsp;&nbsp;&nbsp;&nbsp;`{` `}`

&nbsp;&nbsp;&nbsp;&nbsp;`{` PropertyDefinitionList<sub>[?Yield, ?Await]</sub> `}`

&nbsp;&nbsp;&nbsp;&nbsp;`{` PropertyDefinitionList<sub>[?Yield, ?Await]</sub> `,` `}`

PropertyDefinitionList<sub>[Yield, Await]</sub> :

&nbsp;&nbsp;&nbsp;&nbsp;PropertyDefinition<sub>[?Yield, ?Await]</sub>

&nbsp;&nbsp;&nbsp;&nbsp;PropertyDefinitionList<sub>[?Yield, ?Await]</sub> `,` PropertyDefinition<sub>[?Yield, ?Await]</sub>

PropertyDefinition<sub>[Yield, Await]</sub> :

&nbsp;&nbsp;&nbsp;&nbsp;IdentifierReference<sub>[?Yield, ?Await]</sub>

&nbsp;&nbsp;&nbsp;&nbsp;CoverInitializedName<sub>[?Yield, ?Await]</sub>

&nbsp;&nbsp;&nbsp;&nbsp;PropertyName<sub>[?Yield, ?Await]</sub> `:` AssignmentExpression<sub>[+In, ?Yield, ?Await]</sub>

&nbsp;&nbsp;&nbsp;&nbsp;`...` AssignmentExpression<sub>[+In, ?Yield, ?Await]</sub>

PropertyName<sub>[Yield, Await]</sub> :

&nbsp;&nbsp;&nbsp;&nbsp;LiteralPropertyName

&nbsp;&nbsp;&nbsp;&nbsp;ComputedPropertyName<sub>[?Yield, ?Await]</sub>

LiteralPropertyName :

&nbsp;&nbsp;&nbsp;&nbsp;IdentifierName

&nbsp;&nbsp;&nbsp;&nbsp;StringLiteral

&nbsp;&nbsp;&nbsp;&nbsp;NumericLiteral

ComputedPropertyName<sub>[Yield, Await]</sub> :

&nbsp;&nbsp;&nbsp;&nbsp;[ AssignmentExpression[+In, ?Yield, ?Await] ]

CoverInitializedName<sub>[Yield, Await]</sub> :

&nbsp;&nbsp;&nbsp;&nbsp;IdentifierReference<sub>[?Yield, ?Await]</sub> Initializer<sub>[+In, ?Yield, ?Await]</sub>

Initializer<sub>[In, Yield, Await]</sub> :

&nbsp;&nbsp;&nbsp;&nbsp;`=` AssignmentExpression<sub>[?In, ?Yield, ?Await]</sub>

TemplateLiteral<sub>[Yield, Await, Tagged]</sub> :

&nbsp;&nbsp;&nbsp;&nbsp;NoSubstitutionTemplate

&nbsp;&nbsp;&nbsp;&nbsp;SubstitutionTemplate<sub>[?Yield, ?Await, ?Tagged]</sub>

SubstitutionTemplate<sub>[Yield, Await, Tagged]</sub> :

&nbsp;&nbsp;&nbsp;&nbsp;TemplateHead Expression<sub>[+In, ?Yield, ?Await]</sub> TemplateSpans<sub>[?Yield, ?Await, ?Tagged]</sub>

TemplateSpans<sub>[Yield, Await, Tagged]</sub> :

&nbsp;&nbsp;&nbsp;&nbsp;TemplateTail

&nbsp;&nbsp;&nbsp;&nbsp;TemplateMiddleList<sub>[?Yield, ?Await, ?Tagged]</sub> TemplateTail

TemplateMiddleList<sub>[Yield, Await, Tagged]</sub> :

&nbsp;&nbsp;&nbsp;&nbsp;TemplateMiddle Expression<sub>[+In, ?Yield, ?Await]</sub>

&nbsp;&nbsp;&nbsp;&nbsp;TemplateMiddleList<sub>[?Yield, ?Await, ?Tagged]</sub> TemplateMiddle Expression<sub>[+In, ?Yield, ?Await]</sub>

MemberExpression<sub>[Yield, Await]</sub> :

&nbsp;&nbsp;&nbsp;&nbsp;PrimaryExpression<sub>[?Yield, ?Await]</sub>

&nbsp;&nbsp;&nbsp;&nbsp;MemberExpression<sub>[?Yield, ?Await]</sub> `[` Expression[+In, ?Yield, ?Await] `]`

&nbsp;&nbsp;&nbsp;&nbsp;MemberExpression<sub>[?Yield, ?Await]</sub> `.` IdentifierName

&nbsp;&nbsp;&nbsp;&nbsp;MemberExpression<sub>[?Yield, ?Await]</sub> TemplateLiteral<sub>[?Yield, ?Await, +Tagged]</sub>

&nbsp;&nbsp;&nbsp;&nbsp;MetaProperty

&nbsp;&nbsp;&nbsp;&nbsp;`new` MemberExpression<sub>[?Yield, ?Await]</sub> Arguments<sub>[?Yield, ?Await]</sub>

&nbsp;&nbsp;&nbsp;&nbsp;MemberExpression<sub>[?Yield, ?Await]</sub> `.` PrivateIdentifier

MetaProperty :

&nbsp;&nbsp;&nbsp;&nbsp;NewTarget

&nbsp;&nbsp;&nbsp;&nbsp;ImportMeta

NewTarget :

&nbsp;&nbsp;&nbsp;&nbsp;`new` `.` target

ImportMeta :

&nbsp;&nbsp;&nbsp;&nbsp;`import` `.` `meta`

NewExpression<sub>[Yield, Await]</sub> :

&nbsp;&nbsp;&nbsp;&nbsp;MemberExpression<sub>[?Yield, ?Await]</sub>

&nbsp;&nbsp;&nbsp;&nbsp;`new` NewExpression<sub>[?Yield, ?Await]</sub>

CallExpression<sub>[Yield, Await]</sub> :

&nbsp;&nbsp;&nbsp;&nbsp;CoverCallExpressionAndAsyncArrowHead<sub>[?Yield, ?Await]</sub>

&nbsp;&nbsp;&nbsp;&nbsp;ImportCall<sub>[?Yield, ?Await]</sub>

&nbsp;&nbsp;&nbsp;&nbsp;CallExpression<sub>[?Yield, ?Await]</sub> Arguments<sub>[?Yield, ?Await]</sub>

&nbsp;&nbsp;&nbsp;&nbsp;CallExpression<sub>[?Yield, ?Await]</sub> [ Expression[+In, ?Yield, ?Await] ]

&nbsp;&nbsp;&nbsp;&nbsp;CallExpression<sub>[?Yield, ?Await]</sub> `.` IdentifierName

&nbsp;&nbsp;&nbsp;&nbsp;CallExpression<sub>[?Yield, ?Await]</sub> TemplateLiteral<sub>[?Yield, ?Await, +Tagged]</sub>

&nbsp;&nbsp;&nbsp;&nbsp;CallExpression<sub>[?Yield, ?Await]</sub> `.` PrivateIdentifier

When processing an instance of the production

CallExpression<sub>[Yield, Await]</sub> : CoverCallExpressionAndAsyncArrowHead<sub>[?Yield, ?Await]</sub>



the interpretation of CoverCallExpressionAndAsyncArrowHead is refined using the following grammar:



CallMemberExpression<sub>[Yield, Await]</sub> :

&nbsp;&nbsp;&nbsp;&nbsp;MemberExpression<sub>[?Yield, ?Await]</sub> Arguments<sub>[?Yield, ?Await]</sub>

ImportCall<sub>[Yield, Await]</sub> :

&nbsp;&nbsp;&nbsp;&nbsp;import ( AssignmentExpression<sub>[+In, ?Yield, ?Await]</sub> )

Arguments<sub>[Yield, Await]</sub> :

&nbsp;&nbsp;&nbsp;&nbsp;`(` `)`

&nbsp;&nbsp;&nbsp;&nbsp;`(` ArgumentList<sub>[?Yield, ?Await]</sub> `)`

&nbsp;&nbsp;&nbsp;&nbsp;`(` ArgumentList<sub>[?Yield, ?Await]</sub> `,` `)`

ArgumentList<sub>[Yield, Await]</sub> :

&nbsp;&nbsp;&nbsp;&nbsp;AssignmentExpression<sub>[+In, ?Yield, ?Await]</sub>

&nbsp;&nbsp;&nbsp;&nbsp;`...` AssignmentExpression<sub>[+In, ?Yield, ?Await]</sub>

&nbsp;&nbsp;&nbsp;&nbsp;ArgumentList<sub>[?Yield, ?Await]</sub> `,` AssignmentExpression<sub>[+In, ?Yield, ?Await]</sub>

&nbsp;&nbsp;&nbsp;&nbsp;ArgumentList<sub>[?Yield, ?Await]</sub> `,` `...` AssignmentExpression<sub>[+In, ?Yield, ?Await]</sub>

OptionalExpression<sub>[Yield, Await]</sub> :

&nbsp;&nbsp;&nbsp;&nbsp;MemberExpression<sub>[?Yield, ?Await]</sub> OptionalChain<sub>[?Yield, ?Await]</sub>

&nbsp;&nbsp;&nbsp;&nbsp;CallExpression<sub>[?Yield, ?Await]</sub> OptionalChain<sub>[?Yield, ?Await]</sub>

&nbsp;&nbsp;&nbsp;&nbsp;OptionalExpression<sub>[?Yield, ?Await]</sub> OptionalChain<sub>[?Yield, ?Await]</sub>

OptionalChain<sub>[Yield, Await]</sub> :

&nbsp;&nbsp;&nbsp;&nbsp;`?.` Arguments<sub>[?Yield, ?Await]</sub>

&nbsp;&nbsp;&nbsp;&nbsp;`?.` `<sub>[` Expression[+In, ?Yield, ?Await]</sub> `]`

&nbsp;&nbsp;&nbsp;&nbsp;`?.` IdentifierName

&nbsp;&nbsp;&nbsp;&nbsp;`?.` TemplateLiteral<sub>[?Yield, ?Await, +Tagged]</sub>

&nbsp;&nbsp;&nbsp;&nbsp;`?.` PrivateIdentifier

&nbsp;&nbsp;&nbsp;&nbsp;OptionalChain<sub>[?Yield, ?Await]</sub> Arguments<sub>[?Yield, ?Await]</sub>

&nbsp;&nbsp;&nbsp;&nbsp;OptionalChain<sub>[?Yield, ?Await]</sub> `[` Expression[+In, ?Yield, ?Await] `]`

&nbsp;&nbsp;&nbsp;&nbsp;OptionalChain<sub>[?Yield, ?Await]</sub> `.` IdentifierName

&nbsp;&nbsp;&nbsp;&nbsp;OptionalChain<sub>[?Yield, ?Await]</sub> TemplateLiteral<sub>[?Yield, ?Await, +Tagged]</sub>

&nbsp;&nbsp;&nbsp;&nbsp;OptionalChain<sub>[?Yield, ?Await]</sub> `.` PrivateIdentifier

LeftHandSideExpression<sub>[Yield, Await]</sub> :

&nbsp;&nbsp;&nbsp;&nbsp;NewExpression<sub>[?Yield, ?Await]</sub>

&nbsp;&nbsp;&nbsp;&nbsp;CallExpression<sub>[?Yield, ?Await]</sub>

&nbsp;&nbsp;&nbsp;&nbsp;OptionalExpression<sub>[?Yield, ?Await]</sub>

UpdateExpression<sub>[Yield, Await]</sub> :

&nbsp;&nbsp;&nbsp;&nbsp;LeftHandSideExpression<sub>[?Yield, ?Await]</sub>

&nbsp;&nbsp;&nbsp;&nbsp;LeftHandSideExpression<sub>[?Yield, ?Await]</sub> <sub>[no LineTerminator here]</sub> `++`

&nbsp;&nbsp;&nbsp;&nbsp;LeftHandSideExpression<sub>[?Yield, ?Await]</sub> <sub>[no LineTerminator here]</sub> `--`

&nbsp;&nbsp;&nbsp;&nbsp;`++` UnaryExpression<sub>[?Yield, ?Await]</sub>

&nbsp;&nbsp;&nbsp;&nbsp;`--` UnaryExpression<sub>[?Yield, ?Await]</sub>

UnaryExpression<sub>[Yield, Await]</sub> :

&nbsp;&nbsp;&nbsp;&nbsp;UpdateExpression<sub>[?Yield, ?Await]</sub>

&nbsp;&nbsp;&nbsp;&nbsp;`void` UnaryExpression<sub>[?Yield, ?Await]</sub>

&nbsp;&nbsp;&nbsp;&nbsp;`typeof` UnaryExpression<sub>[?Yield, ?Await]</sub>

&nbsp;&nbsp;&nbsp;&nbsp;`+` UnaryExpression<sub>[?Yield, ?Await]</sub>

&nbsp;&nbsp;&nbsp;&nbsp;`-` UnaryExpression<sub>[?Yield, ?Await]</sub>

&nbsp;&nbsp;&nbsp;&nbsp;`~` UnaryExpression<sub>[?Yield, ?Await]</sub>

&nbsp;&nbsp;&nbsp;&nbsp;`!` UnaryExpression<sub>[?Yield, ?Await]</sub>

&nbsp;&nbsp;&nbsp;&nbsp;<sub>[+Await]</sub> AwaitExpression<sub>[?Yield]</sub>

ExponentiationExpression<sub>[Yield, Await]</sub> :

&nbsp;&nbsp;&nbsp;&nbsp;UnaryExpression<sub>[?Yield, ?Await]</sub>

&nbsp;&nbsp;&nbsp;&nbsp;UpdateExpression<sub>[?Yield, ?Await]</sub> `**` ExponentiationExpression<sub>[?Yield, ?Await]</sub>

MultiplicativeExpression<sub>[Yield, Await]</sub> :

&nbsp;&nbsp;&nbsp;&nbsp;ExponentiationExpression<sub>[?Yield, ?Await]</sub>

&nbsp;&nbsp;&nbsp;&nbsp;MultiplicativeExpression<sub>[?Yield, ?Await]</sub> MultiplicativeOperator ExponentiationExpression<sub>[?Yield, ?Await]</sub>

MultiplicativeOperator : **one of**

&nbsp;&nbsp;&nbsp;&nbsp;`*` `/` `%`

AdditiveExpression<sub>[Yield, Await]</sub> :

&nbsp;&nbsp;&nbsp;&nbsp;MultiplicativeExpression<sub>[?Yield, ?Await]</sub>

&nbsp;&nbsp;&nbsp;&nbsp;AdditiveExpression<sub>[?Yield, ?Await]</sub> `+` MultiplicativeExpression<sub>[?Yield, ?Await]</sub>

&nbsp;&nbsp;&nbsp;&nbsp;AdditiveExpression<sub>[?Yield, ?Await]</sub> `-` MultiplicativeExpression<sub>[?Yield, ?Await]</sub>

ShiftExpression<sub>[Yield, Await]</sub> :

&nbsp;&nbsp;&nbsp;&nbsp;AdditiveExpression<sub>[?Yield, ?Await]</sub>

&nbsp;&nbsp;&nbsp;&nbsp;ShiftExpression<sub>[?Yield, ?Await]</sub> `<<` AdditiveExpression<sub>[?Yield, ?Await]</sub>

&nbsp;&nbsp;&nbsp;&nbsp;ShiftExpression<sub>[?Yield, ?Await]</sub> `>>` AdditiveExpression<sub>[?Yield, ?Await]</sub>

&nbsp;&nbsp;&nbsp;&nbsp;ShiftExpression<sub>[?Yield, ?Await]</sub> `>>>` AdditiveExpression<sub>[?Yield, ?Await]</sub>

RelationalExpression<sub>[In, Yield, Await]</sub> :

&nbsp;&nbsp;&nbsp;&nbsp;ShiftExpression<sub>[?Yield, ?Await]</sub>

&nbsp;&nbsp;&nbsp;&nbsp;RelationalExpression<sub>[?In, ?Yield, ?Await]</sub> `<` ShiftExpression<sub>[?Yield, ?Await]</sub>

&nbsp;&nbsp;&nbsp;&nbsp;RelationalExpression<sub>[?In, ?Yield, ?Await]</sub> `>` ShiftExpression<sub>[?Yield, ?Await]</sub>

&nbsp;&nbsp;&nbsp;&nbsp;RelationalExpression<sub>[?In, ?Yield, ?Await]</sub> `<=` ShiftExpression<sub>[?Yield, ?Await]</sub>

&nbsp;&nbsp;&nbsp;&nbsp;RelationalExpression<sub>[?In, ?Yield, ?Await]</sub> `>=` ShiftExpression<sub>[?Yield, ?Await]</sub>

&nbsp;&nbsp;&nbsp;&nbsp;RelationalExpression<sub>[?In, ?Yield, ?Await]</sub> `instanceof` ShiftExpression<sub>[?Yield, ?Await]</sub>

&nbsp;&nbsp;&nbsp;&nbsp;<sub><sub>[+In]</sub></sub> RelationalExpression<sub>[+In, ?Yield, ?Await]</sub> `in` ShiftExpression<sub>[?Yield, ?Await]</sub>

&nbsp;&nbsp;&nbsp;&nbsp;<sub>[+In]</sub> PrivateIdentifier `in` ShiftExpression<sub>[?Yield, ?Await]</sub>

EqualityExpression<sub>[In, Yield, Await]</sub> :

&nbsp;&nbsp;&nbsp;&nbsp;RelationalExpression<sub>[?In, ?Yield, ?Await]</sub>

&nbsp;&nbsp;&nbsp;&nbsp;EqualityExpression<sub>[?In, ?Yield, ?Await]</sub> `===` RelationalExpression<sub>[?In, ?Yield, ?Await]</sub>

&nbsp;&nbsp;&nbsp;&nbsp;EqualityExpression<sub>[?In, ?Yield, ?Await]</sub> `!==` RelationalExpression<sub>[?In, ?Yield, ?Await]</sub>

BitwiseANDExpression<sub>[In, Yield, Await]</sub> :

&nbsp;&nbsp;&nbsp;&nbsp;EqualityExpression<sub>[?In, ?Yield, ?Await]</sub>

&nbsp;&nbsp;&nbsp;&nbsp;BitwiseANDExpression<sub>[?In, ?Yield, ?Await]</sub> `&` EqualityExpression<sub>[?In, ?Yield, ?Await]</sub>

BitwiseXORExpression<sub>[In, Yield, Await]</sub> :

&nbsp;&nbsp;&nbsp;&nbsp;BitwiseANDExpression<sub>[?In, ?Yield, ?Await]</sub>

&nbsp;&nbsp;&nbsp;&nbsp;BitwiseXORExpression<sub>[?In, ?Yield, ?Await]</sub> `^` BitwiseANDExpression<sub>[?In, ?Yield, ?Await]</sub>

BitwiseORExpression<sub>[In, Yield, Await]</sub> :

&nbsp;&nbsp;&nbsp;&nbsp;BitwiseXORExpression<sub>[?In, ?Yield, ?Await]</sub>

&nbsp;&nbsp;&nbsp;&nbsp;BitwiseORExpression<sub>[?In, ?Yield, ?Await]</sub> `|` BitwiseXORExpression<sub>[?In, ?Yield, ?Await]</sub>

LogicalANDExpression<sub>[In, Yield, Await]</sub> :

&nbsp;&nbsp;&nbsp;&nbsp;BitwiseORExpression<sub>[?In, ?Yield, ?Await]</sub>

&nbsp;&nbsp;&nbsp;&nbsp;LogicalANDExpression<sub>[?In, ?Yield, ?Await]</sub> `&&` BitwiseORExpression<sub>[?In, ?Yield, ?Await]</sub>

LogicalORExpression<sub>[In, Yield, Await]</sub> :

&nbsp;&nbsp;&nbsp;&nbsp;LogicalANDExpression<sub>[?In, ?Yield, ?Await]</sub>

&nbsp;&nbsp;&nbsp;&nbsp;LogicalORExpression<sub>[?In, ?Yield, ?Await]</sub> `||` LogicalANDExpression<sub>[?In, ?Yield, ?Await]</sub>

CoalesceExpression<sub>[In, Yield, Await]</sub> :

&nbsp;&nbsp;&nbsp;&nbsp;CoalesceExpressionHead<sub>[?In, ?Yield, ?Await]</sub> `??` BitwiseORExpression<sub>[?In, ?Yield, ?Await]</sub>

CoalesceExpressionHead<sub>[In, Yield, Await]</sub> :

&nbsp;&nbsp;&nbsp;&nbsp;CoalesceExpression<sub>[?In, ?Yield, ?Await]</sub>

&nbsp;&nbsp;&nbsp;&nbsp;BitwiseORExpression<sub>[?In, ?Yield, ?Await]</sub>

ShortCircuitExpression<sub>[In, Yield, Await]</sub> :

&nbsp;&nbsp;&nbsp;&nbsp;LogicalORExpression<sub>[?In, ?Yield, ?Await]</sub>

&nbsp;&nbsp;&nbsp;&nbsp;CoalesceExpression<sub>[?In, ?Yield, ?Await]</sub>

ConditionalExpression<sub>[In, Yield, Await]</sub> :

&nbsp;&nbsp;&nbsp;&nbsp;ShortCircuitExpression<sub>[?In, ?Yield, ?Await]</sub>

&nbsp;&nbsp;&nbsp;&nbsp;ShortCircuitExpression<sub>[?In, ?Yield, ?Await]</sub> `?` AssignmentExpression<sub>[+In, ?Yield, ?Await]</sub> : AssignmentExpression<sub>[?In, ?Yield, ?Await]</sub>

AssignmentExpression<sub>[In, Yield, Await]</sub> :

&nbsp;&nbsp;&nbsp;&nbsp;ConditionalExpression<sub>[?In, ?Yield, ?Await]</sub>

&nbsp;&nbsp;&nbsp;&nbsp;<sub>[+Yield]</sub> YieldExpression<sub>[?In, ?Await]</sub>

&nbsp;&nbsp;&nbsp;&nbsp;ArrowFunction<sub>[?In, ?Yield, ?Await]</sub>

&nbsp;&nbsp;&nbsp;&nbsp;AsyncArrowFunction<sub>[?In, ?Yield, ?Await]</sub>

&nbsp;&nbsp;&nbsp;&nbsp;LeftHandSideExpression<sub>[?Yield, ?Await]</sub> `=` AssignmentExpression<sub>[?In, ?Yield, ?Await]</sub>

&nbsp;&nbsp;&nbsp;&nbsp;LeftHandSideExpression<sub>[?Yield, ?Await]</sub> AssignmentOperator AssignmentExpression<sub>[?In, ?Yield, ?Await]</sub>

&nbsp;&nbsp;&nbsp;&nbsp;LeftHandSideExpression<sub>[?Yield, ?Await]</sub> `&&=` AssignmentExpression<sub>[?In, ?Yield, ?Await]</sub>

&nbsp;&nbsp;&nbsp;&nbsp;LeftHandSideExpression<sub>[?Yield, ?Await]</sub> `||=` AssignmentExpression<sub>[?In, ?Yield, ?Await]</sub>

&nbsp;&nbsp;&nbsp;&nbsp;LeftHandSideExpression<sub>[?Yield, ?Await]</sub> `??=` AssignmentExpression<sub>[?In, ?Yield, ?Await]</sub>

AssignmentOperator : **one of**

&nbsp;&nbsp;&nbsp;&nbsp;`*=` `/=` `%=` `+=` `-=` `<<=` `>>=` `>>>=` `&=` `^=` `|=` `**=`

In certain circumstances when processing an instance of the production

AssignmentExpression<sub>[In, Yield, Await]</sub> : LeftHandSideExpression<sub>[?Yield, ?Await]</sub> `=` AssignmentExpression<sub>[?In, ?Yield, ?Await]</sub>

the interpretation of LeftHandSideExpression is refined using the following grammar:

AssignmentPattern<sub>[Yield, Await]</sub> :

&nbsp;&nbsp;&nbsp;&nbsp;ObjectAssignmentPattern<sub>[?Yield, ?Await]</sub>

&nbsp;&nbsp;&nbsp;&nbsp;ArrayAssignmentPattern<sub>[?Yield, ?Await]</sub>

ObjectAssignmentPattern<sub>[Yield, Await]</sub> :

&nbsp;&nbsp;&nbsp;&nbsp;`{` `}`

&nbsp;&nbsp;&nbsp;&nbsp;`{` AssignmentRestProperty<sub>[?Yield, ?Await]</sub> `}`

&nbsp;&nbsp;&nbsp;&nbsp;`{` AssignmentPropertyList<sub>[?Yield, ?Await]</sub> `}`

&nbsp;&nbsp;&nbsp;&nbsp;`{` AssignmentPropertyList<sub>[?Yield, ?Await]</sub> `,` AssignmentRestProperty<sub>[?Yield, ?Await]</sub>opt `}`

ArrayAssignmentPattern<sub>[Yield, Await]</sub> :

&nbsp;&nbsp;&nbsp;&nbsp;`<sub>[` Elisionopt AssignmentRestElement[?Yield, ?Await]</sub>opt `]`

&nbsp;&nbsp;&nbsp;&nbsp;`[` AssignmentElementList[?Yield, ?Await] `]`

&nbsp;&nbsp;&nbsp;&nbsp;`[` AssignmentElementList[?Yield, ?Await] `,` Elisionopt AssignmentRestElement<sub>[?Yield, ?Await]</sub>opt `]`

AssignmentRestProperty<sub>[Yield, Await]</sub> :

&nbsp;&nbsp;&nbsp;&nbsp;`...` DestructuringAssignmentTarget<sub>[?Yield, ?Await]</sub>

AssignmentPropertyList<sub>[Yield, Await]</sub> :

&nbsp;&nbsp;&nbsp;&nbsp;AssignmentProperty<sub>[?Yield, ?Await]</sub>

&nbsp;&nbsp;&nbsp;&nbsp;AssignmentPropertyList<sub>[?Yield, ?Await]</sub> `,` AssignmentProperty<sub>[?Yield, ?Await]</sub>

AssignmentElementList<sub>[Yield, Await]</sub> :

&nbsp;&nbsp;&nbsp;&nbsp;AssignmentElisionElement<sub>[?Yield, ?Await]</sub>

&nbsp;&nbsp;&nbsp;&nbsp;AssignmentElementList<sub>[?Yield, ?Await]</sub> `,` AssignmentElisionElement<sub>[?Yield, ?Await]</sub>

AssignmentElisionElement<sub>[Yield, Await]</sub> :

&nbsp;&nbsp;&nbsp;&nbsp;Elisionopt AssignmentElement<sub>[?Yield, ?Await]</sub>

AssignmentProperty<sub>[Yield, Await]</sub> :

I&nbsp;&nbsp;&nbsp;&nbsp;dentifierReference<sub>[?Yield, ?Await]</sub> Initializer<sub>[+In, ?Yield, ?Await]</sub>opt

&nbsp;&nbsp;&nbsp;&nbsp;PropertyName<sub>[?Yield, ?Await]</sub> `:` AssignmentElement<sub>[?Yield, ?Await]</sub>

AssignmentElement<sub>[Yield, Await]</sub> :

&nbsp;&nbsp;&nbsp;&nbsp;DestructuringAssignmentTarget<sub>[?Yield, ?Await]</sub> Initializer<sub>[+In, ?Yield, ?Await]</sub>opt

AssignmentRestElement<sub>[Yield, Await]</sub> :

&nbsp;&nbsp;&nbsp;&nbsp;`...` DestructuringAssignmentTarget<sub>[?Yield, ?Await]</sub>

DestructuringAssignmentTarget<sub>[Yield, Await]</sub> :

&nbsp;&nbsp;&nbsp;&nbsp;LeftHandSideExpression<sub>[?Yield, ?Await]</sub>

Expression<sub>[In, Yield, Await]</sub> :

&nbsp;&nbsp;&nbsp;&nbsp;AssignmentExpression<sub>[?In, ?Yield, ?Await]</sub>

&nbsp;&nbsp;&nbsp;&nbsp;Expression<sub>[?In, ?Yield, ?Await]</sub> `,` AssignmentExpression<sub>[?In, ?Yield, ?Await]</sub>
