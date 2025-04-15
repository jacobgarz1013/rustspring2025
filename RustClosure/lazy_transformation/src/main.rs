use std::{thread, time::Duration};

struct ComputeCache<T>
where
    T: Fn() -> String,
{
    computation: T,
    cached_result: Option<String>,
}

impl<T> ComputeCache<T>
where
    T: Fn() -> String,
{
    fn new(computation: T) -> Self {
        Self {
            computation,
            cached_result: None,
        }
    }

    fn get_result(&mut self) -> String {
        match &self.cached_result {
            Some(result) => {
                println!("Retrieved from cache instantly!");
                result.clone()
            }
            None => {
                let result = (self.computation)();
                self.cached_result = Some(result.clone());
                result
            }
        }
    }
}

fn main() {
    let mut cache = ComputeCache::new(|| {
        println!("Computing (this will take 2 seconds)...");
        thread::sleep(Duration::from_secs(2));
        "Hello, world!".to_string()
    });

    println!("First call:");
    println!("Result: {}", cache.get_result());
    
    println!("\nSecond call:");
    println!("Result (cached): {}", cache.get_result());
}
