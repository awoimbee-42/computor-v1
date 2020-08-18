# Computor

## What this is

This is supposed to be a CAS calculator.
As of now, it's only a normal calculator that can also handle unknown variables.

In order to be a fully fledged `computor-v1`,
it must be able to resolve equations of degree 2 (`a*x^2+b*x+c=0`).

## TODO

* Add complex numbers
* solve some equations ffs
* add global variables & functions

## Parsing

This project uses a Recursive Descent Parser

* Parser grammar: http://pages.cs.wisc.edu/~fischer/cs536.s08/course.hold/html/NOTES/3.CFG.html#exp

```
Expr    --> Expr { '+'|'-' Term } | Term
Term    --> Term { '\*'|'/' Factor }
Factor  --> Value {'^' Factor}
Value   --> Num | Var | Fun | '(' Expr ')'
```
