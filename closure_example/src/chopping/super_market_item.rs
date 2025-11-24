#[derive(Debug, Clone)]
pub struct SupermarketItem {
    name: String,
    price: f64,
}

impl SupermarketItem {
    pub fn new(name: String, price: f64) -> Self {
        Self { name, price }
    }

    pub fn price(&self) -> f64 {
        self.price
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn set_price(&mut self, new_price: f64){
        self.price = new_price;
    }

    pub fn set_name(&mut self, new_name: String){
        self.name = new_name;
    }
}
