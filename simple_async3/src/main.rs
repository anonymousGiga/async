use async_std::task;
// use std::thread::sleep;
use std::time::Duration;

async fn learn_song() {
    //sleep(Duration::from_secs(5));
    task::sleep(Duration::from_secs(1)).await;
    println!("learn song");
}

async fn sing_song() {
    println!("sing song");
}

async fn dance() {
    println!("dance");
}

async fn learn_and_sing_song() {
    learn_song().await;
    sing_song().await;
}

async fn async_main() {
    let f1 = learn_and_sing_song();
    let f2 = dance();

    futures::join!(f1, f2);
    async_std::future::Future
    // f1.await;
    // f2.await;
}

fn main() {
    executor::block_on(async_main());
    println!("Hello, world!");
}
