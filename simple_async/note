## async/.await简单介绍

此节对async/await做简单介绍，旨在让大家有一个简单的认识。

## 作用
async/.await是Rust编写异步的内置工具。
async将一个代码块转化为实现了future特征的状态机。
那么，转化为future后有什么作用呢？
答案：在同步方法中调用阻塞函数（async转化的函数）会阻塞整个线程，但是，阻塞的future会让出线程控制权，允许其它future运行。

## 部分语法
* 准备工作：配置文件Cargo.toml
```
[dependencies]
futures = "0.3.4"
```

* 创建异步函数

创建异步函数的语法：
```
async fn my_function() {
    println!("Hello");
}
```
通过async关键字，上面的函数返回一个Future。换句话说，上面的函数等价于如下代码：
```
fn my_function() -> impl Future<Output = ()> {
    async {
        println!("Hello");
    }
}
```

* 调用异步函数

上面简介绍了创建异步函数的语法，下面我们看下如何调用异步函数。

（１）通过block_on阻塞调用
```
//例子一
use futures::executor;

async fn my_function() {
    println!("Hello");
}

fn main() {
    let f = my_function();
    executor::block_on(f);
}
```
（２）通过.await调用
```
//例子二
use futures::{ self, executor};

async fn learn_song() {
    println!("Learn song!");
}

async fn sing_song() {
    println!("Sing song!");
}

async fn dance() {
    println!("Dance!");
}

async fn learn_and_sing_song() {
    learn_song().await;
    sing_song().await;
}

async fn async_main() {
    let f1 = learn_and_sing_song();
    let f2 = dance();
    futures::join!(f1, f2);
}
fn main() {
    executor::block_on(async_main());
    println!("Hello, world!");
}
```
说明：

a、在learn_and_sing_song()中，会先执行learn_song()，然后再执行sing_song()，两者按照顺序执行；

b、通过join，能等待多个Future完成；

c、当await发生阻塞时，不会阻塞当前线程，可以让其它的任务执行（在此例子中，如果在learn_song阻塞，则learn_and_sing_song会让出当前线程，可以让dance执行）。

## 总结
本节主要简单介绍async/await在异步函数方面的用法，同时通过sing、song、dance的例子，展示了async的应用。

但是，本例子中只是简单的语法展示，并不能真正达到我们想要的异步效果。

**那么，为什么无法达到我们想要的效果，请持续关注我们Rust异步编程的学习笔记。**



