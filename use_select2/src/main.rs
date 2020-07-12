use futures::{future, select, executor};

async fn count() {
	let mut a_fut = future::ready(4);
	let mut b_fut = future::ready(6);
	let mut total = 0;	

	loop {
		select! {
			a = a_fut => total += a,
			b = b_fut => total += b,
			complete => break,   //表示所有的分支都已经完成，并且不会再取得进展的情况
			default => unreachable!(), //表示没有分支完成
		}
	}

	assert_eq!(total, 10);
}

fn main() {
	executor::block_on(count());
    println!("Hello, world!");
}
