# deducnat
Natural deduction helper

## How to use

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

