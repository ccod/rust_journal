// Problem 1603
#[derive(Debug)]
struct ParkingSystem {
    cap: [i32; 3],
    count: [i32; 3],
}

impl ParkingSystem {
    fn new(big: i32, medium: i32, small: i32) -> Self {
        ParkingSystem {
            cap: [big, medium, small],
            count: [0; 3],
        }
    }

    fn add_car(&mut self, car_type: i32) -> bool {
        match car_type {
            1 | 2 | 3 => {
                if self.cap[(car_type - 1) as usize] > self.count[(car_type - 1) as usize] {
                    self.count[(car_type - 1) as usize] += 1;
                    return true;
                } else {
                    return false;
                }
            }
            _ => panic!("add_car insists on values being 1, 2 or 3"),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check_parking_system() {
        let mut ps = ParkingSystem::new(1, 1, 0);
        assert_eq!(ps.add_car(1), true);
        assert_eq!(ps.add_car(2), true);
        assert_eq!(ps.add_car(3), false);
        assert_eq!(ps.add_car(1), false);
    }
}
