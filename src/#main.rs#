use futures::join;
use std::time::Duration;
use async_std::task;

async fn big_task(name: &str) -> u8 {
   for n in 1.. 10 {
      task::sleep(Duration::from_millis(1)).await;
       n * 2;
       println!("{}, {}", name, n);
   }
   8
}

async fn tasks() -> (u8, u8) {
   let one = big_task("one");
   let two = big_task("two");
   join!(one, two)
}

#[async_std::main]
async fn main() {
   tasks().await;
   println!("{}", "Finish");
}