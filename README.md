# Computor

## TODO

* Do math of Value instead of Num (means try_add, try_mul, ...)
  -> This will solve the `x^9*2*2` problem ;)
* Add complex numbers
* solve some equations ffs
* add global variables & functions

## Parsing

Recursive Descent Parser

* Parser grammar: http://pages.cs.wisc.edu/~fischer/cs536.s08/course.hold/html/NOTES/3.CFG.html#exp

```
Expr    --> Expr { '+'|'-' Term } | Term
Term    --> Term { '\*'|'/' Factor }
Factor  --> Value {'^' Factor}
Value   --> Num | Var | Fun | '(' Expr ')'
```
