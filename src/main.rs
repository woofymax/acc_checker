mod run;
use tokio::prelude::*;

#[tokio::main]
async fn main() {
  run::init().await;
}

