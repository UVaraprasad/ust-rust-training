Rust is a statically typed language that is designed to prevent common programming mistakes such as null pointer dereferences. One of the key features of Rust that helps achieve this is its concept of lifetimes.

**What are lifetimes?**

In Rust, lifetimes are a way to specify the scope of a reference to a value. A reference is a value that is a reference to another value. For example, if you have a variable `x` and you do `let y = &x`, then `y` is a reference to `x`. The lifetime of `y` is the scope in which `y` is valid. In other words, it is the duration of time during which `y` can be used.

**Why are lifetimes necessary?**

In languages like C and C++, it is common to have pointers that can point to any memory location. This can lead to null pointer dereferences, which can cause the program to crash or behave unexpectedly. Rust's lifetimes help prevent this by ensuring that a reference can only be used within its lifetime.

**How do lifetimes work?**

In Rust, lifetimes are specified using the ` lifetime` keyword. For example, if you have a function that takes a reference to a value, you can specify the lifetime of that reference like this:

```rust
fn foo<'a>(x: &'a i32) {
    // code here
}
```

In this example, `'a` is a lifetime parameter. It is a placeholder for a lifetime that is specified when the function is called. The `'a` in the function signature means that the reference `x` can be used within the scope of the function.

**Lifetime annotations**

Rust provides several built-in lifetime annotations that can be used to specify the lifetimes of references. These are:

* `'static`: This is the longest possible lifetime. It means that the reference can be used anywhere in the program.
* `'a`, `'b`, etc.: These are generic lifetime parameters that can be used to specify the lifetimes of references.
* `'_` : This is the anonymous lifetime. It means that the reference can be used within the scope of the function or block where it is defined.

**Lifetime rules**

Rust has several rules for lifetimes that help ensure memory safety. These rules are:

1. **The lifetime of a reference is at least as long as the lifetime of the value it references**: This means that if you have a reference to a value, the reference can only be used within the scope of the value it references.
2. **The lifetime of a reference cannot be longer than the lifetime of the value it references**: This means that if you have a reference to a value, the reference cannot be used after the value it references has gone out of scope.
3. **The lifetime of a reference cannot be longer than the lifetime of any other reference to the same value**: This means that if you have multiple references to the same value, the lifetime of each reference cannot be longer than the lifetime of the value it references.

**Example**

Here is an example of how lifetimes work in Rust:

```rust
fn main() {
    let s = "hello";
    let r = &s; // r is a reference to s
    println!("{}", r); // This is okay
    // r is still valid here
    // println!("{}", r); // This would be an error because r is no longer valid
}
```

In this example, `r` is a reference to `s`. The lifetime of `r` is the same as the lifetime of `s`, which is the scope of the `main` function. Therefore, `r` is valid until the end of the `main` function.