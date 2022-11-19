
pub mod exercise {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut element_map: std::collections::HashMap<i32, i32> = std::collections::HashMap::new();
        let mut elements: Vec<i32> = vec![];

        for num in nums {
            element_map.entry(num).and_modify(|x| *x += 1).or_insert(1);
        }

        let mut value_pairs:Vec<(i32, i32)> = element_map.iter().map(|(&k, &v)| {(v, k)}).collect();

        if value_pairs.len() != 0 {
            value_pairs.sort_by(|a, b| a.0.cmp(&b.0));

            let mut count = 0;
            while (count < value_pairs.len()) && count < k.try_into().unwrap() { 
                elements.insert(count, value_pairs[value_pairs.len() - 1 - count].1);
                count += 1;
            }
        }

        elements
    }
}


#[cfg(test)]

mod test {
    use super::exercise;

    #[test]
    fn ut_top_k_frequent_normal_ordered() {
        let input = vec![1, 1, 1, 2, 2, 3];
        let k = 2;

        let output = exercise::top_k_frequent(input, k);

        assert_eq!(output, vec![1, 2]);
    }

    #[test]
    fn ut_top_k_frequent_normal_shuffled() {
        let input = vec![1, 2, 3, 1, 1, 2];
        let k = 2;

        let output = exercise::top_k_frequent(input, k);

        assert_eq!(output, vec![1, 2]);
    }

    #[test]
    fn ut_top_k_frequent_one_entry() {
        let input = vec![1];
        let k = 2;

        let output = exercise::top_k_frequent(input, k);

        assert_eq!(output, vec![1]);
    }

    #[test]
    fn ut_top_k_frequent_one_entry_multiple() {
        let input = vec![1, 1, 1, 1, 1];
        let k = 2;

        let output = exercise::top_k_frequent(input, k);

        assert_eq!(output, vec![1]);
    }

    #[test]
    fn ut_top_k_frequent_no_entry() {
        let input = vec![];
        let k = 2;

        let output = exercise::top_k_frequent(input, k);

        assert_eq!(output, vec![]);
    }
}