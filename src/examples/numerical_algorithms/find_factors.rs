pub fn list_of_integers(mut number: u32) -> Vec<u32> {
    let mut factors = vec![];

    while number % 2 == 0 {
        factors.push(2);
        number /= 2
    }

    let mut i = 3;
    let mut max_factor = (number as f32).sqrt();

    while i <= max_factor as u32 {
        while number % i == 0 {
            factors.push(i);

            number /= i;

            max_factor = (number as f32).sqrt()
        }
        i += 2;
    }

    if number > 1 {
        factors.push(number)
    }

    factors
}

#[cfg(test)]
mod tests {
    use super::list_of_integers;
    use test::Bencher;

    #[test]
    fn it_should_give_list_of_integers() {
        assert_eq!(vec![7], list_of_integers(7));
        assert_eq!(vec![2, 2, 3, 5], list_of_integers(2 * 2 * 3 * 5));
        assert_eq!(vec![7, 19], list_of_integers(7 * 19));
    }

    #[bench]
    fn list_of_integers_bench(b: &mut Bencher) {
        b.iter(|| list_of_integers(1000000));
    }
}
