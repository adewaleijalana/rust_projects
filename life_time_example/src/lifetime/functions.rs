use std::fmt::Display;

fn longest_2<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    print!("Announcement! {ann}");
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

pub fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

pub fn awe_funtion(value: &i32, second: &str) {}

pub fn double_the_length<T>(items: &Vec<T>) -> usize {
    2 * items.len()
}

pub fn last_two<T>(items: &[T]) -> &[T] {
    &items[items.len() - 2..]
}

pub fn first_five<'a>(text: &'a str, announcement: &str) -> &'a str {
    println!("An announcement: {announcement}");
    &text[..5]
}

pub fn find_string_that_has_content<'a, 'b>(
    first: &'a str,
    second: &'a str,
    target: &'b str,
) -> &'a str {
    if first.contains(target) {
        first
    } else {
        second
    }
}
