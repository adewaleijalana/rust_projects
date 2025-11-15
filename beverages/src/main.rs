use beverages::drinks::{Coffee, Drinkable, Milk, Soda};
fn main() {
    let mut latte_coffe = Coffee::<&str>::new("latte", Milk::Whole, 32);
    println!("{:?}", latte_coffe);
    latte_coffe.consume();
    println!("After consuming my coffee: {:?}", latte_coffe);

    let cappuccino = Coffee::<&str>::new("cappuccino", Milk::Almond, 25);
    println!("{:?}", cappuccino);

    let pepsi = Soda::new(10, 25.6, String::from("Cherry"), 24);
    println!("{}", pepsi);

    let mut coke = pepsi.clone();

    println!("Are the Sodas equall? {}", pepsi == coke);

    coke.consume();

    println!("{:?}", coke);
}
