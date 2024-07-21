
// fn longest<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
// where
//     T: Display,
// {
//     print!("Announcement! {ann}");
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }

pub fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}