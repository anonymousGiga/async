use futures;

async fn foo() -> Result<(), String> {
	Ok(())
}

async fn func() -> Result<(), String> {
	let fut = async {
		foo().await?;
		//Ok(())
		Ok::<(), String>(())
	};

	fut.await
}

fn main() {
	let _ = futures::executor::block_on(func());
    println!("Hello, world!");
}
