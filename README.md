# sculpt - natural deduction helper

Sculpt is a small tool to prove first-order formulas using natural deduction, similar to [Coq](https://coq.inria.fr/) (in spirit, not in rigour!).

It allows users to define _theorems_, and then prove it using already-defined theorems by applying deduction rules.

It is mainly used as a REPL. An experimental, unfinished file-based interpreter can be enabled using a feature flag.

## Installation

### Via Github

Use cargo to install directly from this repository:
```
$ cargo install --git https://github.com/MyselfLeo/sculpt -- features exec      # with the file interpreter
$ cargo install --git https://github.com/MyselfLeo/sculpt                       # without

### Via local installation

1. Clone this repository on your system.

2. Inside the repository, use cargo to install the crate on your system:
    ```
    $ cargo install --path . --features exec      # with the file interpreter
    $ cargo install --path .                      # without
    ```

3. You can now use `sculpt` to start the REPL!

## How to use


1. Create a new context. We'll call it _Foobar_.
   ```
   > context FooBar
   ```

2. Let's define some assumptions. We use the command `Thm` to enter proof mode, then leave without proving anything using `Admit`.
This simple example will only use propositionnal logic.
   ```
   > Thm a_and_j ::  A /\ J => Z.  Admit.
   > Thm a_if_j  ::  J => A.       Admit.
   > Thm j_or_z  ::  J \/ Z.       Admit.
   ```


3. Considering the theorems `a_and_j`, `a_if_j` and `j_or_z`, lets prove `Z`.
   ```
   > Thm z :: Z.
   ```
4. We are now in proof mode. We'll import the pre-defined theorems to the current proof context with the `Use` command.
   ```
   > Use a_and_j. Use a_if_j. Use j_or_z
   ```
   
   > _Note: You may have guessed it by now, but just like in Coq, dots ('.') separate commands._

5. We can now apply natural deduction rules to prove `Z`.
   ```
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
   
6. Now that no goal remains, we can end the proof using `Qed`.
   ```
   > Qed
   ```
   The theorem `z` was created!


### File interpreter

If enabled, the file interpreter can be run using the following command:
```
$ sculpt exec path/to/file.sculpt
```

An example of such a file is something like that:
```
Thm t1 :: forall x, (P(x) => Q(x)). Admit.

Thm t2 :: (exists x, P(x)) => (exists x, Q(x)).
    Use t1.
    intro.
    consider exists a, P(a).
    rename_as x.
    axiom.
    fix_as a.
    trans P(a).
    gen a.
    axiom.
    axiom.
Qed.
```
Using dots is really important in this case, as it delimit each command!

## Known issues

- The `help [command]` command might not work for every command. This will be fixed in a future release. I know how to fix it.

## License

Sculpt is licensed under **GNU General Public License v3.0**. See `LICENSE.txt`.
