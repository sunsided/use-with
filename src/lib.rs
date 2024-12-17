//! # use_with
//!
//! Provides resource management utilities, ensuring that resources are properly utilized
//! and subsequently dropped, similar to patterns found in other programming languages like Kotlin's `use` function
//! and C#'s `using` block.
//!
//! This module offers two primary functions:
//! - `use_with`: Executes a closure synchronously, consuming the resource.
//! - `use_with_async`: Executes an asynchronous closure, consuming the resource.
//!
//! These functions facilitate safe and efficient resource handling, ensuring that resources are properly utilized
//! and dropped, even in asynchronous contexts.
//!
//! # Features
//! - **Synchronous Resource Management:** The `use_with` function allows for synchronous operations on resources,
//!   ensuring that resources are properly utilized and dropped after the operation completes.
//!
//! - **Asynchronous Resource Management:** The `use_with_async` function facilitates asynchronous operations on resources,
//!   ensuring that resources are properly utilized and dropped after the asynchronous operation completes.
//!
//! # Usage
//!To use these functions, the `Use` trait is auto-implemented for your resource types; simply call the appropriate method:
//!
//! ```rust
//! use use_with::Use;
//!
//! struct Resource;
//!
//! impl Resource {
//!     fn new() -> Self {
//!         Resource
//!     }
//! }
//!
//! let result = Resource::new().use_with(|res| {
//!     // Perform operations with `res`, return anything.
//!     42
//! });
//!
//! assert_eq!(result, 42);
//! ```

#![forbid(unsafe_code)]

use std::future::Future;

/// A trait that facilitates resource management by ensuring proper usage and subsequent dropping.
///
/// This trait provides two methods:
/// - `use_with`: Executes a closure synchronously, consuming the resource.
/// - `use_with_async`: Executes an asynchronous closure, consuming the resource.
///
/// Implementing this trait allows for safe and efficient resource handling, ensuring that resources
/// are properly utilized and dropped, even in asynchronous contexts.
pub trait Use {
    /// Executes a closure synchronously, consuming the resource.
    ///
    /// This method takes ownership of `self` and applies the provided closure `f` to it.
    /// After the closure executes, `self` is dropped.
    ///
    /// # Parameters
    /// - `f`: A closure that takes ownership of `self` and returns a value of type `T`.
    ///
    /// # Returns
    /// - A value of type `T`, which is the result of the closure `f`.
    ///
    /// # Examples
    /// ```rust
    /// use use_with::Use;
    ///
    /// struct Resource;
    ///
    /// impl Resource {
    ///     fn new() -> Self {
    ///         Resource
    ///     }
    /// }
    ///
    /// let result = Resource::new().use_with(|res| {
    ///     // Perform operations with `res`, return anything.
    ///     42
    /// });
    ///
    /// assert_eq!(result, 42);
    /// ```
    fn use_with<U, F: FnOnce(Self) -> U>(self, f: F) -> U
    where
        Self: Sized,
    {
        f(self)
    }

    /// Executes an asynchronous closure, consuming the resource.
    ///
    /// This method takes ownership of `self` and applies the provided asynchronous closure `f` to it.
    /// After the asynchronous operation completes, `self` is dropped.
    ///
    /// # Parameters
    /// - `f`: An asynchronous closure that takes ownership of `self` and returns a future.
    ///
    /// # Returns
    /// - A future that resolves to a value of type `U`, which is the result of the asynchronous operation.
    ///
    /// # Examples
    /// ```rust
    /// # #[tokio::main]
    /// # async fn main() {
    /// use use_with::Use;
    /// use std::future::Future;
    ///
    /// struct Resource;
    ///
    /// impl Resource {
    ///     fn new() -> Self {
    ///         Resource
    ///     }
    /// }
    ///
    /// // Usage example
    /// let future = Resource::new().use_with_async(|res| async {
    ///     // Perform asynchronous operations with `res`, return anything.
    ///     42
    /// });
    ///
    /// // Await the result
    /// assert_eq!(future.await, 42);
    /// # }
    /// ```
    fn use_with_async<F, Fut, U>(self, f: F) -> impl Future<Output = U> + Send
    where
        Self: Sized + Send,
        F: FnOnce(Self) -> Fut + Send,
        Fut: Future<Output = U> + Send,
    {
        async { f(self).await }
    }
}

impl<T> Use for T {}

/// Executes a closure with a resource, ensuring the resource is properly utilized and dropped.
///
/// # Parameters
/// - `resource`: The resource to be used.
/// - `closure`: A closure that takes ownership of the resource and performs operations with it.
///
/// # Examples
/// ```rust
/// use use_with::using;
///
/// struct Resource(u32);
///
/// impl Resource {
///     fn new(value: u32) -> Self {
///         Resource(value)
///     }
/// }
///
/// let result = using!(Resource::new(10), it -> {
///     it.0 + 32
/// });
/// assert_eq!(result, 42);
///
/// let resource = Resource::new(10);
/// let result = using!(resource, value -> {
///     value.0 + 32
/// });
/// assert_eq!(result, 42);
/// ```
///
/// # Safety
/// - The closure must not retain references to the resource beyond the scope of this function,
///   as the resource will be dropped after the closure executes.
#[macro_export]
macro_rules! using {
    ($resource:expr, $param:ident -> $body:block) => {{
        let $param = $resource;
        $body
    }};
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Mutex};

    #[test]
    fn test_resource_usage() {
        let drop_flag = Arc::new(Mutex::new(false));

        {
            let drop_flag = drop_flag.clone();
            struct TestResource(Arc<Mutex<bool>>);

            impl Drop for TestResource {
                fn drop(&mut self) {
                    let mut flag = self.0.lock().unwrap();
                    *flag = true;
                    println!("TestResource is dropped!");
                }
            }

            TestResource(drop_flag).use_with(|_res| {
                println!("Using the resource");
                // `_res` is consumed here
            });
        }

        assert!(*drop_flag.lock().unwrap(), "Resource was not dropped");
    }

    #[test]
    fn test_return_value() {
        struct Resource;

        impl Drop for Resource {
            fn drop(&mut self) {
                println!("Resource is dropped!");
            }
        }

        let resource = Resource;
        let result = resource.use_with(|_res| {
            // Perform operations and return a value
            42
        });

        assert_eq!(result, 42);
        // Resource should be dropped after this point
    }

    #[test]
    fn test_multiple_resources() {
        struct Resource(&'static str);

        impl Drop for Resource {
            fn drop(&mut self) {
                println!("{} is dropped!", self.0);
            }
        }

        let res1 = Resource("Resource 1");
        let res2 = Resource("Resource 2");

        res1.use_with(|r1| {
            println!("Using {}", r1.0);
        });

        res2.use_with(|r2| {
            println!("Using {}", r2.0);
        });

        // Both resources should be dropped after this point
    }

    #[test]
    fn test_nested_use_with() {
        struct Resource(&'static str);

        impl Drop for Resource {
            fn drop(&mut self) {
                println!("{} is dropped!", self.0);
            }
        }

        let outer = Resource("Outer Resource");
        let inner = Resource("Inner Resource");

        outer.use_with(|o| {
            println!("Using {}", o.0);
            inner.use_with(|i| {
                println!("Using {}", i.0);
            });
            // Inner resource should be dropped here
        });
        // Outer resource should be dropped after this point
    }

    #[test]
    #[should_panic(expected = "Intentional panic")]
    fn test_panic_in_use_with() {
        struct Resource;

        impl Drop for Resource {
            fn drop(&mut self) {
                println!("Resource is dropped!");
            }
        }

        let resource = Resource;
        resource.use_with(|_res| {
            panic!("Intentional panic");
        });

        // Resource should be dropped even after a panic
    }

    #[test]
    fn test_resource_modification() {
        struct Resource {
            value: i32,
        }

        impl Drop for Resource {
            fn drop(&mut self) {
                println!("Resource with value {} is dropped!", self.value);
            }
        }

        let resource = Resource { value: 10 };
        resource.use_with(|mut res| {
            res.value += 5;
            println!("Modified value: {}", res.value);
            assert_eq!(res.value, 15);
        });

        // Resource should be dropped after this point
    }

    #[test]
    fn test_resource_with_dependencies() {
        struct Dependency;

        impl Drop for Dependency {
            fn drop(&mut self) {
                println!("Dependency is dropped!");
            }
        }

        #[allow(dead_code)]
        struct Resource<'a> {
            dep: &'a Dependency,
        }

        impl<'a> Drop for Resource<'a> {
            fn drop(&mut self) {
                println!("Resource is dropped!");
            }
        }

        let dependency = Dependency;
        let resource = Resource { dep: &dependency };
        resource.use_with(|_res| {
            println!("Using resource with dependency");
            // `res` is consumed here
        });

        // Resource should be dropped after this point
        // Dependency will be dropped afterward
    }

    #[test]
    fn test_use_with_modifies_external_state() {
        #[derive(Default)]
        struct Resource;

        // External state that we want to modify
        let mut external_state = 0;

        Resource::default().use_with(|_res| {
            external_state += 1;
        });

        // Verify that the external state was modified
        assert_eq!(external_state, 1);
    }

    #[tokio::test]
    async fn test_use_with_async_modifies_external_state() {
        #[derive(Default)]
        struct Resource;

        // Shared state wrapped in Arc<Mutex<...>>
        let shared_state = Arc::new(tokio::sync::Mutex::new(0));

        {
            // Clone the Arc to increase the reference count
            let state = Arc::clone(&shared_state);

            // Create a new Resource and use `use_with_async` to pass an async closure
            Resource::default()
                .use_with_async(|_res| async move {
                    let mut num = state.lock().await;
                    *num += 1;
                    println!("Shared state incremented: {}", *num);
                    // Simulate asynchronous work
                    tokio::time::sleep(std::time::Duration::from_millis(100)).await;
                })
                .await;
            // `_res` is dropped here
        }

        // Verify that the shared state was modified
        assert_eq!(*shared_state.lock().await, 1);
    }
}
