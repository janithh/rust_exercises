
/*************
 * Exercise
 *************/
pub mod exercise {
    use std::collections::HashMap;

    pub fn get_indexes(data_list: Vec<i32>, check_sum: i32) -> Option<Vec<i32>> {
        if data_list.len() < 2 {
            return None;
        }

        let mut data_map: HashMap<usize, i32> = HashMap::new();

        for (index, value) in data_list.iter().enumerate() {
            data_map.insert(index, *value);
        }

        let mut hash_vec: Vec<(&usize, &i32)> = data_map.iter().collect();
        hash_vec.sort_by(|a, b| b.1.cmp(a.1));

        print!("[two_sum::exercise::get_indexes] {:?}", hash_vec);

        let mut i: usize = 0;
        let mut j: usize = hash_vec.len() - 1;
       
        while i != j {
            let key_value_i;
            let key_value_j;
            match hash_vec.get(i) {
                Some(a) => key_value_i = *a,
                None => return None
            };
            match hash_vec.get(j) {
                Some(a) => key_value_j = *a,
                None => return None
            };

            let sum = *key_value_i.1 + *key_value_j.1;
            if sum == check_sum {
                if *key_value_i.0 < *key_value_j.0 {
                    return Some(vec![*key_value_i.0 as i32, *key_value_j.0 as i32]);
                } 
                else {
                    return Some(vec![*key_value_j.0 as i32, *key_value_i.0 as i32]);
                }
            }
            else if sum < check_sum {
                j -= 1; 
            }
            else {
                i += 1;
            }

        }

        Some(vec![])
    }
}

/*************
 * Unit tests
 *************/
 #[cfg(test)]
 mod test {
    use super::exercise;

    #[test]
    fn ut_get_indexes_empty() {
        let check_list: Vec<i32> = vec![];
        let check_sum: i32 = 5;
        assert_eq!(exercise::get_indexes(check_list, check_sum), None);
    }

    #[test]
    fn ut_get_indexes_one_element() {
        let check_list: Vec<i32> = vec![7];
        let check_sum: i32 = 7;
        assert_eq!(exercise::get_indexes(check_list, check_sum), None);
    }


    #[test]
    fn ut_get_indexes_has_match() {
        let check_list: Vec<i32> = vec![1, 8, 3, 5, 2];
        let check_sum: i32 = 4;
        assert_eq!(exercise::get_indexes(check_list, check_sum), Some(vec![0, 2]));
    }


    #[test]
    fn ut_get_indexes_no_match() {
        let check_list: Vec<i32> = vec![1, 8, 3, 5, 2];
        let check_sum: i32 = 20;
        assert_eq!(exercise::get_indexes(check_list, check_sum), Some(vec![]));
    }

}

