use std::ops::Add;

#[derive(Debug)]
pub struct Millimeters(pub u32);
pub struct Meters(pub u32);

impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, others: Meters) -> Millimeters {
        Millimeters(self.0 + (others.0 * 1000))
    }
}
