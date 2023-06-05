## Statements

**Statement[Yield, Await, Return] :**

&ensp;&ensp;BlockStatement[?Yield, ?Await, ?Return]

~~&ensp;&ensp;VariableStatement[?Yield, ?Await]~~

~~&ensp;&ensp;EmptyStatement~~

&ensp;&ensp;ExpressionStatement[?Yield, ?Await]

&ensp;&ensp;IfStatement[?Yield, ?Await, ?Return]

&ensp;&ensp;BreakableStatement[?Yield, ?Await, ?Return]

&ensp;&ensp;ContinueStatement[?Yield, ?Await]

&ensp;&ensp;BreakStatement[?Yield, ?Await]

&ensp;&ensp;[+Return] ReturnStatement[?Yield, ?Await]

~~&ensp;&ensp;WithStatement[?Yield, ?Await, ?Return]~~

~~&ensp;&ensp;LabelledStatement[?Yield, ?Await, ?Return]~~

&ensp;&ensp;ThrowStatement[?Yield, ?Await]

&ensp;&ensp;TryStatement[?Yield, ?Await, ?Return]

&ensp;&ensp;DebuggerStatement

<br>    

**Declaration[Yield, Await] :**

&ensp;&ensp;HoistableDeclaration[?Yield, ?Await, ~Default]

&ensp;&ensp;~~ClassDeclaration[?Yield, ?Await, ~Default]~~

&ensp;&ensp;LexicalDeclaration[+In, ?Yield, ?Await]

<br>    

**HoistableDeclaration[Yield, Await, Default] :**

&ensp;&ensp;FunctionDeclaration[?Yield, ?Await, ?Default]

&ensp;&ensp;GeneratorDeclaration[?Yield, ?Await, ?Default]

&ensp;&ensp;AsyncFunctionDeclaration[?Yield, ?Await, ?Default]

&ensp;&ensp;AsyncGeneratorDeclaration[?Yield, ?Await, ?Default]

<br>    

**BreakableStatement[Yield, Await, Return] :**

&ensp;&ensp;IterationStatement[?Yield, ?Await, ?Return]

&ensp;&ensp;SwitchStatement[?Yield, ?Await, ?Return]

<br>    

**BlockStatement[Yield, Await, Return] :**

&ensp;&ensp;Block[?Yield, ?Await, ?Return]

<br>    

**Block[Yield, Await, Return] :**

&ensp;&ensp;{ StatementList[?Yield, ?Await, ?Return]opt }

<br>    

**StatementList[Yield, Await, Return] :**

&ensp;&ensp;StatementListItem[?Yield, ?Await, ?Return]

&ensp;&ensp;StatementList[?Yield, ?Await, ?Return] StatementListItem[?Yield, ?Await, ?Return]




<br>    

**StatementListItem[Yield, Await, Return] :**

&ensp;&ensp;Statement[?Yield, ?Await, ?Return]

&ensp;&ensp;Declaration[?Yield, ?Await]




<br>    

**LexicalDeclaration[In, Yield, Await] :**

&ensp;&ensp;LetOrConst BindingList[?In, ?Yield, ?Await] ;




<br>    

**LetOrConst :**

&ensp;&ensp;let

&ensp;&ensp;const




<br>    

**BindingList[In, Yield, Await] :**

&ensp;&ensp;LexicalBinding[?In, ?Yield, ?Await]

&ensp;&ensp;BindingList[?In, ?Yield, ?Await] , LexicalBinding[?In, ?Yield, ?Await]




<br>    

**LexicalBinding[In, Yield, Await] :**

&ensp;&ensp;BindingIdentifier[?Yield, ?Await] Initializer[?In, ?Yield, ?Await]opt

&ensp;&ensp;BindingPattern[?Yield, ?Await] Initializer[?In, ?Yield, ?Await]




<br>    

**VariableStatement[Yield, Await] :**

&ensp;&ensp;var VariableDeclarationList[+In, ?Yield, ?Await] ;




<br>    

**VariableDeclarationList[In, Yield, Await] :**

&ensp;&ensp;VariableDeclaration[?In, ?Yield, ?Await]

&ensp;&ensp;VariableDeclarationList[?In, ?Yield, ?Await] , VariableDeclaration[?In, ?Yield, ?Await]




<br>    

**VariableDeclaration[In, Yield, Await] :**

&ensp;&ensp;BindingIdentifier[?Yield, ?Await] Initializer[?In, ?Yield, ?Await]opt

&ensp;&ensp;BindingPattern[?Yield, ?Await] Initializer[?In, ?Yield, ?Await]




<br>    

**BindingPattern[Yield, Await] :**

&ensp;&ensp;ObjectBindingPattern[?Yield, ?Await]

&ensp;&ensp;ArrayBindingPattern[?Yield, ?Await]




<br>    

**ObjectBindingPattern[Yield, Await] :**

&ensp;&ensp;{ }

&ensp;&ensp;{ BindingRestProperty[?Yield, ?Await] }

&ensp;&ensp;{ BindingPropertyList[?Yield, ?Await] }

&ensp;&ensp;{ BindingPropertyList[?Yield, ?Await] , BindingRestProperty[?Yield, ?Await]opt }




<br>    

**ArrayBindingPattern[Yield, Await] :**

&ensp;&ensp;[ Elisionopt BindingRestElement[?Yield, ?Await]opt ]

&ensp;&ensp;[ BindingElementList[?Yield, ?Await] ]

&ensp;&ensp;[ BindingElementList[?Yield, ?Await] , Elisionopt BindingRestElement[?Yield, ?Await]opt ]




<br>    

**BindingRestProperty[Yield, Await] :**

&ensp;&ensp;... BindingIdentifier[?Yield, ?Await]




<br>    

**BindingPropertyList[Yield, Await] :**

&ensp;&ensp;BindingProperty[?Yield, ?Await]

&ensp;&ensp;BindingPropertyList[?Yield, ?Await] , BindingProperty[?Yield, ?Await]




<br>    

**BindingElementList[Yield, Await] :**

&ensp;&ensp;BindingElisionElement[?Yield, ?Await]

&ensp;&ensp;BindingElementList[?Yield, ?Await] , BindingElisionElement[?Yield, ?Await]




<br>    

**BindingElisionElement[Yield, Await] :**

&ensp;&ensp;Elisionopt BindingElement[?Yield, ?Await]




<br>    

**BindingProperty[Yield, Await] :**

&ensp;&ensp;SingleNameBinding[?Yield, ?Await]

&ensp;&ensp;PropertyName[?Yield, ?Await] : BindingElement[?Yield, ?Await]




<br>    

**BindingElement[Yield, Await] :**

&ensp;&ensp;SingleNameBinding[?Yield, ?Await]

&ensp;&ensp;BindingPattern[?Yield, ?Await] Initializer[+In, ?Yield, ?Await]opt




<br>    

**SingleNameBinding[Yield, Await] :**

&ensp;&ensp;BindingIdentifier[?Yield, ?Await] Initializer[+In, ?Yield, ?Await]opt




<br>    

**BindingRestElement[Yield, Await] :**

&ensp;&ensp;... BindingIdentifier[?Yield, ?Await]

&ensp;&ensp;... BindingPattern[?Yield, ?Await]




<br>    

**EmptyStatement :**

&ensp;&ensp;;




<br>    

**ExpressionStatement[Yield, Await] :**

&ensp;&ensp;[lookahead ∉ { {, function, async [no LineTerminator here] function, class, let [ }] Expression[+In, ?Yield, ?Await] ;




<br>    

**IfStatement[Yield, Await, Return] :**

&ensp;&ensp;if ( Expression[+In, ?Yield, ?Await] ) Statement[?Yield, ?Await, ?Return] else Statement[?Yield, ?Await, ?Return]

&ensp;&ensp;if ( Expression[+In, ?Yield, ?Await] ) Statement[?Yield, ?Await, ?Return] [lookahead ≠ else]




<br>    

**IterationStatement[Yield, Await, Return] :**

&ensp;&ensp;DoWhileStatement[?Yield, ?Await, ?Return]

&ensp;&ensp;WhileStatement[?Yield, ?Await, ?Return]

&ensp;&ensp;ForStatement[?Yield, ?Await, ?Return]

&ensp;&ensp;ForInOfStatement[?Yield, ?Await, ?Return]




<br>    

**DoWhileStatement[Yield, Await, Return] :**

&ensp;&ensp;do Statement[?Yield, ?Await, ?Return] while ( Expression[+In, ?Yield, ?Await] ) ;




<br>    

**WhileStatement[Yield, Await, Return] :**

&ensp;&ensp;while ( Expression[+In, ?Yield, ?Await] ) Statement[?Yield, ?Await, ?Return]




<br>    

**ForStatement[Yield, Await, Return] :**

&ensp;&ensp;for ( [lookahead ≠ let [] Expression[~In, ?Yield, ?Await]opt ; Expression[+In, ?Yield, ?Await]opt ; Expression[+In, ?Yield, ?Await]opt ) Statement[?Yield, ?Await, ?Return]

&ensp;&ensp;for ( var VariableDeclarationList[~In, ?Yield, ?Await] ; Expression[+In, ?Yield, ?Await]opt ; Expression[+In, ?Yield, ?Await]opt ) Statement[?Yield, ?Await, ?Return]

&ensp;&ensp;for ( LexicalDeclaration[~In, ?Yield, ?Await] Expression[+In, ?Yield, ?Await]opt ; Expression[+In, ?Yield, ?Await]opt ) Statement[?Yield, ?Await, ?Return]




<br>    

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




<br>    

**ForDeclaration[Yield, Await] :**

&ensp;&ensp;LetOrConst ForBinding[?Yield, ?Await]




<br>    

**ForBinding[Yield, Await] :**

&ensp;&ensp;BindingIdentifier[?Yield, ?Await]

&ensp;&ensp;BindingPattern[?Yield, ?Await]




<br>    

**ContinueStatement[Yield, Await] :**

&ensp;&ensp;continue ;

&ensp;&ensp;continue [no LineTerminator here] LabelIdentifier[?Yield, ?Await] ;




<br>    

**BreakStatement[Yield, Await] :**

&ensp;&ensp;break ;

&ensp;&ensp;break [no LineTerminator here] LabelIdentifier[?Yield, ?Await] ;




<br>    

**ReturnStatement[Yield, Await] :**

&ensp;&ensp;return ;

&ensp;&ensp;return [no LineTerminator here] Expression[+In, ?Yield, ?Await] ;




<br>    

**WithStatement[Yield, Await, Return] :**

&ensp;&ensp;with ( Expression[+In, ?Yield, ?Await] ) Statement[?Yield, ?Await, ?Return]




<br>    

**SwitchStatement[Yield, Await, Return] :**

&ensp;&ensp;switch ( Expression[+In, ?Yield, ?Await] ) CaseBlock[?Yield, ?Await, ?Return]




<br>    

**CaseBlock[Yield, Await, Return] :**

&ensp;&ensp;{ CaseClauses[?Yield, ?Await, ?Return]opt }

&ensp;&ensp;{ CaseClauses[?Yield, ?Await, ?Return]opt DefaultClause[?Yield, ?Await, ?Return] CaseClauses[?Yield, ?Await, ?Return]opt }

&ensp;&ensp;CaseClauses[Yield, Await, Return] :




<br>    

**CaseClause[?Yield, ?Await, ?Return]**

&ensp;&ensp;CaseClauses[?Yield, ?Await, ?Return] CaseClause[?Yield, ?Await, ?Return]




<br>    

**CaseClause[Yield, Await, Return] :**

&ensp;&ensp;case Expression[+In, ?Yield, ?Await] : StatementList[?Yield, ?Await, ?Return]opt




<br>    

**DefaultClause[Yield, Await, Return] :**

&ensp;&ensp;default : StatementList[?Yield, ?Await, ?Return]opt




<br>    

**LabelledStatement[Yield, Await, Return] :**

&ensp;&ensp;LabelIdentifier[?Yield, ?Await] : LabelledItem[?Yield, ?Await, ?Return]




<br>    

**LabelledItem[Yield, Await, Return] :**

&ensp;&ensp;Statement[?Yield, ?Await, ?Return]

&ensp;&ensp;FunctionDeclaration[?Yield, ?Await, ~Default]




<br>    

**ThrowStatement[Yield, Await] :**

&ensp;&ensp;throw [no LineTerminator here] Expression[+In, ?Yield, ?Await] ;




<br>    

**TryStatement[Yield, Await, Return] :**

&ensp;&ensp;try Block[?Yield, ?Await, ?Return] Catch[?Yield, ?Await, ?Return]

&ensp;&ensp;try Block[?Yield, ?Await, ?Return] Finally[?Yield, ?Await, ?Return]

&ensp;&ensp;try Block[?Yield, ?Await, ?Return] Catch[?Yield, ?Await, ?Return] Finally[?Yield, ?Await, ?Return]




<br>    

**Catch[Yield, Await, Return] :**

&ensp;&ensp;catch ( CatchParameter[?Yield, ?Await] ) Block[?Yield, ?Await, ?Return]

&ensp;&ensp;catch Block[?Yield, ?Await, ?Return]




<br>    

**Finally[Yield, Await, Return] :**

&ensp;&ensp;finally Block[?Yield, ?Await, ?Return]




<br>    

**CatchParameter[Yield, Await] :**

&ensp;&ensp;BindingIdentifier[?Yield, ?Await]

&ensp;&ensp;BindingPattern[?Yield, ?Await]




<br>    

**DebuggerStatement :**

&ensp;&ensp; debugger ;


## Functions and Classes


<br>    

**UniqueFormalParameters[Yield, Await] :**

&ensp;&ensp;FormalParameters[?Yield, ?Await]


<br>    

**FormalParameters[Yield, Await] :**

&ensp;&ensp; [empty]

&ensp;&ensp; FunctionRestParameter[?Yield, ?Await]

&ensp;&ensp; FormalParameterList[?Yield, ?Await]

&ensp;&ensp; FormalParameterList[?Yield, ?Await] ,

&ensp;&ensp; FormalParameterList[?Yield, ?Await] , FunctionRestParameter[?Yield, ?Await]

&ensp;&ensp; FormalParameterList[Yield, Await] :

&ensp;&ensp; FormalParameter[?Yield, ?Await]


<br>    

**FormalParameterList[?Yield, ?Await] , FormalParameter[?Yield, ?Await]**

&ensp;&ensp; FunctionRestParameter[Yield, Await] :

&ensp;&ensp; BindingRestElement[?Yield, ?Await]


<br>    

**FormalParameter[Yield, Await] :**

&ensp;&ensp; BindingElement[?Yield, ?Await]


<br>    

**FunctionDeclaration[Yield, Await, Default] :**

&ensp;&ensp; function BindingIdentifier[?Yield, ?Await] ( FormalParameters[~Yield, ~Await] ) { FunctionBody[~Yield, ~Await] }

&ensp;&ensp; [+Default] function ( FormalParameters[~Yield, ~Await] ) { FunctionBody[~Yield, ~Await] }


<br>    

**FunctionExpression :**

&ensp;&ensp; function BindingIdentifier[~Yield, ~Await]opt ( FormalParameters[~Yield, ~Await] ) { FunctionBody[~Yield, ~Await] }


<br>    

**FunctionBody[Yield, Await] :**

&ensp;&ensp; FunctionStatementList[?Yield, ?Await]


<br>    

**FunctionStatementList[Yield, Await] :**

&ensp;&ensp; StatementList[?Yield, ?Await, +Return]opt


<br>    

**ArrowFunction[In, Yield, Await] :**

&ensp;&ensp; ArrowParameters[?Yield, ?Await] [no LineTerminator here] => ConciseBody[?In]


<br>    

**ArrowParameters[Yield, Await] :**

&ensp;&ensp; BindingIdentifier[?Yield, ?Await]

&ensp;&ensp; CoverParenthesizedExpressionAndArrowParameterList[?Yield, ?Await]


<br>    

**ConciseBody[In] :**

&ensp;&ensp; [lookahead ≠ {] ExpressionBody[?In, ~Await]

&ensp;&ensp; { FunctionBody[~Yield, ~Await] }


<br>    

**ExpressionBody[In, Await] :**

&ensp;&ensp; AssignmentExpression[?In, ~Yield, ?Await]



When processing an instance of the production

ArrowParameters[Yield, Await] : CoverParenthesizedExpressionAndArrowParameterList[?Yield, ?Await]

the interpretation of CoverParenthesizedExpressionAndArrowParameterList is refined using the following grammar:




<br>    

**ArrowFormalParameters[Yield, Await] :**

&ensp;&ensp; ( UniqueFormalParameters[?Yield, ?Await] )

 




<br>    

**AsyncArrowFunction[In, Yield, Await] :**

&ensp;&ensp; async [no LineTerminator here] AsyncArrowBindingIdentifier[?Yield] [no LineTerminator here] => AsyncConciseBody[?In]

&ensp;&ensp; CoverCallExpressionAndAsyncArrowHead[?Yield, ?Await] [no LineTerminator here] => AsyncConciseBody[?In]


<br>    

**AsyncConciseBody[In] :**

&ensp;&ensp; [lookahead ≠ {] ExpressionBody[?In, +Await]

&ensp;&ensp; { AsyncFunctionBody }


<br>    

**AsyncArrowBindingIdentifier[Yield] :**

&ensp;&ensp; BindingIdentifier[?Yield, +Await]


<br>    

**CoverCallExpressionAndAsyncArrowHead[Yield, Await] :**

&ensp;&ensp; MemberExpression[?Yield, ?Await] Arguments[?Yield, ?Await]



When processing an instance of the production

AsyncArrowFunction[In, Yield, Await] : CoverCallExpressionAndAsyncArrowHead[?Yield, ?Await] [no LineTerminator here] => AsyncConciseBody[?In]

the interpretation of CoverCallExpressionAndAsyncArrowHead is refined using the following grammar:




<br>    

**AsyncArrowHead :**

&ensp;&ensp; async [no LineTerminator here] ArrowFormalParameters[~Yield, +Await]

 




<br>    

**MethodDefinition[Yield, Await] :**

&ensp;&ensp; ClassElementName[?Yield, ?Await] ( UniqueFormalParameters[~Yield, ~Await] ) { FunctionBody[~Yield, ~Await] }

&ensp;&ensp; GeneratorMethod[?Yield, ?Await]

&ensp;&ensp; AsyncMethod[?Yield, ?Await]

&ensp;&ensp; AsyncGeneratorMethod[?Yield, ?Await]

&ensp;&ensp; get ClassElementName[?Yield, ?Await] ( ) { FunctionBody[~Yield, ~Await] }

&ensp;&ensp; set ClassElementName[?Yield, ?Await] ( PropertySetParameterList ) { FunctionBody[~Yield, ~Await] }


<br>    

**PropertySetParameterList :**

&ensp;&ensp; FormalParameter[~Yield, ~Await]


<br>    

**GeneratorDeclaration[Yield, Await, Default] :**

&ensp;&ensp; function * BindingIdentifier[?Yield, ?Await] ( FormalParameters[+Yield, ~Await] ) { GeneratorBody }

[+Default] function * ( FormalParameters[+Yield, ~Await] ) { GeneratorBody }

&ensp;&ensp; **GeneratorExpression :**

&ensp;&ensp; function * BindingIdentifier[+Yield, ~Await]opt ( FormalParameters[+Yield, ~Await] ) { GeneratorBody }


<br>    

**GeneratorMethod[Yield, Await] :**

&ensp;&ensp; * ClassElementName[?Yield, ?Await] ( UniqueFormalParameters[+Yield, ~Await] ) { GeneratorBody }


<br>    

**GeneratorBody :**

&ensp;&ensp; FunctionBody[+Yield, ~Await]


<br>    

**YieldExpression[In, Await] :**

&ensp;&ensp; yield

&ensp;&ensp; yield [no LineTerminator here] AssignmentExpression[?In, +Yield, ?Await]

&ensp;&ensp; yield [no LineTerminator here] * AssignmentExpression[?In, +Yield, ?Await]

&ensp;&ensp; **AsyncGeneratorDeclaration[Yield, Await, Default] :**

&ensp;&ensp; async [no LineTerminator here] function * BindingIdentifier[?Yield, ?Await] ( FormalParameters[+Yield, +Await] ) { AsyncGeneratorBody }

&ensp;&ensp; [+Default] async [no LineTerminator here] function * ( FormalParameters[+Yield, +Await] ) { AsyncGeneratorBody }


<br>    

**AsyncGeneratorExpression :**

&ensp;&ensp; async [no LineTerminator here] function * BindingIdentifier[+Yield, +Await]opt ( FormalParameters[+Yield, +Await] ) { AsyncGeneratorBody }


<br>    

**AsyncGeneratorMethod[Yield, Await] :**

&ensp;&ensp; async [no LineTerminator here] * ClassElementName[?Yield, ?Await] ( UniqueFormalParameters[+Yield, +Await] ) { AsyncGeneratorBody }


<br>    

**AsyncGeneratorBody :**

&ensp;&ensp; FunctionBody[+Yield, +Await]


<br>    

**AsyncFunctionDeclaration[Yield, Await, Default] :**

&ensp;&ensp; async [no LineTerminator here] function BindingIdentifier[?Yield, ?Await] ( FormalParameters[~Yield, +Await] ) { AsyncFunctionBody }

&ensp;&ensp; [+Default] async [no LineTerminator here] function ( FormalParameters[~Yield, +Await] ) { AsyncFunctionBody }


<br>    

**AsyncFunctionExpression :**

&ensp;&ensp; async [no LineTerminator here] function BindingIdentifier[~Yield, +Await]opt ( FormalParameters[~Yield, +Await] ) { AsyncFunctionBody }


<br>    

**AsyncMethod[Yield, Await] :**

&ensp;&ensp; async [no LineTerminator here] ClassElementName[?Yield, ?Await] ( UniqueFormalParameters[~Yield, +Await] ) { AsyncFunctionBody }


<br>    

**AsyncFunctionBody :**

&ensp;&ensp; FunctionBody[~Yield, +Await]


<br>    

**AwaitExpression[Yield] :**

&ensp;&ensp; await UnaryExpression[?Yield, +Await]


<br>    

**ClassDeclaration[Yield, Await, Default] :**

&ensp;&ensp; class BindingIdentifier[?Yield, ?Await] ClassTail[?Yield, ?Await]

&ensp;&ensp; [+Default] class ClassTail[?Yield, ?Await]


<br>    

**ClassExpression[Yield, Await] :**

&ensp;&ensp; class BindingIdentifier[?Yield, ?Await]opt ClassTail[?Yield, ?Await]


<br>    

**ClassTail[Yield, Await] :**

&ensp;&ensp; ClassHeritage[?Yield, ?Await]opt { ClassBody[?Yield, ?Await]opt }


<br>    

**ClassHeritage[Yield, Await] :**

&ensp;&ensp; extends LeftHandSideExpression[?Yield, ?Await]


<br>    

**ClassBody[Yield, Await] :**

&ensp;&ensp; ClassElementList[?Yield, ?Await]


<br>    

**ClassElementList[Yield, Await] :**

&ensp;&ensp; ClassElement[?Yield, ?Await]

&ensp;&ensp; ClassElementList[?Yield, ?Await] ClassElement[?Yield, ?Await]


<br>    

**ClassElement[Yield, Await] :**

&ensp;&ensp; MethodDefinition[?Yield, ?Await]

&ensp;&ensp; static MethodDefinition[?Yield, ?Await]

&ensp;&ensp; FieldDefinition[?Yield, ?Await] ;

&ensp;&ensp; static FieldDefinition[?Yield, ?Await] ;

&ensp;&ensp; ClassStaticBlock

&ensp;&ensp; ;


<br>    

**FieldDefinition[Yield, Await] :**

&ensp;&ensp; ClassElementName[?Yield, ?Await] Initializer[+In, ?Yield, ?Await]opt


<br>    

**ClassElementName[Yield, Await] :**

&ensp;&ensp; PropertyName[?Yield, ?Await]

&ensp;&ensp; PrivateIdentifier


<br>    

**ClassStaticBlock :**

&ensp;&ensp; static { ClassStaticBlockBody }


<br>    

**ClassStaticBlockBody :**

&ensp;&ensp; ClassStaticBlockStatementList


<br>    

**ClassStaticBlockStatementList :**

&ensp;&ensp; StatementList[~Yield, +Await, ~Return]opt