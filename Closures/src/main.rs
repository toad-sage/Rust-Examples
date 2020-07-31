use std::collections::HashMap;
use std::thread;
use std::time::Duration;

fn main() {
    // let simulated_user_specified_value = 10;
    // let simulated_random_number = 7;

    // generate_workout(simulated_user_specified_value, simulated_random_number);
    let x = 4;
    let equal_to_x = |z| z == x;
    let y = 4;
    println!("{}", equal_to_x(y))
}

// T is a closure and every closure, implement at least one of
// the traits: Fn , FnMut , or FnOnce
struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    calculation: T,
    hash_map: Option<HashMap<u32, u32>>,
}

impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            hash_map: None,
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match &mut self.hash_map {
            Some(map) => match map.get(&arg) {
                Some(val) => *val,
                None => {
                    let v = (self.calculation)(arg);
                    map.insert(arg, v);
                    v
                }
            },
            None => {
                let v = (self.calculation)(arg);
                let mut map = HashMap::new();
                map.insert(arg, v);
                self.hash_map = Some(map);
                v
            }
        }
    }
}

fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_result = Cacher::new(|num| {
        println!("calculating slowly..");
        thread::sleep(Duration::from_secs(2));
        num
    });
    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_result.value(10));
        println!("Next, do {} situps", expensive_result.value(20));
        println!("Next, do {} situps", expensive_result.value(30));
        println!("Next, do {} situps", expensive_result.value(10));
        println!("Next, do {} situps", expensive_result.value(40));
        println!("Next, do {} situps", expensive_result.value(20));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_result.value(intensity)
            );
        }
    }
}

fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
}

fn generate_workout1(intensity: u32, random_number: u32) {
    if intensity < 25 {
        println!(
            "Today, do {} pushups !",
            simulated_expensive_calculation(intensity)
        );
        println!(
            "Next, do {} situps!",
            simulated_expensive_calculation(intensity)
        );
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated");
        } else {
            println!(
                "Today, run for {} minutes!",
                simulated_expensive_calculation(intensity)
            );
        }
    }
}

fn generate_workout2(intensity: u32, random_number: u32) {
    let expensive_result = simulated_expensive_calculation(intensity);
    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_result);
        println!("Next, do {} situps!", expensive_result);
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!("Today, run for {} minutes!", expensive_result);
        }
    }
}
