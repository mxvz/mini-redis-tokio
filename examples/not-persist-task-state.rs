use tokio::task::yield_now;
use std::rc::Rc;

#[tokio::main]
async fn main() {
    // https://tokio.rs/tokio/tutorial/spawning#send-bound
    tokio::spawn(async {
        let rc = Rc::new("hello");

        // `rc` is used after `.await`. It must be persisted to
        // the task's state.
        yield_now().await;

        println!("{}", rc);
    });
}