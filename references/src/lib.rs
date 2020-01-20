#[cfg(test)]
mod tests {

    use futures::lock::Mutex as AsyncMutex;
    use std::collections::BTreeMap;
    use std::sync::Arc;
    use std::ops::Deref;

    fn test(s: &str) {
        println!("{}", s);
    }

    struct Wrapper {
        s: &'static str,
    }

    impl Wrapper {
        fn new() -> Self {
            Self{ s: "Hello?" }
        }
    }

    impl Deref for Wrapper {
        type Target = &'static str;

        fn deref(&self) -> &Self::Target {
            return &self.s;
        }
    }

    #[test]
    fn dereference() {
        let s = String::from("Test");

        test(&s);
        test(*Wrapper::new());
        test(&Wrapper::new());

        let a: &String = &s;
        let b: &str = *Wrapper::new();
        let c: &Wrapper = &Wrapper::new();

        test(a);
        test(b);
        test(c);

        let a_deref: &str = &a;
        let b_deref: &str = &b;
        let c_deref: &str = &c;

        test(a_deref);
        test(b_deref);
        test(c_deref);
    }

    #[test]
    fn shared_mutable_references() {

        // `tokio::spawn` expects `Send` marker to be implemented.
        // Thus, our map cannot be on the stack?
        // If it's not on the stack, access needs to be protected?
        // Since we cannot know if someone else is also accessing the map?

        let mut runtime = tokio::runtime::Runtime::new().expect("failed to create Tokio runtime");
        runtime.block_on(async {
            let map: BTreeMap<usize, usize> = Default::default();
            let map_1 = Arc::new(AsyncMutex::new(map));
            let map_2 = map_1.clone();

            map_1.lock().await.insert(0, 0);
            let handle = tokio::spawn(async move {
                map_2.lock().await.remove(&0).expect("key not found (clone didn't work as expected?)");
                map_2.lock().await.insert(1, 1);
            });

            handle.await.expect("failed to join on Tokio task");
            map_1.lock().await.remove(&1).expect("key not found (clone didn't work as expected?)");
        });
    }
}
