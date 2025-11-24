pub fn manual_iteration() {
    let numbers = vec![4, 8, 15, 16, 23, 42];

    let mut current_idx = 0;
    let final_idx = numbers.len() - 1;

    loop {
        if current_idx > final_idx {
            break;
        }
        println!("value: {}", numbers[current_idx]);
        current_idx +=1;
    }
}