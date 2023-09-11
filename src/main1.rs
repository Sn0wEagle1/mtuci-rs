// use std::collections::HashMap;

// fn main() {

//     let mut map: HashMap<&String, i32> = HashMap::new();

//     let n1 = "Danil".to_string();
//     let n2 = "Kirill".to_string();
//     let n3 = "Yaroslav".to_string();

//     map.insert(&n1, 10);
//     map.insert(&n2, 20);
//     map.insert(&n3, 30);

//     for (name, mark) in &map {
//         println!("{} is {}", name, mark);
//     }
// }

// -----------------------

// use std::collections::HashMap;

// fn main() {

//     let s = "Learn Rust with me".to_lowercase();

//     let mut count_map: HashMap<&str, i32> = HashMap::new();

//     for w in s.split_whitespace() {
//         let count = count_map.entry(w).or_insert(0);
//         *count += 1;
//     }

//     println!("{:?}", count_map);
// }
