// In this exercise, you'll learn some of the unique advantages that iterators
// can offer.

// TODO: Complete the `capitalize_first` function.
// "hello" -> "Hello"
fn capitalize_first(input: &str) -> String {
    let mut chars = input.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
    }
}

// TODO: Apply the `capitalize_first` function to a slice of string slices.
// Return a vector of strings.
// ["hello", "world"] -> ["Hello", "World"]
fn capitalize_words_vector(words: &[&str]) -> Vec<String> {
    let mut capital_words = vec![];
    for word in words {
        capital_words.push(capitalize_first(word));
    }
    capital_words
}

// TODO: Apply the `capitalize_first` function again to a slice of string
// slices. Return a single string.
// ["hello", " ", "world"] -> "Hello World"
fn capitalize_words_string(words: &[&str]) -> String {
    let mut result = String::new();
    for word in words {
        result.insert_str(result.len(), &capitalize_first(word));
    }
    result
}

fn main() {
    // You can optionally experiment here.
    let my_string_vec = vec!["meow", "meow", "meow"];
    let my_string_iter = my_string_vec.iter();

    println!("{:?}", my_string_iter);

    let my_string = "meooow";
    let mut my_string_chars = my_string.chars();
    println!("{:?}", my_string_chars.as_str());

    let word_with_3_letter_capital: String = "".to_string();

    println!(
        "{:?}",
        my_string_chars
            .next()
            .unwrap()
            .to_uppercase()
            .collect::<String>()
    );
    println!("{:?}", my_string_chars.next());
    println!("{:?}", my_string_chars);
    // println!("{:?}", my_string_chars.next().unwrap().to_uppercase());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        assert_eq!(capitalize_first("hello"), "Hello");
    }

    #[test]
    fn test_empty() {
        assert_eq!(capitalize_first(""), "");
    }

    #[test]
    fn test_iterate_string_vec() {
        let words = vec!["hello", "world"];
        assert_eq!(capitalize_words_vector(&words), ["Hello", "World"]);
    }

    #[test]
    fn test_iterate_into_string() {
        let words = vec!["hello", " ", "world"];
        assert_eq!(capitalize_words_string(&words), "Hello World");
    }
}
