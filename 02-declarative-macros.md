# Declarative Macros

Declarative macros are the most common and most widely used for of macro.
You may also hear these called 'macros by example', or even just 'macros.'
There are other kinds, but these are the most common, and should be the first
variant you look to when solving a problem that will require some
metaprogramming in Rust.

It may help to conceptualize these similarly to `match` statements. Just like
`match`, a macro compare a value to different patterns with associated code.
Macros however, accept source code rather than concrete values, compare the
*structure* of that code with its patterns, and replace that code with the
associated code, rather than running it.

### Declaraing a Macro

To get a better handle on all of this, let's create a simplified version of
the `vec!` macro. To review, this macro allows us to do this:

```rust
let vector_of_ints = vec![1, 2, 3];
let vector_of_floats: Vec<f32> = vec![1.0, 2.0, 3.0, 4.0];
```

Note that the number of arguments, and the type of the arguments, differs in
these cases. This is a situation where functions would not be able to help
us accomplish our task!

```rust
#[macro_export]
macro_rules! vec {
    ( $( $x:expr ),* ) => {
        {
            let mut v = Vec::new();
            $(
                v.push($x);
            )*
            v
        }
    };
}
```

If you squint *real* hard, you can almost see something looking like a `match`
statement here! As mentioned before however, this is generating code rather
than running code, and it matches on code rather than values.

There are a few syntax features here that might look a little initially
intimidatinig, so let's briefly cover what these are.

We match on the pattern `( $( $x:expr ),* )`, which is followed by some code
that will be emitted if the input matches this. On the outside, a set of
parentheses encloses the pattern.

Next, `$()` encloses values that will be used in the replacement code. Within
this is `$x:expr`, which will match any expression and give it the name `$x`.

The comma after the `$()` means that the arguments can be comma separated,
and the `*` means that this can match zero or more times.
