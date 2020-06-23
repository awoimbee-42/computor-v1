# Computor

## Parsing

Recursive Descent Parser

* Rust parser:
  * source: https://github.com/adrianN/simple_rust_parser
  * explainations: https://adriann.github.io/rust_parser.html
* Parser grammar: http://pages.cs.wisc.edu/~fischer/cs536.s08/course.hold/html/NOTES/3.CFG.html#exp

```
Expr    --> Expr { '+'|'-' Term } | Term
Term    --> Term { '\*'|'/' Factor }
Factor  --> Value {'^' Factor}
Value   --> Num | Var | Fun | '(' Expr ')'
```

Pseudocode Expr parser:
```
parse_expr -> Expr {
    let mut expr = Expr::from(parse_term(lexed.next()));
    While let Some(Op('+'|'-': term)) = lexed.next() {
        expr = Expr::Add(expr, term);
    }
    expr
}
```

```Rust
enum Expr {
    Add((Expr, Term)),
    Sub((Expr, Term)),
    Term(Term),
}

enum Term {
    Mul((Term, Factor))
    Div((Term, Factor))
    Factor(Factor),
}

enum Factor {
    Pow((Final, Factor)) // final ^ fact
    Final(Final)
}

enum Final {
    Num(Num),
    Var(Var),
    Fun(Fun),
    Expr(Expr), // '(' expr ')'
}
```
