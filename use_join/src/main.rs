use futures;
use tokio::runtime::Runtime;

async fn func1() {
    tokio::time::delay_for(tokio::time::Duration::from_secs(1)).await;
    // sleep(1);
    println!("func1 finished!");
}

async fn func2() {
    println!("func2 finished!");
}

async fn async_main() {
    let f1 = func1();
    let f2 = func2();

    futures::join!(f1, f2);
}

fn main() {
    let mut runtime = Runtime::new().unwrap();
    runtime.block_on(async_main());

    // futures::executor::block_on(async_main());

    println!("Hello, world!");
}
