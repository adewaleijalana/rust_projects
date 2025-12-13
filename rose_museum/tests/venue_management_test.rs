use pretty_assertions::assert_eq;
use rose_museum::{MovieTheater, VenueManagement};
use rstest::{fixture, rstest};

#[fixture]
fn movie_theater_with_three_movies() -> MovieTheater {
    let mut movie_theater = MovieTheater::new();
    movie_theater.add_movie("Bat Man");
    movie_theater.add_movie("Spider Man");
    movie_theater.add_movie("Iron Man");
    movie_theater
}

#[rstest]
fn venue_management_interacts_with_movie_theater(movie_theater_with_three_movies: MovieTheater) {
    let mut venue_management = VenueManagement::new(movie_theater_with_three_movies);
    venue_management.make_money();

    assert_eq!(venue_management.get_venue().get_sales(), 15);
}

#[rstest]
fn venue_management_interacts_with_movie_theater_with_movies(
    movie_theater_with_three_movies: MovieTheater,
) {
    let mut venue_management = VenueManagement::new(movie_theater_with_three_movies);
    venue_management.make_money();

    assert!(venue_management.get_venue().has_movies_showing());
}
