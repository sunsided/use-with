pub trait Use {
    fn use_with<T, F: FnOnce(Self) -> T>(self, f: F) -> T
    where
        Self: Sized;
}

impl<T> Use for T {
    fn use_with<U, F: FnOnce(Self) -> U>(self, f: F) -> U
    where
        Self: Sized,
    {
        f(self)
    }
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
}
