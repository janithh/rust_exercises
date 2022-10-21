
/*******************
 *  Exercise
 *******************/
pub mod exercise {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut nums = nums;
        
        if nums.len() < 2 {
            return false;            
        }

        nums.sort();

        let length = nums.len();
        let mut count = 0;

        while count < length - 1 {
            if nums[count] == nums[count + 1] {
                return true;
            }
            count += 1;
        }

        false
    }
}

/*******************
 *  Unit tests
 *******************/
 #[cfg(test)]

mod test {
    use super::exercise;
  
    #[test]
    fn ut_contains_duplicate_vec_1() {
        let input = vec![1, 2, 3, 1];
        let result = exercise::contains_duplicate(input);
    
        assert_eq!(result, true);
    }

    #[test]
    fn ut_contains_duplicate_vec_2() {
        let input = vec![1, 2, 3, 4];
        let result = exercise::contains_duplicate(input);
    
        assert_eq!(result, false);
    }

    #[test]
    fn ut_contains_duplicate_vec_3() {
        let input = vec![1, 1, 1, 3, 3, 4, 3, 2, 4, 2];
        let result = exercise::contains_duplicate(input);
    
        assert_eq!(result, true);
    }

    #[test]
    fn ut_contains_duplicate_vec_empty() {
        let input = vec![];
        let result = exercise::contains_duplicate(input);
    
        assert_eq!(result, false);
    }

    #[test]
    fn ut_contains_duplicate_vec_one_element() {
        let input = vec![5];
        let result = exercise::contains_duplicate(input);
    
        assert_eq!(result, false);
    }

    #[test]
    fn ut_contains_duplicate_vec_two_element() {
        let input = vec![5, 5];
        let result = exercise::contains_duplicate(input);
    
        assert_eq!(result, true);
    }
}
