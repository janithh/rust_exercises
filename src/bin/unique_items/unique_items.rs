
/*******************
 *  Exercise
 *******************/
pub mod exercise {
    use std::cmp;

    pub fn get_unique<T>(input_list: &Vec<T>) -> Vec<T> 
        where T: cmp::PartialOrd + Copy {
        let mut unique_list: Vec<T> = Vec::new();

        for element in input_list {
            match unique_list.iter().find(|&&x| x == *element) {
                None => unique_list.push(element.clone()),
                Some(_) => ()
            };
        }

        unique_list
    }

}

/*******************
 *  Unit tests
 *******************/
#[cfg(test)]
mod test {
    use super::exercise;

    #[test]
    fn ut_get_unique_sorted_unique_input() {
        let input_list : Vec<i32> = vec![1, 4, 5];
        let unique_list = exercise::get_unique(&input_list);

        assert_eq!(unique_list, input_list);
    }

    #[test]
    fn ut_get_unique_sorted_dup_input() {
        let input_list : Vec<i32> = vec![1, 1, 3];
        let unique_list = exercise::get_unique(&input_list);

        assert_eq!(unique_list, vec![1, 3]);
    }
    
    #[test]
    fn ut_get_unique_unsorted_unique_input() {
        let input_list : Vec<i32> = vec![5, 1, 4];
        let unique_list = exercise::get_unique(&input_list);

        assert_eq!(unique_list, input_list);
    }

    #[test]
    fn ut_get_unique_unsorted_dup_input() {
        let input_list : Vec<i32> = vec![1, 3, 1];
        let unique_list = exercise::get_unique(&input_list);

        assert_eq!(unique_list, vec![1, 3]);
    }

    #[test]
    fn ut_get_unique_empty_input() {
        let input_list : Vec<i32> = vec![];
        let unique_list = exercise::get_unique(&input_list);

        assert_eq!(unique_list, vec![]);
    }   
}
