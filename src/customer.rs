pub struct Customer {
    name: String,
    id: u32,
}
impl Customer {
    pub fn new(name: String, id: u32) -> Self {
        Customer { name, id }
    }

    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }
    pub fn set_id(&mut self, id: u32) {
        self.id = id;
    }
}
