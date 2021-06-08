use crate::utils;

pub fn part_a() -> i32 {
    let mut total = 0.0;
    if let Ok(lines) = utils::read_lines("inputs/day01.txt") {
        for line in lines {
            if let Ok(mass) = line {
                total += calc_fuel_requirement(mass.parse::<f32>().unwrap());
            }
        }
    }
    total as i32
}

pub fn part_b() -> i32 {
    let mut total = 0.0;
    if let Ok(lines) = utils::read_lines("inputs/day01.txt") {
        for line in lines {
            if let Ok(mass) = line {
                let mut mass = mass.parse::<f32>().unwrap();
                loop {
                    mass = calc_fuel_requirement(mass);
                    if mass > 0.0 {
                        total += mass;
                    } else {
                        break;
                    }
                }
            }
        }
    }
    total as i32
}

fn calc_fuel_requirement(mass: f32) -> f32 {
    ((mass / 3.0).floor()) - 2.0
}
