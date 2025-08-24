fn greeting() -> String {
    "Hello, world!".to_string()
}

fn main() {
    println!("{}", greeting());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_returns_hello_world() {
        assert_eq!(greeting(), "Hello, world!");
    }
}
