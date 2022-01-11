use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

type Point = (i32, i32, i32);
type Observation = Vec<Point>;

struct PlacedObservation {
    points: Observation,
    offset: Point,
    distances: HashSet<i32>,
}

struct UnplacedObservation {
    points: Observation,
    distances: HashSet<i32>,
}

pub fn run() {
    run_part_01();
}

fn run_part_01() {
    let file = File::open("input/day19.txt").expect("Failed to open file");
    let reader = BufReader::new(file);

    let mut known_set: HashSet<Point> = HashSet::new();

    let mut data: Vec<Observation> = Vec::new();
    data.push(Vec::new());

    let mut obvs_load_head = 0;

    for line in reader.lines() {
        let l = line.expect("Failed to read line");

        if l.starts_with("---") {
            continue;
        }

        if l.is_empty() {
            obvs_load_head += 1;
            data.push(Vec::new());
            continue;
        }

        let split: Vec<&str> = l.split(",").collect();
        assert_eq!(split.len(), 3);

        let x = split[0].parse::<i32>().expect("Failed to parse int");
        let y = split[1].parse::<i32>().expect("Failed to parse int");
        let z = split[2].parse::<i32>().expect("Failed to parse int");

        data[obvs_load_head].push((x, y, z));
    }

    let mut unplaced_observations: Vec<UnplacedObservation> = Vec::new();

    for obs in &data {
        let mut dists: HashSet<i32> = HashSet::new();
        for p in obs {
            for p2 in obs {
                if p == p2 {
                    continue;
                }

                let dif = subtract(p.clone(), p2.clone());

                dists.insert(dif.0 * dif.0 + dif.1 * dif.1 + dif.2 * dif.2);
            }
        }

        unplaced_observations.push(UnplacedObservation {
            points: obs.clone(),
            distances: dists,
        });
    }

    let mut placed_observations: Vec<PlacedObservation> = Vec::new();

    let first = unplaced_observations.swap_remove(0);
    placed_observations.push(PlacedObservation {
        points: first.points,
        offset: (0, 0, 0),
        distances: first.distances,
    });

    for point in &placed_observations[0].points {
        known_set.insert(point.clone());
    }

    let mut nf = 0;
    'outer: while unplaced_observations.len() != 0 {
        for i in 0..unplaced_observations.len() {
            for placed in &placed_observations {
                if !check_distances(&unplaced_observations[i].distances, &placed) {
                    nf += 1;
                    continue;
                }
                for rotation in 0..24 {
                    let rotated = rotate(&unplaced_observations[i].points, rotation);
                    match check_pair(&rotated, placed) {
                        Some(p) => {
                            println!("{},{},{}  {}", p.0, p.1, p.2, i);
                            let r = unplaced_observations.swap_remove(i);
                            let new_obs = PlacedObservation {
                                points: rotated,
                                offset: p,
                                distances: r.distances,
                            };
                            for point in &new_obs.points {
                                known_set.insert(add(point.clone(), new_obs.offset));
                            }

                            placed_observations.push(new_obs);
                            continue 'outer;
                        }
                        None => (),
                    }
                }
            }
        }

        break;
    }
    println!("{}", nf);

    let known = known_set.len();
    println!("There are {} points", known);

    let mut max_manhattan = 0;
    for i in &placed_observations {
        for k in &placed_observations {
            let d = dist(i.offset, k.offset);

            if d > max_manhattan {
                max_manhattan = d;
            }
        }
    }

    println!("The max distance is {}", max_manhattan);
}

fn check_pair(points: &Observation, with: &PlacedObservation) -> Option<Point> {
    for i in 0..with.points.len() {
        for q in 0..points.len() {
            let offset = subtract(with.points[i], points[q]);

            let mut num_match = 0;
            for j in 0..points.len() {
                for k in 0..with.points.len() {
                    if add(points[j], offset) == with.points[k] {
                        num_match += 1;
                        continue;
                    }

                    if num_match >= 12 {
                        return Some(add(offset, with.offset));
                    }
                }
            }
        }
    }

    None
}

fn check_distances(point_distances: &HashSet<i32>, with: &PlacedObservation) -> bool {
    let mut num_distance_match = 0;

    for i in point_distances {
        if with.distances.contains(i) {
            num_distance_match += 1;
        }

        if num_distance_match >= 12 {
            return true;
        }
    }

    return false;
}

fn rotate(points: &Observation, direction: u32) -> Observation {
    points
        .clone()
        .into_iter()
        .map(|x| match direction {
            0 => (-x.2, x.1, x.0),
            1 => (x.2, x.1, -x.0),
            2 => x,
            3 => (-x.0, x.1, -x.2),
            4 => (x.2, -x.1, x.0),
            5 => (-x.2, -x.1, -x.0),
            6 => (-x.0, -x.1, x.2),
            7 => (x.0, -x.1, -x.2),
            8 => (x.2, -x.0, -x.1),
            9 => (-x.2, -x.0, x.1),
            10 => (x.1, -x.0, x.2),
            11 => (-x.1, -x.0, -x.2),
            12 => (-x.2, x.0, -x.1),
            13 => (x.2, x.0, x.1),
            14 => (-x.1, x.0, x.2),
            15 => (x.1, x.0, -x.2),
            16 => (-x.0, -x.2, -x.1),
            17 => (x.0, -x.2, x.1),
            18 => (-x.1, -x.2, x.0),
            19 => (x.1, -x.2, -x.0),
            20 => (x.0, x.2, -x.1),
            21 => (-x.0, x.2, x.1),
            22 => (x.1, x.2, x.0),
            23 => (-x.1, x.2, -x.0),
            _ => panic!(),
        })
        .collect::<Observation>()
}

fn add(x1: Point, x2: Point) -> Point {
    (x1.0 + x2.0, x1.1 + x2.1, x1.2 + x2.2)
}

fn subtract(x1: Point, x2: Point) -> Point {
    (x1.0 - x2.0, x1.1 - x2.1, x1.2 - x2.2)
}

fn dist(x1: Point, x2: Point) -> i32 {
    let s = subtract(x1, x2);
    s.0.abs() + s.1.abs() + s.2.abs()
}