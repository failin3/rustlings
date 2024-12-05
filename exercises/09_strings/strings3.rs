fn trim_me(input: &str) -> &str {
    // TODO: Remove whitespace from both ends of a string.
    let mut start = 0;
    let mut end = input.len();
    for c in input.chars() {
        if c != ' ' {
            break;
        }
        start += 1;
    }

    for c in input.chars().rev() {
        if c != ' ' {
            break
        }
        end -= 1
    }
    &input[start..end]
}

fn compose_me(input: &str) -> String {
    // TODO: Add " world!" to the string! There are multiple ways to do this.
    let mut final_string = String::from("");
    final_string.push_str(input);
    final_string.push_str(" world!");
    final_string
}

fn replace_me(input: &str) -> String {
    // TODO: Replace "cars" in the string with "balloons".
    // let output = input.split_once("cars");
    input.replace("cars", "balloons")
    // return "test".to_string();
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trim_a_string() {
        assert_eq!(trim_me("Hello!     "), "Hello!");
        assert_eq!(trim_me("  What's up!"), "What's up!");
        assert_eq!(trim_me("   Hola!  "), "Hola!");
    }

    #[test]
    fn compose_a_string() {
        assert_eq!(compose_me("Hello"), "Hello world!");
        assert_eq!(compose_me("Goodbye"), "Goodbye world!");
    }

    #[test]
    fn replace_a_string() {
        assert_eq!(
            replace_me("I think cars are cool"),
            "I think balloons are cool",
        );
        assert_eq!(
            replace_me("I love to look at cars"),
            "I love to look at balloons",
        );
    }
}
