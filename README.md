# control_flow
Control Flow In Rust

# if Expressions
An `if` expression allows you to branch your code depending on conditions.

The condition must be a `bool`;

Using too many `else if`s can clutter the code. Rust recommends using the `match` branching construct in that case.

## Using if in a let statement
Because `if` is an expression, we can use it on the right side of a `let` statement to assign the outcome to a variable.

Code blocks evaluate to the last expression in them. The value of the `if` in a `let` statement depends on which block of code executes. The values that have the potential to be results from each arm of the `if` must be the same type.
