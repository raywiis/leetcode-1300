struct Solution {}

impl Solution {
    pub fn find_best_value(arr: Vec<i32>, target: i32) -> i32 {
        let mut res = 0;

        let mut new_distance = i32::MAX - 1;
        let mut old_distance = i32::MAX;

        while new_distance < old_distance {
            let result: i32= arr.iter()
                .map(|num| if *num > res {
                    res
                } else {
                    *num
                })
                .sum();

            old_distance = new_distance;
            new_distance = (target - result).abs();

            res += 1;
        }

        res - 2
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn example_1() {
        let arr = vec!(4,9,3);
        let target = 10;
        let res = Solution::find_best_value(arr, target);
        assert_eq!(res, 3);
    }

    #[test]
    fn example_2() {
        let arr = vec!(2,3,5);
        let target = 10;
        let res = Solution::find_best_value(arr, target);
        assert_eq!(res, 5);
    }

    #[test]
    fn example_3() {
        let arr = vec!(60864,25176,27249,21296,20204);
        let target = 56803;
        let res = Solution::find_best_value(arr, target);
        assert_eq!(res, 11361);
    }
}
