use futures;
use tokio::runtime::Runtime;
use std::io::Result;

async fn func1() -> Result<()> {
    tokio::time::delay_for(tokio::time::Duration::from_secs(1)).await;
    println!("func1 finished!");
	Ok(())
}

async fn func2() -> Result<()> {
    println!("func2 finished!");
	Ok(())
}

async fn async_main() {
    let f1 = func1();
    let f2 = func2();

    if let Err(_) = futures::try_join!(f1, f2) {
		println!("Err!");
	}
}

fn main() {
    let mut runtime = Runtime::new().unwrap();
    runtime.block_on(async_main());
    println!("Hello, world!");
}
