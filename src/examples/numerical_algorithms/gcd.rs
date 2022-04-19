pub fn gcd(mut a: u32, mut b: u32) -> u32 {
    while b != 0 {
        let remainder = a % b;

        a = b;
        b = remainder;
    }

    a
}

#[cfg(test)]
mod tests {
    use super::gcd;

    #[test]
    fn if_should_give_right_gcd() {
        assert_eq!(3, gcd(3, 6_u32.pow(2)));
    }
}
