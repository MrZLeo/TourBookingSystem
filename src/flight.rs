use crate::ToRow;
use mysql::{params, PooledConn};
use prettytable::{Cell, Row};

#[derive(Clone)]
pub struct Flight {
    flight_num: String,
    price: u32,
    from_city: String,
    arrive_city: String,
}

impl Flight {
    pub fn new(flight_num: String, price: u32, from_city: String, arrive_city: String) -> Self {
        Flight {
            flight_num,
            price,
            from_city,
            arrive_city,
        }
    }

    pub fn flight_num(&self) -> &str {
        &self.flight_num
    }
    pub fn price(&self) -> u32 {
        self.price
    }
    pub fn from_city(&self) -> &str {
        &self.from_city
    }
    pub fn arrive_city(&self) -> &str {
        &self.arrive_city
    }

    pub fn set_flight_num(&mut self, flight_num: String) {
        self.flight_num = flight_num;
    }
    pub fn set_price(&mut self, price: u32) {
        self.price = price;
    }
    pub fn set_from_city(&mut self, from_city: String) {
        self.from_city = from_city;
    }
    pub fn set_arrive_city(&mut self, arrive_city: String) {
        self.arrive_city = arrive_city;
    }
}

impl ToRow for Flight {
    fn to_row(&self) -> Row {
        Row::new(vec![
            Cell::new(self.flight_num()),
            Cell::new(&self.price().to_string()),
            Cell::new(self.from_city()),
            Cell::new(self.arrive_city()),
        ])
    }
}

impl PartialEq<Self> for Flight {
    fn eq(&self, other: &Self) -> bool {
        self.flight_num == other.flight_num
            && self.arrive_city == other.arrive_city
            && self.from_city == other.from_city
            && self.price == other.price
    }
}

impl Eq for Flight {}
