use log::Level;
use tokio::{io::AsyncReadExt, time};

async fn sleeper(name: &str) {
    log::info!("{}: Sleeping", name);
    time::sleep(time::Duration::from_secs(1)).await;
    log::info!("{}: Awake!", name);
}

async fn reader() {
    log::info!("Reading some beeg data");
    let mut f = tokio::fs::File::open("beeg.csv").await.unwrap();
    let mut contents = vec![];
    f.read_to_end(&mut contents).await.unwrap();
    log::info!("Read beeg {} bytes", contents.len());
}

async fn run() {
    tokio::spawn(async {
        sleeper("A").await;
    });
    sleeper("B").await;

    // sleeper("first").await;
    // sleeper("second").await;

    // sleeper().await;
    // reader().await;

    // tokio::join!(sleeper(), reader(), reader(), reader());
}

fn main() {
    simple_logger::init_with_level(Level::Info).unwrap();
    let start = std::time::Instant::now();

    let rt = tokio::runtime::Runtime::new().unwrap();
    let future = run();

    rt.block_on(future);
    log::info!("It took {}", start.elapsed().as_millis());
}
