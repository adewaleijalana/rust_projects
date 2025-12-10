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

#[cfg(test)]
mod museum_tests {
    use super::*;
    use pretty_assertions::{assert_eq, assert_ne};

    #[test]
    fn museum_sells_ticket_to_increase_revenue() {
        let mut museum = Museum::new();
        museum.sell_ticket();
        assert_eq!(museum.revenue, 25);
    }

    #[test]
    fn museum_can_sell_multiple_tickets() {
        let mut museum = Museum::new();
        museum.sell_ticket();
        museum.sell_ticket();
        assert_eq!(museum.revenue, 55, "The revenue failed");
    }

    #[test]
    fn museum_has_impressive_colletion() {
      let mut museum = Museum::new();
       museum.buy_painting("Monalisa");
       museum.buy_painting("Monalisa");
       museum.buy_painting("Monalisa");

       assert!(museum.has_impressive_colletion());
    }
}
