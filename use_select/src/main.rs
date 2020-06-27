use futures::{select, future::FutureExt, pin_mut};
use tokio::runtime::Runtime;
use std::io::Result;

async fn func1() -> Result<()> {
	tokio::time::delay_for(tokio::time::Duration::from_secs(2)).await;
	println!("func1 finished!");
	Ok(())
}

async fn func2() -> Result<()> {
	println!("func2 finished!");
	Ok(())
}

async fn async_main() {
	let f1 = func1().fuse();
	let f2 = func2().fuse();

	pin_mut!(f1, f2);

	select! {
		_ = f1 => println!("func1 finished++++++!"),
		_ = f2 => println!("func2 finished++++++!"),
	}
}

fn main() {
	let mut runtime = Runtime::new().unwrap();
	runtime.block_on(async_main());
    println!("Hello, world!");
}
