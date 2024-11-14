## How to run

```sh
cargo run -q
```

## Note:
- How to create enum custom same as Option
```rust
// TODO: craft owner Options <-- craft enum options
#[derive(Debug)]
enum MightHaveAValue<'a> {
    ThereIsAValue(&'a Media),
    NoValueAvailable,
}
```

- How to separate modules rust
```sh
src
├── content
│   ├── catalog.rs
│   ├── media.rs
│   └── mod.rs # entry point of content modules same as index.js on javascript
└── main.rs

```