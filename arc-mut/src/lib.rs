#[cfg(test)]
mod tests {

    /*

    Although this benchmark most likely isn't representative of real-world scenarios,
    it shows that it's cheaper to clone a mpsc sender than wrapping it in an atomic reference counter.
    This comes into consideration when you'd like to have internal mutability.

    */

    use tokio::sync::mpsc;
    use futures::lock::Mutex;
    use std::sync::Arc;

    struct VariantA {
        sender: mpsc::Sender<()>,
    }

    impl VariantA {
        fn new(sender: mpsc::Sender<()>) -> Self {
            Self{
                sender,
            }
        }

        async fn work(&self) {
            self.sender.clone().send(()).await.expect("variant-a: could not send");
        }
    }

    struct VariantB {
        sender: Arc<Mutex<mpsc::Sender<()>>>,
    }

    impl VariantB {
        fn new(sender: mpsc::Sender<()>) -> Self {
            Self{
                sender: Arc::new(Mutex::new(sender)),
            }
        }

        async fn work(&self) {
            let sender = self.sender.lock().await;
            sender.clone().send(()).await.expect("variant-b: could not send");
        }
    }

    #[test]
    fn test_a() {
        tokio::runtime::Runtime::new().expect("could not create Tokio runtime").block_on(async {
            let (sender, _receiver) = mpsc::channel(1048576);
            let a = VariantA::new(sender);

            let start_time = std::time::Instant::now();
            for _ in 0..100000 {
                a.work().await;
            }
            let end_time = std::time::Instant::now();
            println!("A: {:?}", (end_time - start_time));
        });
    }

    #[test]
    fn test_b() {
        tokio::runtime::Runtime::new().expect("could not create Tokio runtime").block_on(async {
            let (sender, _receiver) = mpsc::channel(1048576);
            let b = VariantB::new(sender);

            let start_time = std::time::Instant::now();
            for _ in 0..100000 {
                b.work().await;
            }
            let end_time = std::time::Instant::now();
            println!("B: {:?}", (end_time - start_time));
        });
    }
}
