use tokio::task;

#[tokio::main]
async fn main() {
    // https://tokio.rs/tokio/tutorial/spawning#static-bound
    let v = vec![1, 2, 3];

    task::spawn(async {
        println!("Here's a vec: {:?}", v);
    });
}