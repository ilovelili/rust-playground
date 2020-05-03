use std::thread;
use std::time::Duration;

fn main() {
    let mut expensive_result = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });
}

struct Cacher<T>
where
    T: Fn(u8) -> u8, // where T is a closure
{
    calculation: T,
    value: Option<u8>,
}

impl<T> Cacher<T>
where
    T: Fn(u8) -> u8,
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation: calculation,
            value: None,
        }
    }

    fn value(&mut self, arg: u8) -> u8 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}
