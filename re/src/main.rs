use futures;

async fn first() {}
async fn second() {}

async fn foo() {
	first().await;
	second().await;
}

////-------------
//enum Foo{
//	First(first),
//	Second(second),
//}
//

//enum FooState {
//	F1_Pending,
//	F1_Ready,
//	F2_Pending,
//	F2_Ready,
//  Ready
//}
//struct Foo {
//	f1: Foo::First,
//	f2: Foo::Second,
//	state: FooState, 
//}
////--------------

async fn re() {
	re().await;
	re().await;
}

//enum Ree {
//	First(Re),
//	Second(Re),
//}
//
//struct Re {
//	f1: Ree::First,
//	f2: Ree::Second,
//	//state,
//}

fn main() {
	futures::executor::block_on(foo());
	futures::executor::block_on(re());
    println!("Hello, world!");
}
