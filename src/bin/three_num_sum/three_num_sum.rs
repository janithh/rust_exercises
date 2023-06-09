pub mod exercise {
    use std::collections::HashMap;

    pub fn get_indexes(values: Vec<i32>, sum: i32) -> Option<(i32, i32, i32)> {
        let elements: usize = values.len();

        if elements < 3 {
            print!("[three_num_sum::get_indexes] Array size is less. Size {}", elements);
            return None;
        }

        let mut data_map: HashMap<usize, i32> = HashMap::new();

        for (index, value) in values.iter().enumerate() {
            data_map.insert(index, *value);
        }

        let mut hash_vec: Vec<(&usize, &i32)> = data_map.iter().collect();
        hash_vec.sort_by(|a, b| a.1.cmp(b.1));

        let mut count: usize = 0;
        while count < elements - 2 {
            let mut left: usize = count + 1;
            let mut right: usize = elements - 1;
            
            while left < right {
                let temp_sum: i32 = *(hash_vec.get(count).unwrap()).1 + *(hash_vec.get(left).unwrap()).1 + *(hash_vec.get(right).unwrap()).1;

                if temp_sum < sum {
                    left += 1;
                }
                else if temp_sum > sum {
                    right -= 1;
                }
                else {
                    let mut temp_result: Vec<i32> = vec![*(hash_vec.get(count).unwrap()).0 as i32, *(hash_vec.get(left).unwrap()).0 as i32, *(hash_vec.get(right).unwrap()).0 as i32];
                    temp_result.sort();
                    return Some((temp_result[0], temp_result[1], temp_result[2]));
                }

            }

            count += 1;
        }

        None
    }
}

#[cfg(test)]
mod test {
    use super::exercise;

    #[test]
    fn ut_get_indexes_normal_match() {
        let data_set: Vec<i32> = vec![3, 24, 12, 15, 2, 7, 6, 11];
        let sum: i32 = 22;

        assert_eq!(exercise::get_indexes(data_set, sum), Some((0, 2, 5)));
        
    }

    #[test]
    fn ut_get_indexes_normal_nomatch() {
        let data_set: Vec<i32> = vec![3, 24, 12, 15, 2, 7, 6, 11];
        let sum: i32 = 100;

        assert_eq!(exercise::get_indexes(data_set, sum), None);        
    }

    #[test]
    fn ut_get_indexes_three_items_match() {
        let data_set: Vec<i32> = vec![3, 24, 12];
        let sum: i32 = 39;

        assert_eq!(exercise::get_indexes(data_set, sum), Some((0, 1, 2)));        
    }

    #[test]
    fn ut_get_indexes_three_items_nomatch() {
        let data_set: Vec<i32> = vec![3, 24, 12];
        let sum: i32 = 100;

        assert_eq!(exercise::get_indexes(data_set, sum), None);        
    }

    #[test]
    fn ut_get_indexes_less_items_match() {
        let data_set: Vec<i32> = vec![3, 24];
        let sum: i32 = 27;

        assert_eq!(exercise::get_indexes(data_set, sum), None);        
    }

    #[test]
    fn ut_get_indexes_less_items_nomatch() {
        let data_set: Vec<i32> = vec![3, 24];
        let sum: i32 = 30;

        assert_eq!(exercise::get_indexes(data_set, sum), None);        
    }

    #[test]
    fn ut_get_indexes_empty() {
        let data_set: Vec<i32> = vec![];
        let sum: i32 = 10;

        assert_eq!(exercise::get_indexes(data_set, sum), None);        
    }
}