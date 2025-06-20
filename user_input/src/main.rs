use std::io;

macro_rules! user_input {
    ($name:ident, $age:ident) => {
        println!("Enter your name");
        let mut $name = String::new();
        io::stdin()
            .read_line(&mut $name)
            .expect("Failed to read name");
        let $name = $name.trim();

        println!("Enter your age");
        let mut $age = String::new();
        io::stdin().read_line(&mut $age).expect("error reading age");

        println!("Your name is {} and age is {}", $name, $age);
    };
    ($t: ty) => {{
        println!("Enter a value of type {}", stringify!($t));
        let mut num = String::new();
        io::stdin().read_line(&mut num).expect("error reading age");

        let num: $t = num.trim().parse().expect("Please enter a valid number");
        // println!("You entered: {}", num);
        num
    }};
}

macro_rules! add_as {
    ($a: expr, $b: expr, $t: ty) => {
        $a as $t + $b as $t
    };
}

macro_rules! make_struct {

    ($name:ident {$($field:ident: $ty:ty),* }) => {{
        #[derive(Debug)]
        struct $name {
            $($field: $ty),*
        }

    };
};
}

fn main() {
    // println!("This program is to take user input!");

    // println!("Enter your name");

    // let mut name = String::new();

    // io::stdin()
    // .read_line(&mut name)
    // .expect("Failed to read name");

    // let name = name.trim();

    // println!("Enter your age ");
    // let mut age = String::new();

    // io::stdin()
    // .read_line(&mut age)
    // .expect("error reading age");

    // println!("Your name is {name} and age is {age}")
    // user_input!(name, age);
    // let n = user_input!(u8);
    // println!("You entered: {}", n);

    // let sum = add_as!(5, 10, i32);
    // println!("Sum is {sum}")

    // let john = Person {
    //     name: String::from("John"),
    //     age: 25,
    // };
    // let (new_name, new_age) = update_person(john);
    // println!("Updated person: {} ({} years old)", new_name, new_age);
    // // println!("{:?}", john)

    let name = Some(String::from("Alice"));
    let name_ref = name.as_ref();
 
    match name_ref {
        Some(ref_str) => {
            println!("Name: {}", ref_str);
        }
        None => {
            println!("No name available");
        }
    }
}

#[derive(Debug)]
struct Person {
    name: String,
    age: u32,
}
 
fn update_person(mut person: Person) -> (String, u32) {
    let new_name = person.name.clone();
    let new_age = person.age + 1;
    (new_name, new_age)
}