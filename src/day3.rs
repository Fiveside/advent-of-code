use std::f64::consts::PI;

pub fn day3(coord: u64) {
    println!("received coord {}", coord);
    println!("part 1 solution: {}", part1(coord));
}

fn part1(coord: u64) -> f64 {
    // Treat the grid like a unit circle.  To find the position of the coordinate, find the
    // x and y, and add them together to get the travel distance
    // each ring has its maximal number follow this pattern:
    // 1^2, 3^2, 5^2, 7^2...

    // ring number is from 0
    let ring_number = (((coord as f64).sqrt() - 1f64) / 2f64).ceil();
    let ring_max = (ring_number * 2f64 + 1f64).powi(2);
    let ring_min = ((ring_number - 1f64) * 2f64 + 1f64).powi(2);
    let ring_count = ring_max - ring_min;

    // The spiral origin is at the bottom right, not the polar origin
    let polar_unit = 1f64 / ring_count * 2f64 * PI;
    let polar_origin = (ring_number - 1f64) * polar_unit * -1f64;
    let coordpos = coord as f64 - ring_min;
    let polarcoord = (coordpos * 2f64 * PI / ring_count) + polar_origin;

    // Nromalze
    let first_quad = (polarcoord % 90f64).abs();
    let polar = if first_quad > (PI / 4f64) {
        (PI / 2f64) - first_quad
    } else {
        first_quad
    };

    // x is also (base-1)/2 if you follow that pattern above
    let x = ring_number;
    let y = (polar.tan() * (1f64 / x)) / 1f64;

    println!("coordpos: {}, ring_min {}, ring_max {}", coordpos, ring_min, ring_max);
    println!("polar_unit {}, polar_origin {}, polar {}", polar_unit, polar_origin, polar);
    println!("polar rad: {}, deg: {}", polarcoord, polarcoord * 180f64/PI);
    println!("x {}, y {}", x, y);

    x.abs() +  y.abs().round()
}