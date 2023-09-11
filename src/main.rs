// fn main() {
//     let mut list: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7];

//     match list.get(6) {
//         Some(_) => {
//             println!("The 6rd element is {}", list[5]);
//         },
//         None => {
//             println!("Element is not found");
//         }
//     }
// }

// ---------------------------------------

// use std::io;

// fn main() {
//     println!("Введите все числа, среднее арифметическое значение которых вы хотите узнать. Когда введёте все числа, отправьте пустую строку.");

//     let mut list: Vec<i32> = Vec::new();

//     loop {
//         let mut input = String::new();

//         let _ = io::stdin().read_line(&mut input);

//         if input.trim().is_empty() && list.len() == 0 {
//             println!("Введите числа");
//             continue
//         }
//         if input.trim().is_empty() {
//             break
//         }

//         match input.trim().parse() {
//             Ok(num) => list.push(num),
//             Err(e) => println!("Ошибка: {}", e),
//         }
//     }

//     println!("Среднее арифметическое: {}", find_avg(&list))
// }
// fn find_avg(l: &Vec<i32>) -> f64 {
//     let mut sum: i32 = 0;

//     for el in l {
//         sum += el
//     }

//     let length: i32 = (l.len()) as i32;

//     (sum/length) as f64
// }

// ---------------------------------------

enum Types {
    Int(i32),
    Float(f32),
    Bool(bool),
    Text(String)
}

fn main() {
    let list: Vec<Types> = vec! [
        Types::Int(7),
        Types::Float(7.7),
        Types::Bool(true),
        Types::Text("Hello".to_string())
    ];

    match &list[1] {
        Types::Int(i_num) => {
            println!("Int is {}", i_num)
        },
        Types::Float(i_num) => {
            println!("Float is {}", i_num)
        },
        Types::Bool(logic) => {
            println!("Bool is {}", logic)
        },
        Types::Text(str) => {
            println!("Text is {}", str)
        }
    }
}