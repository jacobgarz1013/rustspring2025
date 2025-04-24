use std::thread;
use std::time::Duration;

fn main() {
    println!("Main thread starting");

    // TODO: Create a vector to store thread handles
    let mut handles = vec![];

    // TODO: Spawn 3 threads
    for i in 1..=3 {
        // TODO: Spawn a thread and store its handle
        let handle = thread::spawn(move || {
            println!("Thread {} starting", i);
            thread::sleep(Duration::from_millis(500));
            println!("Thread {} finished", i);
        });

        // TODO: Store the handle
        handles.push(handle);
    }

    // TODO: Wait for all threads to completee
    for handle in handles {
        handle.join().unwrap(); 
    }

    println!("All threads completed.");
}
