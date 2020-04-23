
pub fn sumu32(a: u32,b:u32) -> u32{
    return a+b+1
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]

    fn it_works() {
        assert_eq!(sumu32(2,2), 4);
    }
}
