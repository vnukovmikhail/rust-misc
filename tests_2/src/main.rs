use std::{
    cell::Cell,
    sync::{Arc, Mutex, RwLock, mpsc},
    thread::{self},
    time::{Duration, Instant},
};

use tokio::time::sleep;
use tokio_stream::StreamExt;

thread_local! {
    static MY_COUNTER: Cell<u32> = Cell::new(0);
}

#[tokio::main]
async fn main() {
    let t1 = thread::spawn(|| {
        for i in 0..10 {
            println!("[{:?}] Thread 1: {}", thread::current().id(), i);
            thread::sleep(Duration::from_millis(10));
        }
    });
    let t2 = thread::spawn(|| {
        for i in 0..10 {
            println!("[{:?}] Thread 2: {}", thread::current().id(), i);
            thread::sleep(Duration::from_millis(10));
        }
    });
    t1.join().unwrap();
    t2.join().unwrap();
    println!("[{:?}] Main finished", thread::current().id());

    let counter = Arc::new(Mutex::new(0));
    let counter1 = counter.clone();
    let counter2 = Arc::clone(&counter);
    let t1 = thread::spawn(move || {
        let mut num = counter1.lock().unwrap();
        *num += 1;
        println!("Thread 1 Counter: {}", *num);
    });
    let t2 = thread::spawn(move || {
        let mut num = counter2.lock().unwrap();
        *num += 2;
        println!("Thread 2 Counter: {}", *num);
    });
    t1.join().unwrap();
    t2.join().unwrap();
    println!("Final Counter: {:?}", counter);

    let counter = Arc::new(RwLock::new(0));
    let counter1 = counter.clone();
    let counter2 = Arc::clone(&counter);
    let counter_reader1 = counter.clone();
    let counter_reader2 = Arc::clone(&counter);
    let t1 = thread::spawn(move || {
        let mut num = counter1.write().unwrap();
        *num += 1;
        println!("Thread 1 (Writer) updated counter to: {}", *num);
    });
    let t2 = thread::spawn(move || {
        let mut num = counter2.write().unwrap();
        thread::sleep(Duration::from_millis(70));
        *num += 2;
        println!("Thread 2 (Writer) updated counter to: {}", *num);
    });
    let t3 = thread::spawn(move || {
        thread::sleep(Duration::from_millis(10));
        let num = counter_reader1.read().unwrap();
        println!("Thread 3 (Reader A) sees counter as: {}", *num);
    });
    let t4 = thread::spawn(move || {
        thread::sleep(Duration::from_millis(15));
        let num = counter_reader2.read().unwrap();
        println!("Thread 4 (Reader B) sees counter as: {}", *num);
    });
    t1.join().unwrap();
    t2.join().unwrap();
    t3.join().unwrap();
    t4.join().unwrap();
    println!("Final Counter: {:?}", counter.read().unwrap());

    let t1 = thread::spawn(|| {
        MY_COUNTER.with(|counter| {
            counter.set(counter.get() + 10);
            println!("Thread 1 Counter: {}", counter.get());
        });
    });
    let t2 = thread::spawn(|| {
        MY_COUNTER.with(|counter| {
            counter.set(counter.get() + 20);
            println!("Thread 2 Counter: {}", counter.get());
        });
    });
    t1.join().unwrap();
    t2.join().unwrap();
    MY_COUNTER.with(|counter| {
        println!("Main Thread Counter: {}", counter.get());
    });

    let (tx, rx) = mpsc::channel();
    let tx_clone = tx.clone();
    thread::spawn(move || {
        tx.send("Hello from Thread 1!").unwrap();
    });
    thread::spawn(move || {
        tx_clone.send("Hello from Thread 2!").unwrap();
    });
    // for received in rx {
    //     println!("Got: {}", received);
    // }
    loop {
        match rx.recv() {
            Ok(msg) => println!("Main thread got: {}", msg),
            Err(_) => {
                println!("All threads finished and senders dropped. Exiting...");
                break;
            }
        }
    }

    let start_time = Instant::now();
    let download_handle = tokio::spawn(download_file());
    for i in 1..=3 {
        println!("UI is responsive: Animation frame {} playing...", i);
        sleep(tokio::time::Duration::from_secs(1)).await;
    }
    println!("Main thread is done with other work. Let's check on the file...");
    let _ = download_handle.await;
    println!("Program finished! {:?}ms", start_time.elapsed().as_millis());

    let start_time = std::time::Instant::now();
    let profile_future = fetch_user_profile();
    let posts_future = fetch_user_posts();
    println!("Starting both operations concurrently...");
    // let (user_data, posts_data) = tokio::join!(profile_future, posts_future);
    // println!("Success");
    // println!("User: {}", user_data);
    // println!("Posts count: {}", posts_data.len());
    tokio::select! {
        user_data = profile_future => {
            println!("Winner: User profile fetched first!");
            println!("User: {}", user_data);
        }
        posts_data = posts_future => {
            println!("Winner: User posts fetched first!");
            println!("Posts count: {}", posts_data.len());
        }
    }
    println!("Elapsed time: {}ms", start_time.elapsed().as_millis());

    let start_time = std::time::Instant::now();
    let video_chunks = vec![
        "Chunk A (Intro)",
        "Chunk B (Main Scene)",
        "Chunk C (Credits)",
    ];
    let mut video_stream = tokio_stream::iter(video_chunks);
    println!("Pressing play on video player...");
    while let Some(chunk) = video_stream.next().await {
        println!("Buffering next segment...");
        sleep(tokio::time::Duration::from_millis(1500)).await;
        println!("Now playing: {}", chunk);
    }
    println!("Video playback completed successfully!");
    println!(
        "Total execution time: {:?}",
        start_time.elapsed().as_millis()
    );

    let start_time = std::time::Instant::now();
    let (tx, mut rx) = tokio::sync::mpsc::channel::<String>(10);
    tokio::spawn(async move {
        let tasks = vec!["Resize Image", "Apply Filter", "Compress File"];

        for task in tasks {
            println!("[Projucer] Generating task: {}", task);

            if tx.send(task.to_string()).await.is_err() {
                println!("[Projucer] Receiver was dropped! Stopping.");
                break;
            }

            sleep(tokio::time::Duration::from_millis(500)).await;
        }
        println!("[Projucer] Done sending tasks!");
    });
    println!("[Consumer] Waiting to receive and process tasks...");
    while let Some(message) = rx.recv().await {
        println!("[Consumer] Starting work on: {}", message);
        sleep(tokio::time::Duration::from_secs(1)).await;
        println!("[Consumer] Finished: {}", message);
    }
    println!("All background tasks processed successfully!");
    println!(
        "Total execution time: {:?}",
        start_time.elapsed().as_millis()
    );
}

async fn fetch_user_profile() -> String {
    sleep(tokio::time::Duration::from_secs(1)).await;
    String::from("Profile Data")
}

async fn fetch_user_posts() -> Vec<String> {
    sleep(tokio::time::Duration::from_secs(1)).await;
    vec![String::from("Post 1"), String::from("Post 2")]
}

async fn download_file() {
    println!("Starting file download...");
    sleep(tokio::time::Duration::from_secs(2)).await;
    println!("File download finished!");
}
