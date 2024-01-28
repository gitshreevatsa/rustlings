fn main() {
    let s1 = String::from("hello");


    let (s2, len) = calculate_length(&s1);

    println!("The length of '{}' is {}. s1 : {}", s2, len, s1);
}

fn calculate_length(s: &String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s.to_string(), length)
}