fn reverse_string(string: &str) -> String {
    string.chars().rev().collect::<String>()
}

fn reverse_sentence(string: &str) -> String {
    string.rsplit(" ").collect::<Vec<&str>>().join(" ")
}

fn is_palindrome(string: &str) -> bool {
    if string.is_empty() {
        return true;
    }

    let mut from_start = string.chars().enumerate();
    let mut from_end = string.chars().rev().enumerate();
    let length = string.len();

    loop {
        let (left_index, left_value) = from_start.next().unwrap();
        let (right_index, right_value) = from_end.next().unwrap();

        // return false if pointed char is different
        if left_value != right_value {
            return false;
        }

        // if both iterators are at the same indice, we will return true
        if left_index == length - right_index - 1 {
            return true;
        }
    }
}

fn fill_with_zero(rows: &mut Vec<&mut Vec<u32>>) {}

fn main() {
    println!("{}", reverse_string("Hello, world!"));
    println!("{}", reverse_sentence("Hello kids how are you doing"));
    println!("tacocat is palindrome? {}", is_palindrome("tacocat"));
}

#[cfg(test)]
mod tests {
    #[test]
    fn reverse_string_works() {
        assert_eq!("!dlrow ,olleH", super::reverse_string("Hello, world!"));
    }

    #[test]
    fn reverse_sentence_works() {
        assert_eq!(
            "doing you are how kids Hello",
            super::reverse_sentence("Hello kids how are you doing")
        );
    }

    #[test]
    fn is_palindrome_returns_true_when_palindrome() {
        assert_eq!(true, super::is_palindrome("tacocat"))
    }

    #[test]
    fn is_palindrome_returns_true_when_empty_string() {
        assert_eq!(true, super::is_palindrome(""))
    }

    #[test]
    fn is_palindrome_returns_true_when_string_is_one_char_long() {
        assert_eq!(true, super::is_palindrome("L"))
    }

    #[test]
    fn is_palindrome_returns_false_when_palindrome() {
        assert_eq!(false, super::is_palindrome("hey"))
    }
}
