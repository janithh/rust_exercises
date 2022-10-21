
/*******************
 *  Exercise
 *******************/
pub mod exercise {
    use std::collections::HashMap;

    pub fn get_first(input: String) -> i32 {
        let mut unique_map: HashMap<char, i32> = HashMap::new();
        let mut dup_map: Vec<char> = Vec::new();
        
        for (index, item) in input.chars().enumerate() {
            if dup_map.contains(&item) ==false {
                if unique_map.contains_key(&item) {
                    unique_map.remove(&item);
                    dup_map.push(item);
                }
                else {
                    unique_map.insert(item, index as i32);
                }
            }

        }

        let num_char = unique_map.len(); 
        if num_char == 0 {
            return -1;
        }

        let mut hash_vec: Vec<(&char, &i32)> = unique_map.iter().collect();
        hash_vec.sort_by(|a, b| b.1.cmp(a.1));

        match hash_vec.get(num_char - 1) {
            Some(a) => *(*a).1,
            None => return -1
        }
    }
}

/*******************
 *  Unit tests
 *******************/
#[cfg(test)]

mod test {
    use super::exercise;
 
    #[test]
    fn ut_get_first_normal_1() {
        let input = String::from("leetcode");
        let index = exercise::get_first(input);

        assert_eq!(index, 0);
    }

    #[test]
    fn ut_get_first_normal_2() {
        let input = String::from("loveleetcode");
        let index = exercise::get_first(input);

        assert_eq!(index, 2);
    }

    #[test]
    fn ut_get_first_special_char() {
        let input = String::from("lo#veleet$code");
        let index = exercise::get_first(input);

        assert_eq!(index, 2);
    }

    #[test]
    fn ut_get_first_no_match() {
        let input = String::from("aadadaad");
        let index = exercise::get_first(input);

        assert_eq!(index, -1);
    }
}