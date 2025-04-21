# aabel-types

A Rust library exploring advanced type system concepts and patterns in Rust, including GADTs, phantom types, typeclasses, and zero-sized types.

## Features

- **Advanced Lifetime Management**: Tools and patterns for complex lifetime scenarios
- **Generalized Algebraic Data Types (GADTs)**: Implementation of GADT-like patterns in Rust
- **Phantom Types**: Type-level programming using phantom types for compile-time guarantees
- **Typeclasses**: Rust-style typeclass implementations and patterns
- **Zero-Sized Types (ZSTs)**: Efficient type-level programming using zero-sized types

## Getting Started

### Prerequisites

- Rust (latest stable version recommended)
- Cargo (comes with Rust)

### Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
aabel-types = "0.1.0"
```

## Usage

### Phantom Types Example

```rust
use aabel_types::phantom_types::*;

// Create a type-safe API using phantom types
let valid_user = User::<Valid>::new("username");
let invalid_user = User::<Invalid>::new("username");

// Type system prevents mixing valid and invalid states
// valid_user.activate(); // Compile-time error if not valid
```

### GADTs Example

```rust
use aabel_types::gadts::*;

// Create type-safe expressions
let expr = Expr::Add(
    Box::new(Expr::Int(1)),
    Box::new(Expr::Int(2))
);

// Evaluate with type safety
let result = expr.eval(); // Returns 3
```

### Typeclasses Example

```rust
use aabel_types::typeclass::*;

// Implement typeclass for your types
impl Monoid for MyType {
    fn mempty() -> Self {
        // Implementation
    }
    
    fn mappend(self, other: Self) -> Self {
        // Implementation
    }
}
```

## Documentation

Each module contains detailed documentation about its specific type system concept:

- `advanced_lifetime`: Advanced lifetime patterns and techniques
- `gadts`: Generalized Algebraic Data Types implementation
- `phantom_types`: Phantom type patterns and usage
- `typeclass`: Typeclass implementations and patterns
- `zsts`: Zero-sized type patterns and optimizations

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Resources

[Rust Type System Deep Dive From GATs to Type Erasure](https://minikin.me/blog/rust-type-system-deep-dive)

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request. When contributing:

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request 