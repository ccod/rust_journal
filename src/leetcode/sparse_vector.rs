use std::cmp::Ordering;

// Problem 1570
#[derive(Debug)]
struct Pair(usize, i32);
#[derive(Debug)]
struct SparseVector {
    size: usize,
    points: Vec<Pair>,
}

impl SparseVector {
    fn new(nums: Vec<i32>) -> Self {
        let mut a: Vec<Pair> = Vec::new();
        for (pos, i) in nums.iter().enumerate() {
            if *i != 0 {
                a.push(Pair(pos, *i))
            }
        }
        SparseVector {
            size: nums.len(),
            points: a,
        }
    }

    fn dot_product(&self, vec: SparseVector) -> i32 {
        let mut a_idx: usize = 0;
        let mut b_idx: usize = 0;
        let mut sum: i32 = 0;
        while a_idx < self.points.len() && b_idx < vec.points.len() {
            match self.points[a_idx].0.cmp(&vec.points[b_idx].0) {
                Ordering::Greater => b_idx += 1,
                Ordering::Less => a_idx += 1,
                Ordering::Equal => {
                    sum += self.points[a_idx].1 * vec.points[b_idx].1;
                    a_idx += 1;
                }
            }
        }
        sum
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn chek_dot_product() {
        let v1 = SparseVector::new(vec![1, 0, 0, 2, 3]);
        let v2 = SparseVector::new(vec![0, 3, 0, 4, 0]);
        // println!("v1: {:?}\nv2: {:?}", v1, v2);
        assert_eq!(v1.dot_product(v2), 8);

        let v1 = SparseVector::new(vec![0, 1, 0, 0, 0]);
        let v2 = SparseVector::new(vec![0, 0, 0, 0, 2]);
        assert_eq!(v1.dot_product(v2), 0);

        let v1 = SparseVector::new(vec![0, 1, 0, 0, 2, 0, 0]);
        let v2 = SparseVector::new(vec![1, 0, 0, 0, 3, 0, 4]);
        assert_eq!(v1.dot_product(v2), 6);
    }
}
