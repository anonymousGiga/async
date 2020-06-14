use futures::executor;

async fn move_block() {
    let my_string = "my string".to_string();

    let f = async move {
        println!("String: {}", my_string);
    };

    // println!("after move, String: {}", my_string);

    f.await
}

fn main() {
    executor::block_on(move_block());
}