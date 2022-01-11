use std::fs;

pub fn run() {
    println!("--Part 1");
    run_part_01();
    println!("--Part 2");
    run_part_02();
}

fn run_part_01() {
    let input:String = fs::read_to_string("input/day17.txt").expect("Failed to read file");

    let reduced = &input[15..];
    let mut split = reduced.split(", y=");
    let x_bounds = get_bounds(&mut split).expect("Failed to parse x bounds");
    let y_bounds = get_bounds(&mut split).expect("Failed to parse y bounds");

    let best_x_speed = (0.5 * (f64::sqrt(8.0 * x_bounds.0 as f64 + 1.0) - 1.0)).ceil() as i32;
    let best_y_speed = (-y_bounds.0) - 1;

    let mut sim_point = (0, 0);
    let mut sim_vel = (best_x_speed, best_y_speed);

    let mut max_height = 0;
    let mut intersected = false;

    while sim_point.1 > y_bounds.0 {
        sim_point.0 += sim_vel.0;
        sim_point.1 += sim_vel.1;
        sim_vel.0 -= sim_vel.0.signum();
        sim_vel.1 -= 1;

        max_height = max_height.max(sim_point.1);

        intersected |= contains_point(sim_point, x_bounds, y_bounds);
    }

    println!("The max height was {}, and its intersect value is {}", max_height, intersected);
}

fn run_part_02() {
    let input:String = fs::read_to_string("input/day17.txt").expect("Failed to read file");

    let reduced = &input[15..];
    let mut split = reduced.split(", y=");
    let x_bounds = get_bounds(&mut split).expect("Failed to parse x bounds");
    let y_bounds = get_bounds(&mut split).expect("Failed to parse y bounds");

    let min_x_speed = (0.5 * (f64::sqrt(8.0 * x_bounds.0 as f64 + 1.0) - 1.0)).ceil() as i32;
    let max_x_speed = x_bounds.1;
    let min_y_speed = y_bounds.0;
    let max_y_speed = (-y_bounds.0) - 1;

    let mut good_vels:Vec<(i32, i32)> = Vec::new();

    for x in min_x_speed..=max_x_speed {
        for y in min_y_speed..=max_y_speed {
            let mut sim_point = (0, 0);
            let mut sim_vel = (x, y);

            while sim_point.1 > y_bounds.0 && sim_point.0 < x_bounds.1 {
                sim_point.0 += sim_vel.0;
                sim_point.1 += sim_vel.1;
                sim_vel.0 -= sim_vel.0.signum();
                sim_vel.1 -= 1;

                if contains_point(sim_point, x_bounds, y_bounds) {
                    good_vels.push((x, y));
                    break;
                }
            }
        }
    }

    println!("The number of good values is: {}", good_vels.len());
}

fn get_bounds<'a>(iter: &mut impl Iterator<Item = &'a str>) -> Option<(i32, i32)> {
    match iter.next() {
        Some(x) => {
            let split = x.split("..").collect::<Vec<&str>>();
            assert_eq!(split.len(), 2);
            let x = split[0].parse::<i32>().unwrap();
            let y = split[1].parse::<i32>().unwrap();

            if x > y {
                Some((y, x))
            } else {
                Some((x, y))
            }
        },
        None => {
            return None
        }
    }
}

fn contains_point(point:(i32, i32), x_bounds:(i32, i32), y_bounds:(i32, i32)) -> bool {
    point.0 >= x_bounds.0 && point.0 <= x_bounds.1 && point.1 >= y_bounds.0 && point.1 <= y_bounds.1
}