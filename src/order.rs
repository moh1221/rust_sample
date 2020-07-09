use std::fmt;

pub struct Order {
    name: String,
    des: String,
    val: f32
}

impl Order {
    pub fn new(name: String, des: String, val: f32) -> Order {
        Order {
            name,
            des,
            val
        }
    }
}

impl fmt::Display for Order {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> { 
        write!(f, "Order: {} '{}' @ {}", self.name, self.des, self.val)
    }
}
