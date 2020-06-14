// use futures::Future;
use futures::executor;

async fn hello() {
    println!("hello");
}

//==> 
//fn hello1() -> impl Future<Output=()> {
//    async {
//        println!("hello");
//    }
//}

//block_on
//await
fn main() {
    let f = hello();
    executor::block_on(f);

    my_function();
    // executor::block_on(hello());
    println!("Hello, world!");
}

fn my_function() {
    println!("my function!");
}