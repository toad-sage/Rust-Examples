struct AveragedCollection {
    list: Vec<i32>,
    average: Option<f64>,
}

impl AveragedCollection {
    pub fn new() -> AveragedCollection {
        AveragedCollection {
            list: vec![],
            average: None,
        }
    }

    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_average();
    }

    pub fn remove(&mut self) -> Option<i32> {
        let res = self.list.pop();
        match res {
            Some(val) => {
                self.update_average();
                Some(val)
            }
            None => None,
        }
    }

    pub fn average(&self) -> Option<f64> {
        self.average
    }

    fn update_average(&mut self) {
        let total: i32 = self.list.iter().sum();
        let avg = total as f64 / self.list.len() as f64;
        self.average = Some(avg);
    }
}

fn main() {
    let mut avg = AveragedCollection::new();
    avg.add(4);
    avg.add(3);
    match avg.average() {
        Some(val) => println!("{}", val),
        None => println!("None"),
    }
    avg.add(5);
    match avg.average() {
        Some(val) => println!("{}", val),
        None => println!("None"),
    }
}
