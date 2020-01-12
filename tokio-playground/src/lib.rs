#[cfg(test)]
mod tests {

    use futures::prelude::*;
    use futures::channel::mpsc as futures_mpsc;
    use tokio::sync::mpsc as tokio_mpsc;

    #[test]
    fn channel_comparison() {

        // TODO:
        // Is it ever more appropriate to use tokio_mpsc over futures_mpsc?
        // Or, are the two channels interchangeable?

        let mut runtime = tokio::runtime::Runtime::new().expect("failed to create Tokio runtime");
        runtime.block_on(async {
            let (mut sender, mut receiver) = futures_mpsc::channel::<usize>(1024);
            let mut sender_clone = sender.clone();
            tokio::spawn(async move {
                for _ in 0..10 {
                    sender_clone.send(0).await.expect("failed to send");
                }
            });

            tokio::spawn(async move {
                for _ in 0..10 {
                    sender.send(1).await.expect("failed to send");
                }
            });

            loop {
                if let Some(msg) = receiver.next().await {
                    println!("{}", msg);
                } else {
                    break;
                }
            }
        });

        let mut runtime = tokio::runtime::Runtime::new().expect("failed to create Tokio runtime");
        runtime.block_on(async {
            let (mut sender, mut receiver) = tokio_mpsc::channel::<usize>(1024);
            let mut sender_clone = sender.clone();
            tokio::spawn(async move {
                for _ in 0..10 {
                    sender_clone.send(0).await.expect("failed to send");
                }
            });

            tokio::spawn(async move {
                for _ in 0..10 {
                    sender.send(1).await.expect("failed to send");
                }
            });

            loop {
                if let Some(msg) = receiver.next().await {
                    println!("{}", msg);
                } else {
                    break;
                }
            }
        });
    }
}