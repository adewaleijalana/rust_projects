use crate::ticket_seller::TicketSeller;

#[derive(Debug)]
pub struct MovieTheater {
    movies: Vec<String>,
    sales: u32,
}

impl MovieTheater {
    pub fn new() -> Self {
        Self {
            movies: vec![],
            sales: 0,
        }
    }

    pub fn add_painting(&mut self, movie: &str) {
        self.movies.push(movie.to_string());
    }
}

impl TicketSeller for MovieTheater {
    fn sell_ticket(&mut self) {
        self.sales += 15;
    }
}
