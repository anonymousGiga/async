use futures::executor;

async fn async_function1() {
    println!("async function1 ++++ !");
}

async fn async_function2() {
    println!("async function2 ++++ !");
}

async fn async_main() {
    let f1 = async_function1();
    let f2 = async_function2();
    
    //重点关注这里---------
    let f = async move {
        f1.await;
        f2.await; 
    };
    //---------------------
    f.await;
    // futures::join(f, async_function3());
}

////分析编译器展开的过程:
//// (1)创建一个匿名结构体
//// （2）为结构体定义了对应的状态
//// （3）实现Future trait
//struct AsyncFuture {
//    fut_one: FutFunction1,
//    fut_two: FutFunction2,
//    state: State,
//}
//
//enum State {
//    AwaitFut1,
//    AwaitFut2,
//    Done,
//}
//
//impl Future for AsyncFuture {
//    type Output = ();
//
//    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
//        loop {
//            match self.state {
//                State::AwaitFut1 => match self.fut_one.poll(...) {
//                    Poll::Ready(()) => self.state = State::AwaitFut2,
//                    Poll::Pending => return Poll::Pending,
//                }
//                State::AwaitFut2 => match self.fut_two.poll(...) {
//                    Poll::Ready(()) => self.state = State::Done,
//                    Poll::Pending => return Poll::Pending,
//                }
//                State::Done => return Poll::Ready(());
//            }
//        }
//    }
//}


// async fn async_function3() {
    // 
// }

fn main() {
    executor::block_on(async_main());
}

//------------------------------------------------------
// 分析pin

async fn async_put_data_to_buf(mut buf: &[u8]) {
    //to do something
}

async fn async_main() {
    let f = async {
        let mut x = [0: 128];
        let async_put = async_put_data_to_buf(&mut x);
        async_put.await;
    };
}

////编译器展开
//struct AsyncFuture {
//    x: [u8: 128],
//    async_put: PutIntoBuf<'what_lifetime>,
//}
//
//struct PutIntoBuf {
//    buf: &'a mut[u8],
//}

////如果AsyncFuture发生移动，那么x和async_put都发生移动，
//// 但是async_put.buf还是指向移动之前的x，显然不是我们期望的
//// 我们期望的是async_put.buf指向移动之后的x
//
//// Pin类型包着指针类型，保证指针类型背后的值不被移动
//// Pin<T>
//// 大多数类型都不存移动的问题，这些类型实现UnPin trait，u8
//// let f = Pin<AsyncFuture>
//
//let f = async {}
//
//fn my_function<T>(T: impl UnPin)
//
//let fut = Box::pin(f);
//pin_mut!(fut);
//my_function(fut);