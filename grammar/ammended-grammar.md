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

~~&ensp;&ensp;ClassDeclaration[?Yield, ?Await, ~Default]~~

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



**Block[Yield, Await, Return] :**

&ensp;&ensp;{ StatementList[?Yield, ?Await, ?Return]opt }



**StatementList[Yield, Await, Return] :**

&ensp;&ensp;StatementListItem[?Yield, ?Await, ?Return]

&ensp;&ensp;StatementList[?Yield, ?Await, ?Return] StatementListItem[?Yield, ?Await, ?Return]



**StatementListItem[Yield, Await, Return] :**

&ensp;&ensp;Statement[?Yield, ?Await, ?Return]

&ensp;&ensp;Declaration[?Yield, ?Await]



**LexicalDeclaration[In, Yield, Await] :**

&ensp;&ensp;LetOrConst BindingList[?In, ?Yield, ?Await] ;



**LetOrConst :**

&ensp;&ensp;let

&ensp;&ensp;const



**BindingList[In, Yield, Await] :**

&ensp;&ensp;LexicalBinding[?In, ?Yield, ?Await]

&ensp;&ensp;BindingList[?In, ?Yield, ?Await] , LexicalBinding[?In, ?Yield, ?Await]



**LexicalBinding[In, Yield, Await] :**

&ensp;&ensp;BindingIdentifier[?Yield, ?Await] Initializer[?In, ?Yield, ?Await]opt

&ensp;&ensp;BindingPattern[?Yield, ?Await] Initializer[?In, ?Yield, ?Await]



**VariableStatement[Yield, Await] :**

&ensp;&ensp;var VariableDeclarationList[+In, ?Yield, ?Await] ;



**VariableDeclarationList[In, Yield, Await] :**

&ensp;&ensp;VariableDeclaration[?In, ?Yield, ?Await]

&ensp;&ensp;VariableDeclarationList[?In, ?Yield, ?Await] , VariableDeclaration[?In, ?Yield, ?Await]



**VariableDeclaration[In, Yield, Await] :**

&ensp;&ensp;BindingIdentifier[?Yield, ?Await] Initializer[?In, ?Yield, ?Await]opt

&ensp;&ensp;BindingPattern[?Yield, ?Await] Initializer[?In, ?Yield, ?Await]



**BindingPattern[Yield, Await] :**

&ensp;&ensp;ObjectBindingPattern[?Yield, ?Await]

&ensp;&ensp;ArrayBindingPattern[?Yield, ?Await]



**ObjectBindingPattern[Yield, Await] :**

&ensp;&ensp;{ }

&ensp;&ensp;{ BindingRestProperty[?Yield, ?Await] }

&ensp;&ensp;{ BindingPropertyList[?Yield, ?Await] }

&ensp;&ensp;{ BindingPropertyList[?Yield, ?Await] , BindingRestProperty[?Yield, ?Await]opt }



**ArrayBindingPattern[Yield, Await] :**

&ensp;&ensp;[ Elisionopt BindingRestElement[?Yield, ?Await]opt ]

&ensp;&ensp;[ BindingElementList[?Yield, ?Await] ]

&ensp;&ensp;[ BindingElementList[?Yield, ?Await] , Elisionopt BindingRestElement[?Yield, ?Await]opt ]



**BindingRestProperty[Yield, Await] :**

&ensp;&ensp;... BindingIdentifier[?Yield, ?Await]



**BindingPropertyList[Yield, Await] :**

&ensp;&ensp;BindingProperty[?Yield, ?Await]

&ensp;&ensp;BindingPropertyList[?Yield, ?Await] , BindingProperty[?Yield, ?Await]



**BindingElementList[Yield, Await] :**

&ensp;&ensp;BindingElisionElement[?Yield, ?Await]

&ensp;&ensp;BindingElementList[?Yield, ?Await] , BindingElisionElement[?Yield, ?Await]



**BindingElisionElement[Yield, Await] :**

&ensp;&ensp;Elisionopt BindingElement[?Yield, ?Await]



**BindingProperty[Yield, Await] :**

&ensp;&ensp;SingleNameBinding[?Yield, ?Await]

&ensp;&ensp;PropertyName[?Yield, ?Await] : BindingElement[?Yield, ?Await]



**BindingElement[Yield, Await] :**

&ensp;&ensp;SingleNameBinding[?Yield, ?Await]

&ensp;&ensp;BindingPattern[?Yield, ?Await] Initializer[+In, ?Yield, ?Await]opt



**SingleNameBinding[Yield, Await] :**

&ensp;&ensp;BindingIdentifier[?Yield, ?Await] Initializer[+In, ?Yield, ?Await]opt



**BindingRestElement[Yield, Await] :**

&ensp;&ensp;... BindingIdentifier[?Yield, ?Await]

&ensp;&ensp;... BindingPattern[?Yield, ?Await]



**EmptyStatement :**

&ensp;&ensp;;



**ExpressionStatement[Yield, Await] :**

&ensp;&ensp;[lookahead ∉ { {, function, async [no LineTerminator here] function, class, let [ }] Expression[+In, ?Yield, ?Await] ;



**IfStatement[Yield, Await, Return] :**

&ensp;&ensp;if ( Expression[+In, ?Yield, ?Await] ) Statement[?Yield, ?Await, ?Return] else Statement[?Yield, ?Await, ?Return]

&ensp;&ensp;if ( Expression[+In, ?Yield, ?Await] ) Statement[?Yield, ?Await, ?Return] [lookahead ≠ else]



**IterationStatement[Yield, Await, Return] :**

&ensp;&ensp;DoWhileStatement[?Yield, ?Await, ?Return]

&ensp;&ensp;WhileStatement[?Yield, ?Await, ?Return]

&ensp;&ensp;ForStatement[?Yield, ?Await, ?Return]

&ensp;&ensp;ForInOfStatement[?Yield, ?Await, ?Return]



**DoWhileStatement[Yield, Await, Return] :**

&ensp;&ensp;do Statement[?Yield, ?Await, ?Return] while ( Expression[+In, ?Yield, ?Await] ) ;



**WhileStatement[Yield, Await, Return] :**

&ensp;&ensp;while ( Expression[+In, ?Yield, ?Await] ) Statement[?Yield, ?Await, ?Return]



**ForStatement[Yield, Await, Return] :**

&ensp;&ensp;for ( [lookahead ≠ let [] Expression[~In, ?Yield, ?Await]opt ; Expression[+In, ?Yield, ?Await]opt ; Expression[+In, ?Yield, ?Await]opt ) Statement[?Yield, ?Await, ?Return]

&ensp;&ensp;for ( var VariableDeclarationList[~In, ?Yield, ?Await] ; Expression[+In, ?Yield, ?Await]opt ; Expression[+In, ?Yield, ?Await]opt ) Statement[?Yield, ?Await, ?Return]

&ensp;&ensp;for ( LexicalDeclaration[~In, ?Yield, ?Await] Expression[+In, ?Yield, ?Await]opt ; Expression[+In, ?Yield, ?Await]opt ) Statement[?Yield, ?Await, ?Return]



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



**ForDeclaration[Yield, Await] :**

&ensp;&ensp;LetOrConst ForBinding[?Yield, ?Await]



**ForBinding[Yield, Await] :**

&ensp;&ensp;BindingIdentifier[?Yield, ?Await]

&ensp;&ensp;BindingPattern[?Yield, ?Await]



**ContinueStatement[Yield, Await] :**

&ensp;&ensp;continue ;

&ensp;&ensp;continue [no LineTerminator here] LabelIdentifier[?Yield, ?Await] ;



**BreakStatement[Yield, Await] :**

&ensp;&ensp;break ;

&ensp;&ensp;break [no LineTerminator here] LabelIdentifier[?Yield, ?Await] ;



**ReturnStatement[Yield, Await] :**

&ensp;&ensp;return ;

&ensp;&ensp;return [no LineTerminator here] Expression[+In, ?Yield, ?Await] ;



**WithStatement[Yield, Await, Return] :**

&ensp;&ensp;with ( Expression[+In, ?Yield, ?Await] ) Statement[?Yield, ?Await, ?Return]



**SwitchStatement[Yield, Await, Return] :**

&ensp;&ensp;switch ( Expression[+In, ?Yield, ?Await] ) CaseBlock[?Yield, ?Await, ?Return]



**CaseBlock[Yield, Await, Return] :**

&ensp;&ensp;{ CaseClauses[?Yield, ?Await, ?Return]opt }

&ensp;&ensp;{ CaseClauses[?Yield, ?Await, ?Return]opt DefaultClause[?Yield, ?Await, ?Return] CaseClauses[?Yield, ?Await, ?Return]opt }

&ensp;&ensp;CaseClauses[Yield, Await, Return] :



**CaseClause[?Yield, ?Await, ?Return]**

&ensp;&ensp;CaseClauses[?Yield, ?Await, ?Return] CaseClause[?Yield, ?Await, ?Return]



**CaseClause[Yield, Await, Return] :**

&ensp;&ensp;case Expression[+In, ?Yield, ?Await] : StatementList[?Yield, ?Await, ?Return]opt



**DefaultClause[Yield, Await, Return] :**

&ensp;&ensp;default : StatementList[?Yield, ?Await, ?Return]opt



**LabelledStatement[Yield, Await, Return] :**

&ensp;&ensp;LabelIdentifier[?Yield, ?Await] : LabelledItem[?Yield, ?Await, ?Return]



**LabelledItem[Yield, Await, Return] :**

&ensp;&ensp;Statement[?Yield, ?Await, ?Return]

&ensp;&ensp;FunctionDeclaration[?Yield, ?Await, ~Default]



**ThrowStatement[Yield, Await] :**

&ensp;&ensp;throw [no LineTerminator here] Expression[+In, ?Yield, ?Await] ;



**TryStatement[Yield, Await, Return] :**

&ensp;&ensp;try Block[?Yield, ?Await, ?Return] Catch[?Yield, ?Await, ?Return]

&ensp;&ensp;try Block[?Yield, ?Await, ?Return] Finally[?Yield, ?Await, ?Return]

&ensp;&ensp;try Block[?Yield, ?Await, ?Return] Catch[?Yield, ?Await, ?Return] Finally[?Yield, ?Await, ?Return]



**Catch[Yield, Await, Return] :**

&ensp;&ensp;catch ( CatchParameter[?Yield, ?Await] ) Block[?Yield, ?Await, ?Return]

&ensp;&ensp;catch Block[?Yield, ?Await, ?Return]



**Finally[Yield, Await, Return] :**

&ensp;&ensp;finally Block[?Yield, ?Await, ?Return]



**CatchParameter[Yield, Await] :**

&ensp;&ensp;BindingIdentifier[?Yield, ?Await]

&ensp;&ensp;BindingPattern[?Yield, ?Await]



**DebuggerStatement :**

&ensp;&ensp;debugger ;