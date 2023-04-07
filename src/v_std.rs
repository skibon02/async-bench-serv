// simple tcp repeater server

use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::sync::Arc;
use std::sync::atomic::AtomicU32;
use std::thread;

pub fn run() {
    let listener = TcpListener::bind(crate::addr).unwrap();

    let counter = Arc::new(AtomicU32::new(0));
    let counter_reader = counter.clone();
    thread::spawn(move || {
        loop {
            thread::sleep(std::time::Duration::from_secs(1));
            let cur_val = counter_reader.load(std::sync::atomic::Ordering::Relaxed);
            println!("packets/sec: {}", cur_val);
            println!("KB/sec: {}", cur_val as f32 * 4f32 / 1024f32 );
            counter_reader.store(0, std::sync::atomic::Ordering::Relaxed);
        }
    });

    // accept connections and process them serially
    for stream in listener.incoming() {
        let counter = counter.clone();
        match stream {
            Ok(stream) => {
                thread::spawn(move || {
                    // connection succeeded
                    handle_client(stream, counter)
                });
            }
            Err(e) => { /* connection failed */ }
        }
    }

    // close the socket server
    drop(listener);
}

fn handle_client(mut stream: TcpStream, counter: Arc<AtomicU32>) {
    let mut data = [0 as u8; 4]; // using 50 byte buffer
    println!("New connection: {}", stream.peer_addr().unwrap());
    loop {
        match stream.read(&mut data) {
            Err(_) | Ok(0) => {
                println!("Connection closed");
                break;
            }
            Ok(size) => {
                // echo everything!
                stream.write(&data[0..size]).unwrap();
                counter.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            }
        }
    }
}