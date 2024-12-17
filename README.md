# use_with

[![codecov](https://codecov.io/gh/sunsided/use-with/graph/badge.svg?token=Ex1Ok4n9yd)](https://codecov.io/gh/sunsided/use-with)

Provides resource management utilities, ensuring that resources are properly utilized
and subsequently dropped, similar to patterns found in other programming languages like Kotlin's `use` function
and C#'s `using` block.

This module offers two primary functions:
- `use_with`: Executes a closure synchronously, consuming the resource.
- `use_with_async`: Executes an asynchronous closure, consuming the resource.

These functions facilitate safe and efficient resource handling, ensuring that resources are properly utilized
and dropped, even in asynchronous contexts.

# Features
- **Synchronous Resource Management:** The `use_with` function allows for synchronous operations on resources,
  ensuring that resources are properly utilized and dropped after the operation completes.

- **Asynchronous Resource Management:** The `use_with_async` function facilitates asynchronous operations on resources,
  ensuring that resources are properly utilized and dropped after the asynchronous operation completes.

# Usage
To use these functions, the `Use` trait is auto-implemented for your resource types; simply call the appropriate method:

```rust
use use_with::Use;

struct Resource;

impl Resource {
    fn new() -> Self {
        Resource
    }
}

#[test]
fn it_works() {
    let resource = Resource::new();
    let result = resource.use_with(|res| {
        // Perform operations with `res`, return anything.
        42
    });

    // The resource is now dropped.
    assert_eq!(result, 42);
}
```