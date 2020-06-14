use futures::{self, executor};
use std::thread::sleep;
use std::time::Duration;

async fn learn_song() {
    sleep(Duration::from_secs(5));
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
}

fn main() {
    executor::block_on(async_main());
    println!("Hello, world!");
}
