use crate::ToRow;
use prettytable::{Cell, Row};

pub struct Hotels {
    hotel_num: String,
    location: String,
    price: u32,
}

impl Hotels {
    pub fn new(hotel_num: String, location: String, price: u32) -> Self {
        Hotels {
            hotel_num,
            location,
            price,
        }
    }

    pub fn hotel_num(&self) -> &str {
        &self.hotel_num
    }
    pub fn location(&self) -> &str {
        &self.location
    }
    pub fn price(&self) -> u32 {
        self.price
    }

    pub fn set_hotel_num(&mut self, hotel_num: String) {
        self.hotel_num = hotel_num;
    }
    pub fn set_location(&mut self, location: String) {
        self.location = location;
    }
    pub fn set_price(&mut self, price: u32) {
        self.price = price;
    }
}

impl ToRow for Hotels {
    fn to_row(&self) -> Row {
        Row::new(vec![
            Cell::new(self.hotel_num()),
            Cell::new(self.location()),
            Cell::new(&self.price.to_string()),
        ])
    }
}
