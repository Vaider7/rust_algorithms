use rand::{thread_rng, Rng};

pub fn is_prime(p: u32, max_tests: u32) -> bool {
    let mut rand = thread_rng();

    for _i in 0..max_tests {
        let n: u128 = rand.gen_range(1..p as u128);

        if n.pow(p - 1) % p as u128 != 1 {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::is_prime;

    #[test]
    fn it_should_check_is_num_prime() {
        assert!(is_prime(7, 1000))
    }
}
