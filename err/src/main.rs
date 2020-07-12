use futures;

async fn foo() {
	//"foo"
}

fn bar() {
	"bar"
}


fn main() {
	futures::executor::block_on(foo());
    println!("Hello, world!");

	bar();
}
