use std::cmp;
use std::collections::HashMap;
use std::hash;
use std::thread;
use std::time::Duration;

struct Prac<'a, T, U: 'a, V: 'a>
where
    T: Fn(&'a U) -> &V,
{
    calculation: T,
    hash_map: Option<HashMap<&'a U, &'a V>>,
}

impl<'a, T, U, V> Prac<'a, T, U, V>
where
    U: cmp::Eq + hash::Hash,
    T: Fn(&'a U) -> &V,
{
    fn new(calculation: T) -> Prac<'a, T, U, V> {
        Prac {
            calculation,
            hash_map: None,
        }
    }

    fn value(&mut self, arg: &'a U) -> &V {
        match &mut self.hash_map {
            Some(map) => match map.get(arg) {
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

fn main() {
    let mut solver = Prac::new(|x| {
        println!("calculating slowly..");
        thread::sleep(Duration::from_secs(2));
        x
    });

    let mut int_solver = Prac::new(|x| {
        println!("calculating slowly..");
        thread::sleep(Duration::from_secs(2));
        x
    });

    println!("{}", solver.value(&"Robert"));
    println!("{}", solver.value(&"Brownie"));
    println!("{}", int_solver.value(&4));
    println!("{}", solver.value(&"Robert"));
    println!("{}", int_solver.value(&1));
    println!("{}", int_solver.value(&4));
}
