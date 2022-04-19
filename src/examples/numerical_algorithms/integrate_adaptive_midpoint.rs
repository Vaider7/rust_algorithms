pub fn integrate_adaptive_midpoint<T>(
    function: T,
    x_min: f32,
    x_max: f32,
    num_intervals: u32,
    max_slice_error: f32,
) -> f32
where
    T: Fn(f32) -> f32 + Copy,
{
    let dx = (x_max - x_min) / num_intervals as f32;

    let mut total_area = 0.0;
    let mut x = x_min;

    for _i in 0..num_intervals {
        total_area += slice_area(function, x, x + dx, max_slice_error);
        x += dx
    }

    total_area
}

pub fn slice_area<T>(function: T, x1: f32, x2: f32, max_slice_error: f32) -> f32
where
    T: Fn(f32) -> f32 + Copy,
{
    let y1 = function(x1);
    let y2 = function(x2);
    let xm = (x1 + x2) / 2.0;
    let ym = function(xm);

    let area_12 = (x2 - x1) * (y1 + y2) / 2.0;
    let area_1m = (xm - x1) * (y1 + ym) / 2.0;
    let area_m2 = (x2 - xm) * (ym + y2) / 2.0;
    let area_1m2 = area_1m + area_m2;

    let err = ((area_1m2 - area_12) / area_12).abs();

    if err < max_slice_error {
        area_1m2
    } else {
        slice_area(function, x1, xm, max_slice_error)
            + slice_area(function, xm, x2, max_slice_error)
    }
}

#[cfg(test)]
mod tests {
    use super::integrate_adaptive_midpoint;

    #[test]
    fn it_should_calculate_area_by_integrate_adaptive_midpoint() {
        assert_eq!(
            18.413158,
            integrate_adaptive_midpoint(|x| { 1.0 + x + (2.0 * x).sin() }, 0.0, 5.0, 2, 0.01)
        )
    }
}
