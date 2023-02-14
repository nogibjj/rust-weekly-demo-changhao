/*build a function marco polo which take a string as input, if input is Marco, return Polo, otherwise, return Who?*/

pub fn marco_polo(input: &str) -> String {
    if input == "Marco" {
        "Polo".to_string()
    } else {
        "Who?".to_string()
    }
}
