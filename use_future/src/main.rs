use std::future::Future;

// async fn hello() {
//     println!("hello");
// }

fn hello() -> impl Future<Output=()> {
    async {
        println!("hello");
    }
}

// trait SimpleFuture {
//     type Output;
//     fn poll(&mut self, wake: fn()) -> Poll<Self::Output>;
// }

// //通过Pin可以创建不可移动的Future。不可移动的对象可以在它们的字段之间存储指针，例如：
// struct MyFut {
//     a: i32,
//     ptr: *const i32,
// }

// pub trait Future {
//     type Output;
//     fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>;
// }


fn main() {
    println!("Hello, world!");
}
