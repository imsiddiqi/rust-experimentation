pub mod channel_implementations;
pub mod select;

#[cfg(test)]
mod tests {

    #[test]
    fn asymmetric_send_recv() {
        use crate::channel_implementations::asymmetric::Channel;
        use serde::{Deserialize, Serialize};
        use std::net::Ipv4Addr;
        use std::str::FromStr;

        #[derive(Debug, PartialEq, Deserialize, Serialize)]
        pub struct Request(String);
        #[derive(Debug, PartialEq, Deserialize, Serialize)]
        pub struct Response(String);

        tokio::runtime::Runtime::new().expect("failed to create Tokio runtime").block_on(async {
            let handle_1 = tokio::spawn(async {
                let address = Ipv4Addr::from_str("127.0.0.1")
                    .expect("failed to construct address");
                let mut channel: Channel<Request, Response> = Channel::accept(&address, 20000).await
                    .expect("failed to accept connection");

                let (mut sender, mut receiver) = channel.split();

                // Send message:
                sender.send(Request(String::from("123"))).await.unwrap();

                // Receive message:
                let msg = receiver.recv().await.unwrap();
                assert_eq!(msg, Some(Response(String::from("321"))));

                // Send message:
                sender.send(Request(String::from("456"))).await.unwrap();

                // Receive message:
                let msg = receiver.recv().await.unwrap();
                assert_eq!(msg, Some(Response(String::from("654"))));
            });

            let handle_2 = tokio::spawn(async {
                let address = Ipv4Addr::from_str("127.0.0.1")
                    .expect("failed to construct address");
                let mut channel: Channel<Response, Request> = Channel::connect(&address, 20000).await
                    .expect("failed to accept connection");

                let (mut sender, mut receiver) = channel.split();

                // Receive message:
                let msg = receiver.recv().await.unwrap();
                assert_eq!(msg, Some(Request(String::from("123"))));

                // Send message:
                sender.send(Response(String::from("321"))).await.unwrap();

                // Receive message:
                let msg = receiver.recv().await.unwrap();
                assert_eq!(msg, Some(Request(String::from("456"))));

                // Send message:
                sender.send(Response(String::from("654"))).await.unwrap();
            });

            handle_2.await.unwrap();
            handle_1.await.unwrap();
        });
    }

    #[test]
    fn symmetric_send_recv() {
        use crate::channel_implementations::symmetric::Channel;
        use std::net::Ipv4Addr;
        use std::str::FromStr;

        tokio::runtime::Runtime::new().expect("failed to create Tokio runtime").block_on(async {
            let handle_1 = tokio::spawn(async {
                let address = Ipv4Addr::from_str("127.0.0.1")
                    .expect("failed to construct address");
                let mut channel: Channel<String> = Channel::accept(&address, 21000).await
                    .expect("failed to accept connection");

                let (mut sender, mut receiver) = channel.split();

                // Send message:
                sender.send(String::from("123")).await.unwrap();

                // Receive message:
                let msg = receiver.recv().await.unwrap();
                assert_eq!(msg, Some(String::from("321")));

                // Send message:
                sender.send(String::from("456")).await.unwrap();

                // Receive message:
                let msg = receiver.recv().await.unwrap();
                assert_eq!(msg, Some(String::from("654")));
            });

            let handle_2 = tokio::spawn(async {
                let address = Ipv4Addr::from_str("127.0.0.1")
                    .expect("failed to construct address");
                let mut channel: Channel<String> = Channel::connect(&address, 21000).await
                    .expect("failed to accept connection");

                let (mut sender, mut receiver) = channel.split();

                // Receive message:
                let msg = receiver.recv().await.unwrap();
                assert_eq!(msg, Some(String::from("123")));

                // Send message:
                sender.send(String::from("321")).await.unwrap();

                // Receive message:
                let msg = receiver.recv().await.unwrap();
                assert_eq!(msg, Some(String::from("456")));

                // Send message:
                sender.send(String::from("654")).await.unwrap();
            });

            handle_2.await.unwrap();
            handle_1.await.unwrap();
        });
    }

    #[test]
    fn channel_comparison() {

        // TODO: What is the difference between `futures::channel` and `tokio::sync`?

        use futures::prelude::*;
        use futures::channel::mpsc as futures_mpsc;
        use tokio::sync::mpsc as tokio_mpsc;

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

    #[test]
    fn select_1() {
        tokio::runtime::Runtime::new()
            .expect("failed to create Tokio runtime")
            .block_on(async {
                crate::select::select_1().await;
            });
    }

    #[test]
    fn select_2() {
        tokio::runtime::Runtime::new()
            .expect("failed to create Tokio runtime")
            .block_on(async {
                crate::select::select_2().await;
            });
    }
}
