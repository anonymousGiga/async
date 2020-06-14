use std::future::Future;

fn my_function() -> impl Future<Output  = u8> {
	let closure = async |x: u8| {
		x
	};
	
	closure(5)
}

fn main() {
    println!("Hello, world!");
}
