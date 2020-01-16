#[cfg(test)]
mod tests {

    use futures::lock::Mutex as AsyncMutex;
    use std::collections::BTreeMap;
    use std::sync::Arc;

    #[test]
    fn shared_mutable_references() {
        let mut runtime = tokio::runtime::Runtime::new().expect("failed to create tokio runtime");
        runtime.block_on(async {
            let map: BTreeMap<usize, usize> = Default::default();
            let map_1 = Arc::new(AsyncMutex::new(map));
            let map_2 = map_1.clone();

            map_1.lock().await.insert(0, 0);

            // `tokio::spawn` expects `Send` marker to be implemented.
            // Our map cannon be on the stack?
            // If it's not on the stack, access needs to be protected?
            // Since we cannot know if someone else is also accessing the map?
            let handle = tokio::spawn(async move {
                map_2.lock().await.remove(&0).expect("not found (clone didn't work as expected?)");
                map_2.lock().await.insert(1, 1);
            });

            handle.await.expect("failed to join on Tokio task");
            map_1.lock().await.remove(&1).expect("not found (clone didn't work as expected?)");
        });
    }
}
