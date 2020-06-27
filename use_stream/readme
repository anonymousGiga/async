作者：令狐一冲
## Stream 介绍
`Stream`和`Future`类似，但是`Future`对应的是一个`item`的状态的变化，而`Stream`则是类似于`iterator`，在结束之前能够得到多个值。或者我们可以简单的理解为，`Stream`是由一系列的`Future`组成，我们可以从`Stream`读取各个`Future`的结果，直到`Stream`结束。

## Stream trait定义
定义如下：
```
trait Stream {
    type Item;

    fn poll_next(self: Pin<&mut Self>, lw: &LocalWaker)
        -> Poll<Option<Self::Item>>;
}
```
`poll_next`函数有三种可能的返回值，分别如下：
- `Poll::Pending` 说明下一个值还没有就绪，仍然需要等待。
- `Poll::Ready(Some(val))` 已经就绪，成功返回一个值，程序可以通过调用`poll_next`再获取下一个值。
- `Poll::Ready(None)` 表示`Stream`已经结束，不应该在调用`poll_next`。

## 迭代
和同步的`Iterator`类似，`Stream`可以迭代处理其中的值，如使用`map, filter, fold,  try_map, try_filter, and try_fold`等。但是`Stream`不支持使用`for`，而`while let`和 `next/try_next`则是允许的。
例子如下：
```
async fn sum_with_next(mut stream: Pin<&mut dyn Stream<Item = i32>>) -> i32 {
    use futures::stream::StreamExt; // for `next`
    let mut sum = 0;
    while let Some(item) = stream.next().await {
        sum += item;
    }
    sum
}

async fn sum_with_try_next(
    mut stream: Pin<&mut dyn Stream<Item = Result<i32, io::Error>>>,
) -> Result<i32, io::Error> {
    use futures::stream::TryStreamExt; // for `try_next`
    let mut sum = 0;
    while let Some(item) = stream.try_next().await? {
        sum += item;
    }
    Ok(sum)
}
```

## 并发
上面的使用的迭代处理，如果我们要并发的处理流，则应该使用`for_each_concurrent` 和 `try_for_each_concurrent`，示例如下：
```
async fn jump_around(
    mut stream: Pin<&mut dyn Stream<Item = Result<u8, io::Error>>>,
) -> Result<(), io::Error> {
    use futures::stream::TryStreamExt; // for `try_for_each_concurrent`
    const MAX_CONCURRENT_JUMPERS: usize = 100;

    stream.try_for_each_concurrent(MAX_CONCURRENT_JUMPERS, |num| async move {
        jump_n_times(num).await?;
        report_n_jumps(num).await?;
        Ok(())
    }).await?;

    Ok(())
}
```

## 参考资料
Rust异步编程
