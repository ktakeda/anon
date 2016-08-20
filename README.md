anon
====

anon!{} is a Rust macro which create anonymous structs, like C# anonymous types. 

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
anon = "0.1.0"
# or, anon = { git = "https://github.com/ktakeda/anon" }
```

and this to your crate root:

```rust
#[macro_use(anon)]
extern crate anon;
```

## Examples
```rust
#[macro_use(anon)]
extern crate anon;

#[allow(non_camel_case_types)]
fn main() {
    let a = anon! { a: 10, b: 20 };
    assert_eq!(10, a.a);
    assert_eq!(20, a.b);
}
```

Anon can be used in closure:

```rust
#[macro_use(anon)]
extern crate anon;

#[allow(non_camel_case_types)]
fn main() {
    let y: Vec<_> = vec![1,2].into_iter().map(|x| {
        anon! { a: x, b: x*x } // create anonymous struct
    }).collect();
    assert_eq!(1, y[0].a);
    assert_eq!(2, y[1].a);
    assert_eq!(1, y[0].b);
    assert_eq!(4, y[1].b);
}
```

## LICENSE
MIT.

