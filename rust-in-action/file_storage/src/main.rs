use file_storage::city::City;

fn main() {
    let calabar = City::new(String::from("Calabar"), 470_000, 4.95, 8.33);

    let as_json = serde_json::to_string(&calabar).unwrap_or_default();
    println!("{as_json}");
}
