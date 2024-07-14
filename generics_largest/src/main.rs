use summary::Summary;

mod summary;
fn main() {
     let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {result}");

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {result}");

    let tweet = Tweet {
        username: String::from("Test")
    };

    notify(&tweet);
}


fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn notify(item: &impl Summary){
    println!("{}", item.summarize_author())
}

struct Tweet {
    username: String
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}