# sculpt - natural deduction helper

Sculpt is a small tool to prove first-order formulas using natural deduction.

It allows the user to apply natural deduction rules onto a goal formula until it can be proven in the current context.

It can be used as a REPL, but later will be able to prove proofs from a file.

## Installation

1. Clone this repository on your system.

2. Inside the repository, use cargo to install the crate on your system:
```
cargo install --path .
```

3. You can now use `sculpt` to start the REPL!

## How to use

1. Create a new context. We'll call it 



1. Start your proof.
```
> proof (A /\ J => Z) => (J => A) => (J \/ Z) => Z
```

2. Apply natural deduction rules. The full list of commands can be found using the `help` command.
```
> intros
> from_or J \/ Z
> axiom
> trans A /\ J
> axiom
> split
> trans J
> axiom
> axiom
> axiom
> axiom
```

3. Finish your proof using `qed`. It's done!

