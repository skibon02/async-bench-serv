// simple tcp repeater server

use async_std::prelude::*;
use async_std::task;
use async_std::net::{TcpListener, TcpStream};
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Arc;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

pub fn run() {
    task::block_on(async {
        match run_worker().await {
            Ok(_) => println!("Server closed"),
            Err(e) => println!("Error: {}", e),
        }
    });


}

async fn run_worker() -> Result<()> {

    let listener = TcpListener::bind("127.0.0.1:8080").await?;

    let counter = Arc::new(AtomicU32::new(0));
    let counter_reader = counter.clone();
    task::spawn( async move {    // spawn a new task to print the counter
        loop {
            task::sleep(std::time::Duration::from_secs(1)).await;
            let cur_val = counter_reader.load(std::sync::atomic::Ordering::Relaxed);
            println!("packets/sec: {}", cur_val);
            println!("KB/sec: {}", cur_val as f32 * 4f32 / 1024f32 );
            counter_reader.store(0, std::sync::atomic::Ordering::Relaxed);
        }
    });
    
    // accept connections and process them serially
    let mut incoming = listener.incoming();
    while let Some(stream) = incoming.next().await {
        let counter = counter.clone();
        let stream = stream?;
        
        task::spawn(async move {
            handle_client(stream, counter).await;
        });
    }
    println!("Server closed");

    // close the socket server
    Ok(())
}

async fn handle_client(mut stream: TcpStream, counter: Arc<AtomicU32>) {
    let mut data = [0 as u8; 4]; // using 4 byte buffer
    println!("New connection: {}", stream.peer_addr().unwrap());
    loop {
        match stream.read(&mut data).await {
            Ok(_) => {
                counter.fetch_add(1, Ordering::Relaxed);
                stream.write_all(&data).await.unwrap();
            }
            Err(_) => {
                println!("Connection with {} terminated.", stream.peer_addr().unwrap());
                break;
            }
        }
    }
}