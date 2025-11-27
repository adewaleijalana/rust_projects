use std::collections::HashMap;

pub fn manual_iteration() {
    let mut  numbers = vec![4, 8, 15, 16, 23, 42];

    // let mut current_idx = 0;
    // let final_idx = numbers.len() - 1;

    // loop {
    //     if current_idx > final_idx {
    //         break;
    //     }
    //     println!("value: {}", numbers[current_idx]);
    //     current_idx +=1;
    // }

    for num in &numbers {
        println!("{}", *num * 2);
    }

    println!("{numbers:?}");

    for val in numbers.iter_mut() {
        *val *= *val;
    }

    println!("{numbers:?}")

}


pub fn hash_map_iteration() {
    let mut todos = HashMap::new();
    todos.insert("Study Rust", true);
    todos.insert("Sleep", false);

    for (k, v) in &mut todos {
        println!("key: {} | status: {}", k, v);
        *v = true;
    }

    println!("{todos:?}")
}