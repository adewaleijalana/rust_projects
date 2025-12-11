use crate::{movie_theater::MovieTheater, ticket_seller::TicketSeller};

#[derive(Debug)]
pub struct VenueManagement<T: TicketSeller> {
    venue: T,
    manager: Option<String>,
}

impl VenueManagement<MovieTheater> {
    pub fn new() -> Self {
        Self {
            venue: MovieTheater::new(),
            manager: None,
        }
    }

    pub fn hire_manager(&mut self, manager: &str) {
        self.manager = Some(manager.to_string());
    }

    pub fn make_money(&mut self){
      self.venue.sell_ticket();
    }
}

#[cfg(test)]
mod tests{
  use super::*;
  use pretty_assertions::assert_eq;

  #[test]
  fn venue_management_can_hire_manager() {
      let mut venue_management = VenueManagement::new();
      venue_management.hire_manager("Rose Manager");

      assert_eq!(venue_management.manager, Some("Rose Manager".to_string()))
  }

}
