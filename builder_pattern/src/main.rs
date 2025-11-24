use builder_pattern::UserBuilder;
fn main() {
    let user = UserBuilder::new()
        .username("john_doe")
        .email("john@example.com")
        .age(30)
        .build();

    println!("User created: {:?}", user);
}
