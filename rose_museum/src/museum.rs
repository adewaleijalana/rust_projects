use crate::ticket_seller::TicketSeller;

pub struct Museum {
    paintings: Vec<String>,
    revenue: u32,
}

impl Museum {

    /// Creates a new Museum instance.
    /// 
    /// # Examples
    /// ```
    /// use rose_museum::museum::Museum;
    /// 
    /// let museum = Museum::new();
    /// let empty_vec:Vec<String> = Vec::new();
    /// assert_eq!(*museum.get_paintings(), empty_vec);
    /// ```
    pub fn new() -> Self {
        Self {
            paintings: vec![],
            revenue: 0,
        }
    }

    pub fn buy_painting(&mut self, painting: &str) {
        self.paintings.push(painting.to_string());
    }

    pub fn get_paintings(&self) -> &Vec<String>{
        &self.paintings
    }

    pub fn has_impressive_colletion(&self) -> bool {
        self.paintings.len() > 2
    }
}

impl TicketSeller for Museum {
    fn sell_ticket(&mut self) {
        self.revenue += 25;
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
        assert_eq!(museum.revenue, 50, "The revenue failed");
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
