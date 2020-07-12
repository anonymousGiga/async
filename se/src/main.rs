use std::rc::Rc;

#[derive(Default)]
struct NoSend(Rc<()>);

async fn bar() {}

async fn foo() {
	NoSend::default();

	//{
	//	let x = NoSend::default();
	//	//to do : xxxxx
	//}

	let _ = NoSend::default();

	bar().await;
}

//Send trait：如果所有的子类型都是实现Send trait的，那么它本身也是实现Send Trait的

////not let x: impl Send Trait
//struct Foo {
//	f: Future,
//}

////let x: Not impl Send Trait
//struct Foo {
//	x: NoSend, //not impl Send Trait
//	f: Future, //impl Send Trait
//}

fn required_send(_: impl Send) {}

fn main() {
	required_send(foo());

    println!("Hello, world!");
}
