

use std::sync::{Arc,Mutex};
use std::thread;
use std::time::Duration;

pub fn thread_example(){
   let handle = thread::spawn(|| {
        for i in 0..100 {
            println!("Hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(2));
        }
    });
    handle.join().unwrap();

    for i in 0..50 {
        println!("Hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    
}

pub fn thread_with_data_increment(){
    let mut v =vec![];
    let counter = Arc::new(Mutex::new(0));

    for i in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn( move || {

            let mut num = counter.lock().unwrap();
            *num +=1;


        });
        v.push(handle);
    }

    for handle in v {
        handle.join().unwrap();
    }

    println!("Counter: {}", *counter.lock().unwrap());

}