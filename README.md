# tralloc: tracing allocator for Rust

This project allows you to log all the allocations to stderr, stdout or a file.

To use it, register the global allocator and activate it:

```rust
#![feature(global_allocator)]

extern crate tralloc;

#[global_allocator]
static GLOBAL: tralloc::Allocator = tralloc::Allocator{};

fn main() {
  tralloc::Allocator::write_to_stderr();
  tralloc::Allocator::activate();

  let s = String::from("Hello world!");

  let mut v = Vec::new();
  v.push(1);
```

The following will be printed:

```rust
00029801ACDA259B A 00007FB780500000 000000000000000C
00029801ACDB7EFB A 00007FB780500010 0000000000000010
00029801ACDBAAC1 D 00007FB780500010 0000000000000010
00029801ACDBCD09 D 00007FB780500000 000000000000000C
```

Columns:

- time (monotonic, so not linked to any timezone)
- `A` for allocation, `D` for deallocation
- memory address
- size

You can use the `activate` and `deactivate` methods to start
and stop collection at any time.

#### Note

This repository is based on tracing_allocator. It was forked because of breaking changes in Rust internals and lack of communication from the original author.
