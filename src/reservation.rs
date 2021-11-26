pub struct Reservation {
    customer: u32,
    res_type: u32,
    res_id: String,
    id: u32,
}

impl Reservation {
    pub fn new(customer: u32, res_type: u32, res_id: String, id: u32) -> Self {
        Reservation {
            customer,
            res_type,
            res_id,
            id,
        }
    }

    pub fn set_customer(&mut self, customer: u32) {
        self.customer = customer;
    }
    pub fn set_res_type(&mut self, res_type: u32) {
        self.res_type = res_type;
    }
    pub fn set_res_id(&mut self, res_id: String) {
        self.res_id = res_id;
    }
    pub fn set_id(&mut self, id: u32) {
        self.id = id;
    }
    pub fn customer(&self) -> u32 {
        self.customer
    }
    pub fn res_type(&self) -> u32 {
        self.res_type
    }
    pub fn res_id(&self) -> &str {
        &self.res_id
    }
    pub fn id(&self) -> u32 {
        self.id
    }
}
