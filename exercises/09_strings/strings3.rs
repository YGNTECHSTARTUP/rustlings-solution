fn trim_me(input: &str) -> &str {
    input.trim()
    // TODO: Remove whitespace from both ends of a string.
}

fn compose_me(input: &str) -> String {
    let newstr = input.to_owned() + " world!";
    newstr
    // TODO: Add " world!" to the string! There are multiple ways to do this.
}

fn replace_me(input: &str) -> String {
    let chars: String = input.replace("cars", "balloons");
    chars // TODO: Replace "cars" in the string with "balloons".
}

fn main() {
    let letters = "i love cars".replace("cars", "balloons");
    println!("{}", letters);

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
