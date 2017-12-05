const INPUT: isize = 368078;

fn main() {
    let (sq_size, sq_area) = (0..)
        .map(|n| n * 2 + 1)
        .map(|d| (d, d * d))
        .skip_while(|&(_, a)| a < INPUT)
        .nth(0)
        .unwrap();

    let lower_corner = (1..)
        .map(|n| sq_area - sq_size * n)
        .filter(|lo| *lo < INPUT)
        .nth(0)
        .unwrap();

    let edge_midpoint = lower_corner + (sq_size + 1) / 2;

    let midpoint_dist = (INPUT - edge_midpoint).abs();

    let total_dist = midpoint_dist + sq_size / 2;

    println!("{}", total_dist);
}
