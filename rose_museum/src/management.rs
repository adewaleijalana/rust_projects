use crate::{movie_theater::MovieTheater, ticket_seller::TicketSeller};

#[derive(Debug)]
pub struct VenueManagement<T: TicketSeller> {
    venue: T,
    manager: Option<String>,
}

impl<T: TicketSeller> VenueManagement<T> {
    pub fn new(venue: T) -> Self {
        Self {
            venue,
            manager: None,
        }
    }

    pub fn hire_manager(&mut self, manager: &str) {
        self.manager = Some(manager.to_string());
    }

    pub fn make_money(&mut self) {
        self.venue.sell_ticket();
    }

    pub fn get_manage(&self) -> Option<String> {
        self.manager.clone()
    }

    pub fn get_venue(&self) -> &T{
        &self.venue
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn venue_management_can_hire_manager() {
        let movie_theater = MovieTheater::new();
        let mut venue_management = VenueManagement::new(movie_theater);
        venue_management.hire_manager("Rose Manager");

        assert_eq!(venue_management.manager, Some("Rose Manager".to_string()))
    }
}
