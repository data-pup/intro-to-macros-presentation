# What Are Macros?

If you've been programming Rust for a little while already, you have almost
certainly been using macros already. Some common tasks you have been using
this for might include printing text to `println!`, or creating a vector
using `vec!`.

Calls to macros are visibly different from calls to normal functions,
because they end with an exclamation mark `like_this!(..)` as opposed to normal
functions that look `like_this(..)`.

But what *are* macros, why do we need them, and what are they useful for?

### Summary

The most important thing to understand about macros is that they are a
way to write code that writes other code. This is also referred to as
"metaprogramming", and helps to reduce the amount of code you have to
write and maintain.

Aside from brevity, macros also have the advantage of providing more
flexibility than functions. Functions must declare the number of parameters,
and the type of each parameter, in advance. Macros however, can work with
a variable number of parameters!

For example:

```rust
println!("Hello, World!");
println!("Hello, {}!", "World");
```

Both of these will work! How?

When the compiler reads your program, it makes an initial pass that processes
macros, and expands them before interpreting the program. This means that
macros can be used to do things like implement a trait for a given type!

Functions could not do this because they called at runtime, and a trait
must be implemented at compile time. So that's a small glimpse into how
the `#[derive]` attribute works!

### Disadvantages

The biggest downside to using a macro rather than a function is that defining
a macro is almost certainly more difficult than defining a function. Macros
also tend to be more difficult to read and understand than functions, which
can make them more difficult to maintain if you are not careful.

Another minor downside worth keeping in mind is that macros are not namespaced.
This is part of why you must explicitly opt into using the macros from an
external crate, when you do something like this:

```rust
#[macro_use]
extern crate serde;
```

Lastly, macros must be brought into scope or defined before they are used, as
opposed to functions which can be defined or called anywhere.

---

Now, let's look at some different kinds of macros in further detail.
