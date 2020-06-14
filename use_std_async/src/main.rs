use async_std::task;
use std::time::Duration;

async fn learn_song() {
    task::sleep(Duration::from_secs(2)).await;
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

    let f = f1.join(f2);
    f.await;
}

fn main() {
    task::block_on(async_main());
    println!("Hello, world!");
}