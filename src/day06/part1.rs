use crate::utils::read_file_lines::read_file_lines;
#[derive(Debug)]
struct Data {
    time: i32,
    dist_record: i32,
}

pub fn run() {
    let lines = read_file_lines("./src/day06/inputData.txt");

    //  how do I get the first line?
    let times = lines[0]
        .as_str()
        .split_whitespace()
        .filter_map(|x| x.parse::<i32>().ok())
        .collect::<Vec<i32>>();

    let distances = lines[1]
        .as_str()
        .split_whitespace()
        .filter_map(|x| x.parse::<i32>().ok())
        .collect::<Vec<i32>>();

    println!("times: {:?}, distances: {:?}", times, distances);

    // combine the two vectors into a vector of tuples
    let time_distances: Vec<Data> = times
        .iter()
        .zip(distances.iter())
        .map(|(&time, &distance)| Data {
            time,
            dist_record: distance,
        })
        .collect();

    let mut total = 1;
    for (i, data) in time_distances.iter().enumerate() {
        println!("i: {}, data: {:?}", i, data);
        let a = -1;
        let b = data.time;
        let c = -data.dist_record;

        let (root_1, root_2) = quadratic_roots(a as f64, b as f64, c as f64).unwrap();

        println!("root 1: {:?} root 2: {:?}", root_1, root_2);

        let num_solutions = count_real_integers_between(root_1, root_2);

        println!("num_solutions: {:?}", num_solutions);

        total *= num_solutions;
    }

    println!("total: {:?}", total);
}

fn quadratic_roots(a: f64, b: f64, c: f64) -> Option<(f64, f64)> {
    let discriminant = b * b - 4.0 * a * c;

    if discriminant < 0.0 {
        // No real roots
        None
    } else if discriminant == 0.0 {
        // One real root (repeated)
        let root = -b / (2.0 * a);
        Some((root, root))
    } else {
        // Two real roots
        let sqrt_discriminant = discriminant.sqrt();
        let root1 = (-b + sqrt_discriminant) / (2.0 * a);
        let root2 = (-b - sqrt_discriminant) / (2.0 * a);
        Some((root1, root2))
    }
}

// Do not include the endpoints themselves
fn count_real_integers_between(a: f64, b: f64) -> usize {
    if a >= b {
        return 0;
    }
    let lower_bound = a.floor() as i64 + 1;
    let upper_bound = b.ceil() as i64 - 1;

    if lower_bound > upper_bound {
        0
    } else {
        (upper_bound - lower_bound + 1) as usize
    }
}
