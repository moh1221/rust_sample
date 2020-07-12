use std::fmt;

pub struct Order {
    name: String,
    level: i32,
    des: String,
    val: f32
}

impl Order {
    pub fn new(name: String, level: i32, des: String, val: f32) -> Order {
        Order {
            name,
            level,
            des,
            val
        }
    }
}

impl fmt::Display for Order {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> { 
        write!(f, "Order: {} '{}' @ {} - level {}", self.name, self.des, self.val, self.level)
    }
}
