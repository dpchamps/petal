The Petal Lexer adheres to the [ECMA2020 Specification](https://262.ecma-international.org/#sec-ecmascript-language-lexical-grammar).

# Grammar

As an intersection of the ECMAScript Language, the Petal Grammar Omits valid ECMA, as outlined in this document.

## Keywords and Reserved Words

ReservedWord :: one of

&emsp;&emsp; await break case const continue debugger default do else enum export extends false finally

&emsp;&emsp; for if import in let return switch true typeof while yield

### Diff

Removed:

* catch
* class
* delete
* function
* instanceof
* new
* null
* super
* this
* throw
* try
* var 
* void
* with

Removed:


## Punctuators

Punctuator :: one of

&emsp;&emsp; { ( ) [ ] . ... ; , < > <= >= == != + - * % ** << >> >>> & | ^ ! ~ && || ?? ? : =

&emsp;&emsp; += -= *= %= **= <<= >>= >>>= &= |= ^= =>

DivPunctuator::

&emsp;&emsp; /

&emsp;&emsp; /=

RightBracePunctuator::

&emsp;&emsp; }

### Diff:

Removed: 

* `OptionalChainingPunctuator` Production
* `===`
* `!==` 
* `++`
* `--`

## Literals

Removed:

* `NullLiteral` Production

# Expressions

## Identifiers

Removed:

* `LabelIdentifier` Production

## Primary Expression

PrimaryExpression<sub>`[Yield, Await]`</sub>:

&emsp;&emsp; IdentifierReference<sub>`[?Yield, ?Await]`</sub>

&emsp;&emsp; Literal

&emsp;&emsp;  ArrayLiteral<sub>`[?Yield, ?Await]`</sub>

&emsp;&emsp; ObjectLiteral<sub>`[?Yield, ?Await]`</sub>

&emsp;&emsp; RegularExpressionLiteral

&emsp;&emsp; TemplateLiteral<sub>`[?Yield, ?Await, ~Tagged]`</sub>

&emsp;&emsp; CoverParenthesizedExpressionAndArrowParameterList<sub>`[?Yield, ?Await]`</sub>


CoverParenthesizedExpressionAndArrowParameterList<sub>`[?Yield, ?Await]`</sub>:

&emsp;&emsp; (Expression<sub>`[[+In, ?Yield, ?Await]`</sub>)

&emsp;&emsp; (Expression<sub>`[[+In, ?Yield, ?Await]`</sub>,)

&emsp;&emsp; ()

&emsp;&emsp; (...BindingIdentifier<sub>`[?Yield, ?Await]`</sub>)

&emsp;&emsp; (...BindingPattern<sub>`[?Yield, ?Await]`</sub>)

&emsp;&emsp; (Expression<sub>`[[+In, ?Yield, ?Await]`</sub>,...BindingIdentifier<sub>`[?Yield, ?Await]`</sub>)

&emsp;&emsp; (Expression<sub>`[[+In, ?Yield, ?Await]`</sub>,...BindingPattern<sub>`[?Yield, ?Await]`</sub>)

Removed:

* this
* FunctionExpression
* ClassExpression
* GeneratorExpression
* AsyncFunctionExpression
* AsyncGeneratorExpression

## Literals

Literal:

&emsp;&emsp; BooleanLiteral

&emsp;&emsp; NumericLiteral

&emsp;&emsp; StringLiteral

Removed:

* NullLiteral

# Left-Hand-Side Expressions

Removed:

* LeftHandSideExpression production
    * MemberExpression production
    * SuperProperty production
    * MetaProperty production
    * NewTarget production
    * ImportMeta production
    * CallExpression production
    * ImportCallExpression production  
    * NewExpression production
    * SuperCall production
    * OptionalExpression production  
    * OptionalChain production  
    
# Update Expressions

* UpdateExpression

# Unary Operators

UnaryExpression<sub>`[Yield, Await]`</sub>:

&emsp;&emsp; typeof UnaryExpression<sub>`[?Yield, ?Await]`</sub>

&emsp;&emsp; + UnaryExpression<sub>`[?Yield, ?Await]`</sub>

&emsp;&emsp; - UnaryExpression<sub>`[?Yield, ?Await]`</sub>

&emsp;&emsp; ~ UnaryExpression<sub>`[?Yield, ?Await]`</sub>

&emsp;&emsp; ! UnaryExpression<sub>`[?Yield, ?Await]`</sub>

&emsp;&emsp; `[+Await]` AwaitExpression <sub>`[?Yield]`</sub>

Removed:

* UpdateExpression
* delete UnaryExpression
* void UnaryExpression

# RelationalOperators

RelationalExpression<sub>`[In, Yield, Await]`</sub>:

&emsp;&emsp; ShiftExpression<sub>`[?Yield, ?Await]`</sub>

&emsp;&emsp; RelationalExpression<sub>`[?In, ?Yield, ?Await]`</sub> < ShiftExpression<sub>`[?Yield, ?Await]`</sub>

&emsp;&emsp; RelationalExpression<sub>`[?In, ?Yield, ?Await]`</sub> > ShiftExpression<sub>`[?Yield, ?Await]`</sub>

&emsp;&emsp; RelationalExpression<sub>`[?In, ?Yield, ?Await]`</sub> <= ShiftExpression<sub>`[?Yield, ?Await]`</sub>

&emsp;&emsp; RelationalExpression<sub>`[?In, ?Yield, ?Await]`</sub> >= ShiftExpression<sub>`[?Yield, ?Await]`</sub>

&emsp;&emsp; RelationalExpression<sub>`[?In, ?Yield, ?Await]`</sub>instanceofShiftExpression<sub>`[?Yield, ?Await]`</sub>

&emsp;&emsp; `[+In]`RelationalExpression<sub>`[In, Yield, Await]`</sub>inShiftExpression<sub>`[?Yield, ?Await]`</sub>

removed:

* RelationalExpression<sub>`[?In, ?Yield, ?Await]`</sub>instanceofShiftExpression<sub>`[?Yield, ?Await]`</sub>

# Equality Operators

EqualityExpression<sub>`[In, Yield, Await]`</sub>:

RelationalExpression<sub>`[?In, ?Yield, ?Await]`</sub>

EqualityExpression<sub>`[?In, ?Yield, ?Await]`</sub> == RelationalExpression<sub>`[?In, ?Yield, ?Await]`</sub>

EqualityExpression<sub>`[?In, ?Yield, ?Await]`</sub> != RelationalExpression<sub>`[?In, ?Yield, ?Await]`</sub>


Removed:
* EqualityExpression<sub>`[?In, ?Yield, ?Await]`</sub> === RelationalExpression<sub>`[?In, ?Yield, ?Await]`</sub>
* EqualityExpression<sub>`[?In, ?Yield, ?Await]`</sub> !== RelationalExpression<sub>`[?In, ?Yield, ?Await]`</sub>