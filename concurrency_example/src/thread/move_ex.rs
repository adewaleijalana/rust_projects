use std::thread;

pub fn move_xeample() {
    let v = vec![1, 2, 3, 4, 5];
    let v2 = v.clone();

    let handle = thread::spawn(move || {
        println!("Here's a vector: {v:?}");
    });

    let handle2 = thread::spawn(move || {
      let v3: Vec<i32> = v2.iter().map(|x| x * 2)
      .collect();
      println!("Here's doubling vector element: {v3:?}")
    });

    handle.join().unwrap();
    handle2.join().unwrap();
}
