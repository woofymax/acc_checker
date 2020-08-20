mod run;

#[tokio::main]
async fn main() {
  run::init().await;
}

