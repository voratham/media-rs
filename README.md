## How to read
- i step by step learning each commit !
```sh
e44d2e7 chore: refactoring with multiple modules
d3374f1 chore: put to exercise
2ef78e0 chore: replcate our custom enum with option enum standard
7783b44 chore: athernative pattern match in case if-else
4eb7605 chore: craft enum owner'options
ae281f5 chore: example unlabeled fields
d477071 chore: new catelog struct
7ad2652 chore: pattern matching with enum
f4f74cc init: first enum media
```


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