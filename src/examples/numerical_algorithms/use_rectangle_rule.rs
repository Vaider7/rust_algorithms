pub fn use_rectangle_rule<T>(function: T, x_min: f32, x_max: f32, num_intervals: u32) -> f32
where
    T: Fn(f32) -> f32,
{
    let dx = (x_max - x_min) / num_intervals as f32;

    let mut total_area = 0.0;

    let mut x = x_min;

    for _i in 0..num_intervals {
        total_area += dx * function(x);
        x += dx;
    }

    total_area
}

#[cfg(test)]
mod tests {
    use super::use_rectangle_rule;

    #[test]
    fn it_should_calculate_area_by_rectangle_rule() {
        assert_eq!(
            17.227604,
            use_rectangle_rule(|x| { 1. + x + (2. * x).sin() }, 0., 5., 10)
        )
    }
}
