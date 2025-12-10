pub struct Museum {
    paintings: Vec<String>,
    revenue: u32,
}

impl Museum {
    pub fn new() -> Self {
        Self {
            paintings: vec![],
            revenue: 0,
        }
    }

    pub fn buy_painting(&mut self, painting: &str) {
        self.paintings.push(painting.to_string());
    }

    pub fn sell_ticket(&mut self) {
        self.revenue += 25;
    }

    pub fn has_impressive_colletion(&self) -> bool {
        self.paintings.len() > 2
    }
}
