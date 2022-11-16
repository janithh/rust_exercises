
pub mod exercise {
    pub fn sum_with_missing(input: Vec<Option<i32>>) -> i32 {
        /* MY ANSWER     
            let mut sum = 0;

            for value in input  {
                sum += match value {
                            Some(a) => a,
                            None => 0
                        };
            
            }

            sum
        */
        input.iter().map(|x| x.unwrap_or(0)).sum()
    }
}


#[cfg(test)]
mod test {
    use super::exercise;

    #[test]
    fn ut_sum_with_missing_all_values() {
        let input = vec![Some(1), Some(8), Some(15), Some(7), Some(10)];

        let sum = exercise::sum_with_missing(input);

        assert_eq!(sum, 41);
    }

    #[test]
    fn ut_sum_with_missing_missing_values() {
        let input = vec![Some(1), None, Some(15), Some(7), Some(10)];

        let sum = exercise::sum_with_missing(input);

        assert_eq!(sum, 33);
    }

    #[test]
    fn ut_sum_with_missing_all_missing_values() {
        let input = vec![None, None, None, None, None];

        let sum = exercise::sum_with_missing(input);

        assert_eq!(sum, 0);
    }

    #[test]
    fn ut_sum_with_missing_empty() {
        let input = vec![];

        let sum = exercise::sum_with_missing(input);

        assert_eq!(sum, 0);
    }

}