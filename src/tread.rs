

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