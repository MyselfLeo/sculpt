   Compiling deducnat v0.1.0 (/home/leo/Projects/rust/deducnat)
error: failed to run custom build command for `deducnat v0.1.0 (/home/leo/Projects/rust/deducnat)`

Caused by:
  process didn't exit successfully: `/home/leo/Projects/rust/deducnat/target/debug/build/deducnat-3b94fb99c7a626e5/build-script-build` (exit status: 1)
  --- stdout
  processing file `/home/leo/Projects/rust/deducnat/src/parsetree/parser.lalrpop`
  /home/leo/Projects/rust/deducnat/src/parsetree/parser.lalrpop:66:5: 67:0: Local ambiguity detected

    The problem arises after having observed the following symbols in the input:
      "~" "(" PrimitiveFormula
    At that point, if the next token is a `")"`, then the parser can proceed in two different ways.

    First, the parser could execute the production at
    /home/leo/Projects/rust/deducnat/src/parsetree/parser.lalrpop:66:5: 67:0, which would consume
    the top 1 token(s) from the stack and produce a `BoolOp`. This might then yield a parse tree
    like
      "(" PrimitiveFormula ")"
      │   ├─BoolOp───────┤   │
      │   ├─Implication──┤   │
      │   └─Formula──────┘   │
      └─Parenthesized────────┘

    Alternatively, the parser could shift the `")"` token and later use it to construct a
    `Parenthesized`. This might then yield a parse tree like
      "~" "(" PrimitiveFormula ")"
      │   ├─Parenthesized────────┤
      │   └─PrimitiveFormula─────┤
      └─Formula──────────────────┘

    See the LALRPOP manual for advice on making your grammar LR(1).

  /home/leo/Projects/rust/deducnat/src/parsetree/parser.lalrpop:17:9: 17:19: Conflict detected

      when in this state:
    (<Term> ",")+ = (*) (<Term> ",")+ Term "," ["(", ")", ",", "/\\", "=>", "\\/", "exists", "forall", "~", r#"[A-Z][a-zA-Z0-9]*'*"#, r#"[a-z]'*"#, r#"[a-z][a-zA-Z0-9]+'*"#, Eof]
    (<Term> ",")+ = (*) Term "," ["(", ")", ",", "/\\", "=>", "\\/", "exists", "forall", "~", r#"[A-Z][a-zA-Z0-9]*'*"#, r#"[a-z]'*"#, r#"[a-z][a-zA-Z0-9]+'*"#, Eof]
    Comma<Term> = (*) ["(", ")", ",", "/\\", "=>", "\\/", "exists", "forall", "~", r#"[A-Z][a-zA-Z0-9]*'*"#, r#"[a-z]'*"#, r#"[a-z][a-zA-Z0-9]+'*"#, Eof]
    Comma<Term> = (*) (<Term> ",")+ ["(", ")", ",", "/\\", "=>", "\\/", "exists", "forall", "~", r#"[A-Z][a-zA-Z0-9]*'*"#, r#"[a-z]'*"#, r#"[a-z][a-zA-Z0-9]+'*"#, Eof]
    Comma<Term> = (*) (<Term> ",")+ Term ["(", ")", ",", "/\\", "=>", "\\/", "exists", "forall", "~", r#"[A-Z][a-zA-Z0-9]*'*"#, r#"[a-z]'*"#, r#"[a-z][a-zA-Z0-9]+'*"#, Eof]
    Comma<Term> = (*) Term ["(", ")", ",", "/\\", "=>", "\\/", "exists", "forall", "~", r#"[A-Z][a-zA-Z0-9]*'*"#, r#"[a-z]'*"#, r#"[a-z][a-zA-Z0-9]+'*"#, Eof]
    PrimitiveFormula = r#"[A-Z][a-zA-Z0-9]*'*"# "(" (*) Terms ")" ["(", ")", ",", "/\\", "=>", "\\/", "exists", "forall", "~", r#"[A-Z][a-zA-Z0-9]*'*"#, r#"[a-z]'*"#, r#"[a-z][a-zA-Z0-9]+'*"#, Eof]
    Term = (*) Variable ["(", ")", ",", "/\\", "=>", "\\/", "exists", "forall", "~", r#"[A-Z][a-zA-Z0-9]*'*"#, r#"[a-z]'*"#, r#"[a-z][a-zA-Z0-9]+'*"#, Eof]
    Term = (*) r#"[a-z][a-zA-Z0-9]+'*"# ["(", ")", ",", "/\\", "=>", "\\/", "exists", "forall", "~", r#"[A-Z][a-zA-Z0-9]*'*"#, r#"[a-z]'*"#, r#"[a-z][a-zA-Z0-9]+'*"#, Eof]
    Term = (*) r#"[a-z][a-zA-Z0-9]+'*"# "(" Terms ")" ["(", ")", ",", "/\\", "=>", "\\/", "exists", "forall", "~", r#"[A-Z][a-zA-Z0-9]*'*"#, r#"[a-z]'*"#, r#"[a-z][a-zA-Z0-9]+'*"#, Eof]
    Terms = (*) Comma<Term> ["(", ")", ",", "/\\", "=>", "\\/", "exists", "forall", "~", r#"[A-Z][a-zA-Z0-9]*'*"#, r#"[a-z]'*"#, r#"[a-z][a-zA-Z0-9]+'*"#, Eof]
    Variable = (*) r#"[a-z]'*"# ["(", ")", ",", "/\\", "=>", "\\/", "exists", "forall", "~", r#"[A-Z][a-zA-Z0-9]*'*"#, r#"[a-z]'*"#, r#"[a-z][a-zA-Z0-9]+'*"#, Eof]

    and looking at a token `r#"[a-z]'*"#` we can reduce to a `Comma<Term>` but we can also shift

  /home/leo/Projects/rust/deducnat/src/parsetree/parser.lalrpop:17:9: 17:19: Conflict detected

      when in this state:
    (<Term> ",")+ = (*) (<Term> ",")+ Term "," ["(", ")", ",", "/\\", "=>", "\\/", "exists", "forall", "~", r#"[A-Z][a-zA-Z0-9]*'*"#, r#"[a-z]'*"#, r#"[a-z][a-zA-Z0-9]+'*"#, Eof]
    (<Term> ",")+ = (*) Term "," ["(", ")", ",", "/\\", "=>", "\\/", "exists", "forall", "~", r#"[A-Z][a-zA-Z0-9]*'*"#, r#"[a-z]'*"#, r#"[a-z][a-zA-Z0-9]+'*"#, Eof]
    Comma<Term> = (*) ["(", ")", ",", "/\\", "=>", "\\/", "exists", "forall", "~", r#"[A-Z][a-zA-Z0-9]*'*"#, r#"[a-z]'*"#, r#"[a-z][a-zA-Z0-9]+'*"#, Eof]
    Comma<Term> = (*) (<Term> ",")+ ["(", ")", ",", "/\\", "=>", "\\/", "exists", "forall", "~", r#"[A-Z][a-zA-Z0-9]*'*"#, r#"[a-z]'*"#, r#"[a-z][a-zA-Z0-9]+'*"#, Eof]
    Comma<Term> = (*) (<Term> ",")+ Term ["(", ")", ",", "/\\", "=>", "\\/", "exists", "forall", "~", r#"[A-Z][a-zA-Z0-9]*'*"#, r#"[a-z]'*"#, r#"[a-z][a-zA-Z0-9]+'*"#, Eof]
    Comma<Term> = (*) Term ["(", ")", ",", "/\\", "=>", "\\/", "exists", "forall", "~", r#"[A-Z][a-zA-Z0-9]*'*"#, r#"[a-z]'*"#, r#"[a-z][a-zA-Z0-9]+'*"#, Eof]
    PrimitiveFormula = r#"[A-Z][a-zA-Z0-9]*'*"# "(" (*) Terms ")" ["(", ")", ",", "/\\", "=>", "\\/", "exists", "forall", "~", r#"[A-Z][a-zA-Z0-9]*'*"#, r#"[a-z]'*"#, r#"[a-z][a-zA-Z0-9]+'*"#, Eof]
    Term = (*) Variable ["(", ")", ",", "/\\", "=>", "\\/", "exists", "forall", "~", r#"[A-Z][a-zA-Z0-9]*'*"#, r#"[a-z]'*"#, r#"[a-z][a-zA-Z0-9]+'*"#, Eof]
    Term = (*) r#"[a-z][a-zA-Z0-9]+'*"# ["(", ")", ",", "/\\", "=>", "\\/", "exists", "forall", "~", r#"[A-Z][a-zA-Z0-9]*'*"#, r#"[a-z]'*"#, r#"[a-z][a-zA-Z0-9]+'*"#, Eof]
    Term = (*) r#"[a-z][a-zA-Z0-9]+'*"# "(" Terms ")" ["(", ")", ",", "/\\", "=>", "\\/", "exists", "forall", "~", r#"[A-Z][a-zA-Z0-9]*'*"#, r#"[a-z]'*"#, r#"[a-z][a-zA-Z0-9]+'*"#, Eof]
    Terms = (*) Comma<Term> ["(", ")", ",", "/\\", "=>", "\\/", "exists", "forall", "~", r#"[A-Z][a-zA-Z0-9]*'*"#, r#"[a-z]'*"#, r#"[a-z][a-zA-Z0-9]+'*"#, Eof]
    Variable = (*) r#"[a-z]'*"# ["(", ")", ",", "/\\", "=>", "\\/", "exists", "forall", "~", r#"[A-Z][a-zA-Z0-9]*'*"#, r#"[a-z]'*"#, r#"[a-z][a-zA-Z0-9]+'*"#, Eof]

    and looking at a token `r#"[a-z][a-zA-Z0-9]+'*"#` we can reduce to a `Comma<Term>` but we can also shift

  /home/leo/Projects/rust/deducnat/src/parsetree/parser.lalrpop:17:9: 17:19: Conflict detected

      when in this state:
    (<Term> ",")+ = (<Term> ",")+ (*) Term "," ["(", ")", ",", "/\\", "=>", "\\/", "exists", "forall", "~", r#"[A-Z][a-zA-Z0-9]*'*"#, r#"[a-z]'*"#, r#"[a-z][a-zA-Z0-9]+'*"#, Eof]
    Comma<Term> = (<Term> ",")+ (*) ["(", ")", ",", "/\\", "=>", "\\/", "exists", "forall", "~", r#"[A-Z][a-zA-Z0-9]*'*"#, r#"[a-z]'*"#, r#"[a-z][a-zA-Z0-9]+'*"#, Eof]
    Comma<Term> = (<Term> ",")+ (*) Term ["(", ")", ",", "/\\", "=>", "\\/", "exists", "forall", "~", r#"[A-Z][a-zA-Z0-9]*'*"#, r#"[a-z]'*"#, r#"[a-z][a-zA-Z0-9]+'*"#, Eof]
    Term = (*) Variable ["(", ")", ",", "/\\", "=>", "\\/", "exists", "forall", "~", r#"[A-Z][a-zA-Z0-9]*'*"#, r#"[a-z]'*"#, r#"[a-z][a-zA-Z0-9]+'*"#, Eof]
    Term = (*) r#"[a-z][a-zA-Z0-9]+'*"# ["(", ")", ",", "/\\", "=>", "\\/", "exists", "forall", "~", r#"[A-Z][a-zA-Z0-9]*'*"#, r#"[a-z]'*"#, r#"[a-z][a-zA-Z0-9]+'*"#, Eof]
    Term = (*) r#"[a-z][a-zA-Z0-9]+'*"# "(" Terms ")" ["(", ")", ",", "/\\", "=>", "\\/", "exists", "forall", "~", r#"[A-Z][a-zA-Z0-9]*'*"#, r#"[a-z]'*"#, r#"[a-z][a-zA-Z0-9]+'*"#, Eof]
    Variable = (*) r#"[a-z]'*"# ["(", ")", ",", "/\\", "=>", "\\/", "exists", "forall", "~", r#"[A-Z][a-zA-Z0-9]*'*"#, r#"[a-z]'*"#, r#"[a-z][a-zA-Z0-9]+'*"#, Eof]

    and looking at a token `r#"[a-z]'*"#` we can reduce to a `Comma<Term>` but we can also shift

  /home/leo/Projects/rust/deducnat/src/parsetree/parser.lalrpop:17:9: 17:19: Conflict detected

      when in this state:
    (<Term> ",")+ = (<Term> ",")+ (*) Term "," ["(", ")", ",", "/\\", "=>", "\\/", "exists", "forall", "~", r#"[A-Z][a-zA-Z0-9]*'*"#, r#"[a-z]'*"#, r#"[a-z][a-zA-Z0-9]+'*"#, Eof]
    Comma<Term> = (<Term> ",")+ (*) ["(", ")", ",", "/\\", "=>", "\\/", "exists", "forall", "~", r#"[A-Z][a-zA-Z0-9]*'*"#, r#"[a-z]'*"#, r#"[a-z][a-zA-Z0-9]+'*"#, Eof]
    Comma<Term> = (<Term> ",")+ (*) Term ["(", ")", ",", "/\\", "=>", "\\/", "exists", "forall", "~", r#"[A-Z][a-zA-Z0-9]*'*"#, r#"[a-z]'*"#, r#"[a-z][a-zA-Z0-9]+'*"#, Eof]
    Term = (*) Variable ["(", ")", ",", "/\\", "=>", "\\/", "exists", "forall", "~", r#"[A-Z][a-zA-Z0-9]*'*"#, r#"[a-z]'*"#, r#"[a-z][a-zA-Z0-9]+'*"#, Eof]
    Term = (*) r#"[a-z][a-zA-Z0-9]+'*"# ["(", ")", ",", "/\\", "=>", "\\/", "exists", "forall", "~", r#"[A-Z][a-zA-Z0-9]*'*"#, r#"[a-z]'*"#, r#"[a-z][a-zA-Z0-9]+'*"#, Eof]
    Term = (*) r#"[a-z][a-zA-Z0-9]+'*"# "(" Terms ")" ["(", ")", ",", "/\\", "=>", "\\/", "exists", "forall", "~", r#"[A-Z][a-zA-Z0-9]*'*"#, r#"[a-z]'*"#, r#"[a-z][a-zA-Z0-9]+'*"#, Eof]
    Variable = (*) r#"[a-z]'*"# ["(", ")", ",", "/\\", "=>", "\\/", "exists", "forall", "~", r#"[A-Z][a-zA-Z0-9]*'*"#, r#"[a-z]'*"#, r#"[a-z][a-zA-Z0-9]+'*"#, Eof]

    and looking at a token `r#"[a-z][a-zA-Z0-9]+'*"#` we can reduce to a `Comma<Term>` but we can also shift

  /home/leo/Projects/rust/deducnat/src/parsetree/parser.lalrpop:17:9: 17:19: Conflict detected

      when in this state:
    (<Term> ",")+ = Term (*) "," ["(", ")", ",", "/\\", "=>", "\\/", "exists", "forall", "~", r#"[A-Z][a-zA-Z0-9]*'*"#, r#"[a-z]'*"#, r#"[a-z][a-zA-Z0-9]+'*"#, Eof]
    Comma<Term> = Term (*) ["(", ")", ",", "/\\", "=>", "\\/", "exists", "forall", "~", r#"[A-Z][a-zA-Z0-9]*'*"#, r#"[a-z]'*"#, r#"[a-z][a-zA-Z0-9]+'*"#, Eof]

    and looking at a token `","` we can reduce to a `Comma<Term>` but we can also shift

  /home/leo/Projects/rust/deducnat/src/parsetree/parser.lalrpop:29:5: 29:84: Conflict detected

      when in this state:
    Term = r#"[a-z][a-zA-Z0-9]+'*"# (*) ["(", ")", ",", "/\\", "=>", "\\/", "exists", "forall", "~", r#"[A-Z][a-zA-Z0-9]*'*"#, r#"[a-z]'*"#, r#"[a-z][a-zA-Z0-9]+'*"#, Eof]
    Term = r#"[a-z][a-zA-Z0-9]+'*"# (*) "(" Terms ")" ["(", ")", ",", "/\\", "=>", "\\/", "exists", "forall", "~", r#"[A-Z][a-zA-Z0-9]*'*"#, r#"[a-z]'*"#, r#"[a-z][a-zA-Z0-9]+'*"#, Eof]

    and looking at a token `"("` we can reduce to a `Term` but we can also shift

  /home/leo/Projects/rust/deducnat/src/parsetree/parser.lalrpop:17:9: 17:19: Conflict detected

      when in this state:
    (<Term> ",")+ = (<Term> ",")+ Term (*) "," ["(", ")", ",", "/\\", "=>", "\\/", "exists", "forall", "~", r#"[A-Z][a-zA-Z0-9]*'*"#, r#"[a-z]'*"#, r#"[a-z][a-zA-Z0-9]+'*"#, Eof]
    Comma<Term> = (<Term> ",")+ Term (*) ["(", ")", ",", "/\\", "=>", "\\/", "exists", "forall", "~", r#"[A-Z][a-zA-Z0-9]*'*"#, r#"[a-z]'*"#, r#"[a-z][a-zA-Z0-9]+'*"#, Eof]

    and looking at a token `","` we can reduce to a `Comma<Term>` but we can also shift

  /home/leo/Projects/rust/deducnat/src/parsetree/parser.lalrpop:17:9: 17:19: Conflict detected

      when in this state:
    (<Term> ",")+ = (*) (<Term> ",")+ Term "," ["(", ")", ",", "/\\", "=>", "\\/", "exists", "forall", "~", r#"[A-Z][a-zA-Z0-9]*'*"#, r#"[a-z]'*"#, r#"[a-z][a-zA-Z0-9]+'*"#, Eof]
    (<Term> ",")+ = (*) Term "," ["(", ")", ",", "/\\", "=>", "\\/", "exists", "forall", "~", r#"[A-Z][a-zA-Z0-9]*'*"#, r#"[a-z]'*"#, r#"[a-z][a-zA-Z0-9]+'*"#, Eof]
    Comma<Term> = (*) ["(", ")", ",", "/\\", "=>", "\\/", "exists", "forall", "~", r#"[A-Z][a-zA-Z0-9]*'*"#, r#"[a-z]'*"#, r#"[a-z][a-zA-Z0-9]+'*"#, Eof]
    Comma<Term> = (*) (<Term> ",")+ ["(", ")", ",", "/\\", "=>", "\\/", "exists", "forall", "~", r#"[A-Z][a-zA-Z0-9]*'*"#, r#"[a-z]'*"#, r#"[a-z][a-zA-Z0-9]+'*"#, Eof]
    Comma<Term> = (*) (<Term> ",")+ Term ["(", ")", ",", "/\\", "=>", "\\/", "exists", "forall", "~", r#"[A-Z][a-zA-Z0-9]*'*"#, r#"[a-z]'*"#, r#"[a-z][a-zA-Z0-9]+'*"#, Eof]
    Comma<Term> = (*) Term ["(", ")", ",", "/\\", "=>", "\\/", "exists", "forall", "~", r#"[A-Z][a-zA-Z0-9]*'*"#, r#"[a-z]'*"#, r#"[a-z][a-zA-Z0-9]+'*"#, Eof]
    Term = (*) Variable ["(", ")", ",", "/\\", "=>", "\\/", "exists", "forall", "~", r#"[A-Z][a-zA-Z0-9]*'*"#, r#"[a-z]'*"#, r#"[a-z][a-zA-Z0-9]+'*"#, Eof]
    Term = (*) r#"[a-z][a-zA-Z0-9]+'*"# ["(", ")", ",", "/\\", "=>", "\\/", "exists", "forall", "~", r#"[A-Z][a-zA-Z0-9]*'*"#, r#"[a-z]'*"#, r#"[a-z][a-zA-Z0-9]+'*"#, Eof]
    Term = (*) r#"[a-z][a-zA-Z0-9]+'*"# "(" Terms ")" ["(", ")", ",", "/\\", "=>", "\\/", "exists", "forall", "~", r#"[A-Z][a-zA-Z0-9]*'*"#, r#"[a-z]'*"#, r#"[a-z][a-zA-Z0-9]+'*"#, Eof]
    Term = r#"[a-z][a-zA-Z0-9]+'*"# "(" (*) Terms ")" ["(", ")", ",", "/\\", "=>", "\\/", "exists", "forall", "~", r#"[A-Z][a-zA-Z0-9]*'*"#, r#"[a-z]'*"#, r#"[a-z][a-zA-Z0-9]+'*"#, Eof]
    Terms = (*) Comma<Term> ["(", ")", ",", "/\\", "=>", "\\/", "exists", "forall", "~", r#"[A-Z][a-zA-Z0-9]*'*"#, r#"[a-z]'*"#, r#"[a-z][a-zA-Z0-9]+'*"#, Eof]
    Variable = (*) r#"[a-z]'*"# ["(", ")", ",", "/\\", "=>", "\\/", "exists", "forall", "~", r#"[A-Z][a-zA-Z0-9]*'*"#, r#"[a-z]'*"#, r#"[a-z][a-zA-Z0-9]+'*"#, Eof]

    and looking at a token `r#"[a-z]'*"#` we can reduce to a `Comma<Term>` but we can also shift

  /home/leo/Projects/rust/deducnat/src/parsetree/parser.lalrpop:17:9: 17:19: Conflict detected

      when in this state:
    (<Term> ",")+ = (*) (<Term> ",")+ Term "," ["(", ")", ",", "/\\", "=>", "\\/", "exists", "forall", "~", r#"[A-Z][a-zA-Z0-9]*'*"#, r#"[a-z]'*"#, r#"[a-z][a-zA-Z0-9]+'*"#, Eof]
    (<Term> ",")+ = (*) Term "," ["(", ")", ",", "/\\", "=>", "\\/", "exists", "forall", "~", r#"[A-Z][a-zA-Z0-9]*'*"#, r#"[a-z]'*"#, r#"[a-z][a-zA-Z0-9]+'*"#, Eof]
    Comma<Term> = (*) ["(", ")", ",", "/\\", "=>", "\\/", "exists", "forall", "~", r#"[A-Z][a-zA-Z0-9]*'*"#, r#"[a-z]'*"#, r#"[a-z][a-zA-Z0-9]+'*"#, Eof]
    Comma<Term> = (*) (<Term> ",")+ ["(", ")", ",", "/\\", "=>", "\\/", "exists", "forall", "~", r#"[A-Z][a-zA-Z0-9]*'*"#, r#"[a-z]'*"#, r#"[a-z][a-zA-Z0-9]+'*"#, Eof]
    Comma<Term> = (*) (<Term> ",")+ Term ["(", ")", ",", "/\\", "=>", "\\/", "exists", "forall", "~", r#"[A-Z][a-zA-Z0-9]*'*"#, r#"[a-z]'*"#, r#"[a-z][a-zA-Z0-9]+'*"#, Eof]
    Comma<Term> = (*) Term ["(", ")", ",", "/\\", "=>", "\\/", "exists", "forall", "~", r#"[A-Z][a-zA-Z0-9]*'*"#, r#"[a-z]'*"#, r#"[a-z][a-zA-Z0-9]+'*"#, Eof]
    Term = (*) Variable ["(", ")", ",", "/\\", "=>", "\\/", "exists", "forall", "~", r#"[A-Z][a-zA-Z0-9]*'*"#, r#"[a-z]'*"#, r#"[a-z][a-zA-Z0-9]+'*"#, Eof]
    Term = (*) r#"[a-z][a-zA-Z0-9]+'*"# ["(", ")", ",", "/\\", "=>", "\\/", "exists", "forall", "~", r#"[A-Z][a-zA-Z0-9]*'*"#, r#"[a-z]'*"#, r#"[a-z][a-zA-Z0-9]+'*"#, Eof]
    Term = (*) r#"[a-z][a-zA-Z0-9]+'*"# "(" Terms ")" ["(", ")", ",", "/\\", "=>", "\\/", "exists", "forall", "~", r#"[A-Z][a-zA-Z0-9]*'*"#, r#"[a-z]'*"#, r#"[a-z][a-zA-Z0-9]+'*"#, Eof]
    Term = r#"[a-z][a-zA-Z0-9]+'*"# "(" (*) Terms ")" ["(", ")", ",", "/\\", "=>", "\\/", "exists", "forall", "~", r#"[A-Z][a-zA-Z0-9]*'*"#, r#"[a-z]'*"#, r#"[a-z][a-zA-Z0-9]+'*"#, Eof]
    Terms = (*) Comma<Term> ["(", ")", ",", "/\\", "=>", "\\/", "exists", "forall", "~", r#"[A-Z][a-zA-Z0-9]*'*"#, r#"[a-z]'*"#, r#"[a-z][a-zA-Z0-9]+'*"#, Eof]
    Variable = (*) r#"[a-z]'*"# ["(", ")", ",", "/\\", "=>", "\\/", "exists", "forall", "~", r#"[A-Z][a-zA-Z0-9]*'*"#, r#"[a-z]'*"#, r#"[a-z][a-zA-Z0-9]+'*"#, Eof]

    and looking at a token `r#"[a-z][a-zA-Z0-9]+'*"#` we can reduce to a `Comma<Term>` but we can also shift

  /home/leo/Projects/rust/deducnat/src/parsetree/parser.lalrpop:66:5: 67:0: Conflict detected

      when in this state:
    BoolOp = PrimitiveFormula (*) ["(", ")", ",", "/\\", "=>", "\\/", "exists", "forall", "~", r#"[A-Z][a-zA-Z0-9]*'*"#, r#"[a-z]'*"#, r#"[a-z][a-zA-Z0-9]+'*"#, Eof]
    Parenthesized = "(" PrimitiveFormula (*) ")" ["(", ")", ",", "/\\", "=>", "\\/", "exists", "forall", "~", r#"[A-Z][a-zA-Z0-9]*'*"#, r#"[a-z]'*"#, r#"[a-z][a-zA-Z0-9]+'*"#, Eof]

    and looking at a token `")"` we can reduce to a `BoolOp` but we can also shift

