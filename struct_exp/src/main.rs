mod user;
use user::User;

fn main() {
    // let user = User::default_user();
    let user2 = User::new_user(String::from("Rose"), String::from("Adewale"), 30);
    let user3 = User{
        first_name: String::from("Jazmine"),
        ..user2
    };

    // println!("{:?}", user2);
    println!("first name is {}", user3.get_first_name());
}
