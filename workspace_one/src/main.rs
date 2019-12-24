fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_workspace() {
        assert_eq!(true, false)
    }
}
