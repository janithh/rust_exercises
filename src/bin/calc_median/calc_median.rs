pub mod exercise {
    pub fn calculate_median(value_list: &mut Vec<f64>) -> Option<f64> {
        if value_list.is_empty() {
            return None;
        }

        value_list.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let element_count = value_list.len();

        let median: f64 = if element_count % 2 == 0 {
                                (value_list.get(element_count / 2).unwrap() + value_list.get(element_count / 2 - 1).unwrap()) / 2.0
                            }
                            else {
                                *value_list.get(element_count / 2).unwrap()
                            };

        Some(median)
    }
}

#[cfg(test)]
mod test {
    use super::exercise;

    #[test]
    fn ut_calculate_median_odd_sorted() {
        let mut list: Vec<f64> = vec![1.0, 2.0, 3.2, 4.7, 8.7];

        let median = exercise::calculate_median(&mut list);

        assert_eq!(median, Some(3.2));
    }

    #[test]
    fn ut_calculate_median_odd_unsorted() {
        let mut list: Vec<f64> = vec![4.7, 1.0, 8.7, 3.2, 2.0];

        let median = exercise::calculate_median(&mut list);

        assert_eq!(median, Some(3.2));
    }

    #[test]
    fn ut_calculate_median_even_sorted() {
        let mut list: Vec<f64> = vec![1.0, 2.0, 3.2, 4.1, 4.7, 8.7];

        let median = exercise::calculate_median(&mut list);

        assert_eq!(median, Some(3.65));
    }

    #[test]
    fn ut_calculate_median_even_unsorted() {
        let mut list: Vec<f64> = vec![4.7, 1.0, 8.7, 3.2, 4.1, 2.0];

        let median = exercise::calculate_median(&mut list);

        assert_eq!(median, Some(3.65));
    }

    #[test]
    fn ut_calculate_median_one_element() {
        let mut list: Vec<f64> = vec![4.7];

        let median = exercise::calculate_median(&mut list);

        assert_eq!(median, Some(4.7));
    }

    #[test]
    fn ut_calculate_median_empty() {
        let mut list: Vec<f64> = vec![];

        let median = exercise::calculate_median(&mut list);

        assert_eq!(median, None);
    }
}