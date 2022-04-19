pub fn find_zero<T, J>(function: T, df_dx: J, initial_guess: f32, max_error: f32) -> f32
where
    T: Fn(f32) -> f32,
    J: Fn(f32) -> f32,
{
    let mut x = initial_guess;

    let mut i = 0;

    loop {
        let y = function(x);

        if y.abs() < max_error {
            break;
        }

        if i == 100 {
            break;
        }

        x = x - y / df_dx(x);
        i += 1
    }

    x
}

#[cfg(test)]
mod tests {
    use super::find_zero;

    #[test]
    fn it_should_find_zero() {
        assert_eq!(
            1.6180346,
            find_zero(
                |x| { x.powi(2) - x - 1.0 },
                |x| { 2.0 * x - 1.0 },
                10.0,
                0.001
            )
        )
    }
}
