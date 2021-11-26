use crate::ToRow;
use prettytable::{Cell, Row};

pub struct Bus {
    bus_num: String,
    location: String,
    price: u32,
}

impl Bus {
    pub fn new(bus_num: String, location: String, price: u32) -> Self {
        Bus {
            bus_num,
            location,
            price,
        }
    }

    pub fn bus_num(&self) -> &str {
        &self.bus_num
    }
    pub fn location(&self) -> &str {
        &self.location
    }
    pub fn price(&self) -> u32 {
        self.price
    }

    pub fn set_bus_num(&mut self, bus_num: String) {
        self.bus_num = bus_num;
    }
    pub fn set_location(&mut self, location: String) {
        self.location = location;
    }
    pub fn set_price(&mut self, price: u32) {
        self.price = price;
    }
}

impl ToRow for Bus {
    fn to_row(&self) -> Row {
        Row::new(vec![
            Cell::new(self.bus_num()),
            Cell::new(self.location()),
            Cell::new(&self.price.to_string()),
        ])
    }
}
