# Procedural Macros

The other type of macro is a "procedural macro." These are called "procedural"
because they bear more resemblance to functions, which are a type of procedure.
Procedural macros accept some Rust code as an input, operate on that code,
and produce some code as an output, rather than matching against patterns
and replacing code like declarative macros.

The primary use for procedural macros is to allow a trait you have defined
to be derivable for various types, which lets you place the trait's name
inside of a `#[derive]` annotation.

Let's say we have a trait named `HelloMacro`, that looks like this:

```rust
pub trait HelloMacro {
    fn hello_macro();
}
```

We *could* manually implement this trait, like so:

```rust
extern crate hello_macro;

use hello_macro::HelloMacro;

struct Pancakes;

impl HelloMacro for Pancakes {
    fn hello_macro() {
        println!("Hello, Macro! My name is Pancakes!");
    }
}

fn main() {
    Pancakes::hello_macro();
}
```

This would work fine, but what if we wanted to spare the users of our library
the effort of implementing this trait manually for their own types?

First, let's make sure to mention an important note: Procedural macros must be
defined within their own crate. So, for a crate named `awoo`, a custom derive
procedural macro crate would be named `awoo_derive`.

There are some different ways to structure a project with two tightly related
crates like this, but that discussion will be considered out of scrope for
this presentation.

Instead, we would like to do something like this:

```rust
extern crate hello_macro;
#[macro_use]
extern crate hello_macro_derive;

use hello_macro::HelloMacro;

#[derive(HelloMacro)]
struct Pancakes;

fn main() {
    Pancakes::hello_macro();
}
```

Let's look into the example for more information about this!
