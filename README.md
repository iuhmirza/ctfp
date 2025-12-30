# Category Theory for Programmers (in Rust)

This repository contains Rust implementations of the concepts and exercises presented in **Bartosz Milewski's** famous series and book, [Category Theory for Programmers](https://github.com/hmemcpy/milewski-ctfp-pdf).

While the original text often utilizes C++ and Haskell for examples, this project aims to explore how these Category Theoretic concepts map to **Rust's** type system, ownership model, and traits.

## 📂 Project Structure

The project is set up as a Rust library.

```text
.
├── Cargo.lock
├── Cargo.toml
└── src
    ├── lib.rs       # Module declarations
    ├── identity.rs  # The Identity Morphism
    └── compose.rs   # Morphism Composition

```

## 📚 Progress & Concepts

### Chapter 1: Category: The Essence of Composition

A category consists of objects and morphisms (arrows) that go between them. In programming, we generally treat **Types** as objects and **Functions** as morphisms. To form a valid category, two properties must hold:

#### 1. The Identity Morphism

In Rust, this is implemented as a generic function that takes a value of any type `T` and returns it unchanged.

**Implementation (`src/identity.rs`):**

```rust
pub fn identity<T>(x: T) -> T {
    x
}

```

#### 2. Composition
In Rust, composition is slightly more complex than in Haskell due to ownership. We implement a function that takes two closures (or functions) and returns a new closure (`impl Fn`).

**Implementation (`src/compose.rs`):**

```rust
pub fn compose<A, B, C, F, G>(f: F, g: G) -> impl Fn(A) -> C
where 
    F: Fn(A) -> B,
    G: Fn(B) -> C,
{
    // The 'move' keyword is crucial here to take ownership 
    // of f and g into the returned closure context.
    move |x| { g(f(x)) }
}

```

---

## 🧪 Testing and Verification

The repository includes unit tests to verify the categorical laws (Associativity and Identity).

To run the tests:

```bash
cargo test

```

### Verified Laws

The tests in `src/compose.rs` and `src/identity.rs` verify:

1. **Identity:** `identity(x) == x`
2. **Composition:** `compose(f, g)` produces the correct result.
3. **Identity Laws (Left and Right identity):**




*Snippet from tests:*

```rust
let add_one = |x: i32| x + 1;
// Testing that composing with identity doesn't change behavior
assert_eq!(
    compose(add_one, identity)(1), 
    compose(identity, add_one)(1)
);

```

## 🛠 Usage

To use these functions in your own code, simply import them from the crate:

```rust
use ctfp::identity::identity;
use ctfp::compose::compose;

fn main() {
    let f = |x: i32| x * 2;
    let g = |x: i32| x + 5;
    
    // Mathematically: g(f(x))
    let combined = compose(f, g); 
    
    println!("Result: {}", combined(10)); // (10 * 2) + 5 = 25
}

```

## 🔗 References

* [Category Theory for Programmers (PDF)](https://github.com/hmemcpy/milewski-ctfp-pdf)
* [Bartosz Milewski's Blog](https://bartoszmilewski.com/)
