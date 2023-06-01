// strings3.rs
// Execute `rustlings hint strings3` or use the `hint` watch subcommand for a hint.

fn trim_me(input: &str) -> String {
    // TODO: Remove whitespace from both ends of a string!
    let newString = input.trim().to_string();
    newString
}

fn compose_me(input: &str) -> String {
    // TODO: Add " world!" to the string! There's multiple ways to do this!

// ALIYAH: 

let length: usize = input.len();

let mut input = input.to_string();

input.insert_str(length, " world!"); 
// implicitly return "input" by ommiting the ;
input

// THIRD ATTEMPT 
  // let mut newString = input.to_string().push_str("world!");
  // newString

// SECOND ATTEMPT
    // format!(input, "world!")
// Apparently the correct solution is, but this also doesn't confirm it works....  
// format!("{} world!", input)

// FIRST ATTEMPT - doesn't give errors but doesn't confirm it works.... 
    // let mut input2 = String::from(input);
    // input2.push_str(" world!");
    // input2
}

fn replace_me(input: &str) -> String {
    // TODO: Replace "cars" in the string with "balloons"!
    let newString = input.replace("cars", "balloons").to_string();
    newString
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
        assert_eq!(replace_me("I think cars are cool"), "I think balloons are cool");
        assert_eq!(replace_me("I love to look at cars"), "I love to look at balloons");
    }
}
