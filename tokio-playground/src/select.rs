use futures::channel::*;
use futures::prelude::*;

pub async fn select_1() {
    /*

    The purpose of this example is to show that select behaves deterministically.

    The output is always:
        1: 1
        1: 2
        2: 0
        2: 3
        2: 4
        1: 10
        Complete!
        Reached end of select_example

    The reason is that the select checks each arm in sequential order.

    * It checks receiver_1, success
    * It checks receiver_1, success
    * It checks receiver_1, nothing...
    * It checks receiver_2, success
    * It checks receiver_1, nothing...
    * It checks receiver_2, success
    * It checks receiver_1, nothing...
    * It checks receiver_2, success
    * It checks receiver_1, nothing...
    * It checks receiver_2, closed! (This channel is no longer considered.)
    * It checks receiver_1, nothing...

    Until...

    The task we spawn adds a message to receiver_1 and closes it.
    This results in select effectively printing the value from receiver_1, and
    checking receiver_1 again which is when it realises that receiver_1 has been closed.

    This triggers the complete arm.

    The exact details may be inaccurate, but in terms of the observable behaviour, I think this is goo enough.

    This behaviour can be different from what one expects.
    A arm is only scheduled if the previous arm's future wasn't ready.
    This can result in code that looks asynchronous but is actually sequential.

    Consider example_deterministic_select_concrete...

    */

    let (mut sender_1, receiver_1) = mpsc::channel::<u64>(64);
    let (mut sender_2, receiver_2) = mpsc::channel::<u64>(64);

    let mut receiver_1 = receiver_1.fuse();
    let mut receiver_2 = receiver_2.fuse();

    sender_2.send(0).await.unwrap();
    sender_1.send(1).await.unwrap();
    sender_1.send(2).await.unwrap();
    sender_2.send(3).await.unwrap();
    sender_2.send(4).await.unwrap();

    let (os_sender, os_receiver) = oneshot::channel::<()>();
    let mut sender_1_clone = sender_1.clone();
    tokio::spawn(async move {
        tokio::time::delay_for( std::time::Duration::from_secs(5)).await;
        sender_1.send(10).await.unwrap();
        sender_1_clone.close().await.unwrap();
        os_sender.send(()).unwrap();
    });

    sender_2.close().await.unwrap();

    loop {
        futures::select!(
            msg = receiver_1.next() => {
                match msg {
                    Some(msg) => {
                        println!("1: {}", msg);
                    },
                    None => {}
                }
            },
            msg = receiver_2.next() => {
                match msg {
                    Some(msg) => {
                        println!("2: {}", msg);
                    },
                    None => {}
                }
            },
            complete => {
                println!("Complete!");
                break;
            }
        );
    }

    os_receiver.await.unwrap();

    println!("Reached end of select_example");
}

async fn process(channel_id: u64, msg: Option<u64>) {
    match msg {
        Some(msg) => {
            // You'd hope that this delay will result in select checking the second arm...
            tokio::time::delay_for( std::time::Duration::from_secs(5)).await;
            println!("{}: {}", channel_id, msg);
        }
        None => {}
    }
}

pub async fn select_2() {
    let (mut sender_1, receiver_1) = mpsc::channel::<u64>(64);
    let (mut sender_2, receiver_2) = mpsc::channel::<u64>(64);

    let mut receiver_1 = receiver_1.fuse();
    let mut receiver_2 = receiver_2.fuse();

    sender_2.send(0).await.unwrap();
    sender_1.send(1).await.unwrap();
    sender_1.send(2).await.unwrap();
    sender_2.send(3).await.unwrap();
    sender_2.send(4).await.unwrap();

    sender_1.close().await.unwrap();
    sender_2.close().await.unwrap();

    loop {
        futures::select!(
            msg = receiver_1.next() => process(1, msg).await,
            msg = receiver_2.next() => process(2, msg).await,
            complete => {
                println!("Complete!");
                break;
            }
        );
    }
}
