use pretty_assertions::assert_eq;
use rose_museum::{MovieTheater, VenueManagement};

#[test]
fn venue_management_interacts_with_movie_theater() {
    let mut movie_theater = MovieTheater::new();
    movie_theater.add_painting("Mona Lina");
    let mut venue_management = VenueManagement::new(movie_theater);
    venue_management.make_money();

    assert_eq!(venue_management.get_venue().get_sales(), 15);
}
