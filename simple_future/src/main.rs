use std::thread;
use std::time::Duration;

trait SimpleFuture {
    type Output;
    //fn poll(&mut self, wake: fn()) -> Poll<Self::Output>;
    fn poll(&mut self, wake: u32) -> Poll<Self::Output>;
}

enum Poll<T> {
    Ready(T),
    Pending,
}

struct MySleeper {
    polls: u64,
    wake: u32,
}

static mut FINISHED: bool = false;

impl MySleeper {
    fn new(wake: u32) -> Self {
        MySleeper {
            polls: 0,
            wake: wake,
        }
    }
}

impl SimpleFuture for MySleeper {
    type Output = ();
    fn poll(&mut self, wake: u32) -> Poll<Self::Output> {
        unsafe {
            if FINISHED {
                Poll::Ready(())
            } else {
                self.wake = wake;
                self.polls += 1;
                println!("polls = {}", self.polls);
                Poll::Pending
            }
        }
    }
}

struct MyReactor {
    wake: u32,
    handle: Option<thread::JoinHandle<()>>,
}

impl MyReactor {
    fn new() -> Self {
        MyReactor {
            wake: 0,
            handle: None,
        }
    }

    fn add_wake(&mut self, wake: u32) {
        self.wake = wake;
    }

    fn check_status(&mut self) {
        if self.handle.is_none() {
            let _wake = self.wake;
            let handle = thread::spawn(|| loop {
                thread::sleep(Duration::from_secs(5));
                unsafe {
                    //模拟future就绪，然后调用wake
                    FINISHED = true;
                }
            });
            self.handle = Some(handle);
        }
    }
}

struct MyExecutor;

impl MyExecutor {
    fn block_on<F: SimpleFuture>(mut myfuture: F, wake: u32) {
        loop {
            match myfuture.poll(wake) {
                Poll::Ready(_) => {
                    println!("my future is ok!");
                    break;
                }
                Poll::Pending => unsafe {
                    while !FINISHED {
                        thread::sleep(Duration::from_secs(1));
                    }
                },
            }
        }
    }
}

fn main() {
    let mut reactor = MyReactor::new();
    let sleeper = MySleeper::new(5);
    let wake = sleeper.wake;
    reactor.add_wake(wake);
    reactor.check_status();
    MyExecutor::block_on(sleeper, wake);
}
