use futures::future::{BoxFuture, FutureExt};

fn re() -> BoxFuture<'static, ()> {
	async move{
		re().await;
		re().await;
	}.boxed()
}

fn main() {
	re();
    println!("Hello, world!");
}
