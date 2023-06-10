# Grammar

This directory contains several grammars to assist in the development of Petal, subject to change and general chaos.

## Overview

- [Complete ECMA Grammar](./complete-ecma-grammar.md)
  - The complete ECMA Syntactic Grammar, taken from [ECMA-262 draft](https://tc39.es/ecma262).  
- [Amended Grammar](./amended-grammar.md)
  - The amended ECMA Syntactic Grammar for Petal. It's the Complete ECMA Grammar, minus the productions removed for Petal
- [Petal Grammar Diff](./petal-grammar.diff)
  - A helpful patch file for showing which productions have been removed from the complete ECMA grammar
- [Petal Types Grammar](./petal-types-grammar.md)
  - Type productions not present in the Complete ECMA Grammar
- [Complete Petal Grammar](./complete-petal-grammar.md)
  - Complete Syntactic Grammar that describes the Petal Language

### Lexical Grammar Note

Petal maintains parity with ECMA lexical grammar.