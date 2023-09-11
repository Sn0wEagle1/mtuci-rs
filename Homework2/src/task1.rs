/*
----> ЗАДАНИЕ 1 "Поиск слова в строке"

Вывести номер строки в котором встречается нужное слово и саму строку в формате:
номер строки: строка...

 */

const SEARCH_TERM: &str = "picture";
const QUOTE: &str = "Every face, every shop, bedroom window, public-house, and
dark square is a picture feverishly turned--in search of what?";

fn main() {
    let result = find_term(SEARCH_TERM, QUOTE);
    println!("{}", result);
}

fn find_term(search_term: &str, quote: &str) -> String {
    let mut result = String::new();
    let lines: Vec<&str> = quote.split('\n').collect();

    for (index, line) in lines.iter().enumerate() {
        if line.contains(search_term) {
            result.push_str(&format!("{}: {}", index + 1, line));
            break;
        }
    }

    result
}

// ----> TESTS
#[cfg(test)]
mod tests {
    use crate::find_term;
    use crate::{SEARCH_TERM, QUOTE};

    #[test]
    fn correct_line() {
        let ans = find_term(SEARCH_TERM, QUOTE);

        assert_eq!("2: dark square is a picture feverishly turned--in search of what?", ans);
    }
}
