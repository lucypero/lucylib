pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn hello() -> String {
    "hellope!".into()
}

pub fn hello_2() -> String {
    "hellope from lucylib version 2!".into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
