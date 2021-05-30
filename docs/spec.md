The Petal Lexer adheres to the [ECMA2020 Specification](https://262.ecma-international.org/#sec-ecmascript-language-lexical-grammar).

# Grammar

As an intersection of the ECMAScript Language, the Petal Grammar Omits valid ECMA, as outlined in this document.

## Keywords and Reserved Words

ReservedWord :: one of
&emsp;&emsp; await break case const continue debugger default do else enum export extends false finally
&emsp;&emsp; for if import in return switch true typeof while yield

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

