pub fn find_primes(max_number: u32) -> Vec<u32> {
    let mut is_composite: Vec<bool> = Vec::with_capacity(max_number as usize);

    is_composite.resize(max_number as usize, false);

    for i in (4..max_number).step_by(2) {
        is_composite[i as usize] = true
    }

    let mut next_prime = 3;

    let stop_at = (max_number as f32).sqrt() as u32;

    while next_prime <= stop_at {
        for i in (next_prime * next_prime..max_number).step_by(next_prime as usize) {
            is_composite[i as usize] = true
        }

        next_prime += 2;

        while next_prime <= max_number && is_composite[next_prime as usize] {
            next_prime += 2
        }
    }

    let mut primes: Vec<u32> = vec![];

    for i in 2..max_number {
        if !is_composite[i as usize] {
            primes.push(i)
        }
    }

    primes
}

#[cfg(test)]
mod tests {
    use super::find_primes;

    #[test]
    fn it_should_give_primes() {
        assert_eq!(vec![2, 3, 5, 7], find_primes(10));
        assert_eq!(vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29], find_primes(30));
    }
}
