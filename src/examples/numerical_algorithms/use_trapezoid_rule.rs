pub fn use_trapezoid_rule<T>(function: T, x_min: f32, x_max: f32, num_intervals: u32) -> f32
where
    T: Fn(f32) -> f32,
{
    let dx = (x_max - x_min) / num_intervals as f32;

    let mut total_area = 0.0;
    let mut x = x_min;

    for _i in 0..num_intervals {
        total_area += dx * (function(x) + function(x + dx)) / 2.0;
        x += dx
    }

    total_area
}

#[cfg(test)]
mod tests {
    use super::use_trapezoid_rule;

    #[test]
    fn it_should_calculate_area_by_trapezoid_rule() {
        assert_eq!(
            18.3416,
            use_trapezoid_rule(|x| { 1.0 + x + (2.0 * x).sin() }, 0.0, 5.0, 10)
        )
    }
}
