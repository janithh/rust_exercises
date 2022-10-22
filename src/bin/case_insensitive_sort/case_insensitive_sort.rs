
/*******************
 *  Exercise
 *******************/
pub mod exercise {
    pub fn sort_usernames(user_list: &mut Vec<String>) {
        if user_list.len() < 2 {
            return;
        }

        user_list.sort_by(|a, b| {a.to_lowercase().cmp(&b.to_lowercase())});      
    }
}

/*******************
 *  Unit tests
 *******************/
 #[cfg(test)]

mod test {
    use super::exercise;
  
    #[test]
    fn sort_usernames_normal_1() {
        let mut dataset = vec![String::from("Tom"), String::from("amy")]; 

        exercise::sort_usernames(&mut dataset);

        assert_eq!(dataset, vec![String::from("amy"), String::from("Tom")]);
    }

    #[test]
    fn sort_usernames_normal_2() {
        let mut dataset = vec![String::from("Tom"), String::from("Satyajith"), String::from("amy"), String::from("satya99"), String::from("MAhesh"), String::from("maheel")]; 

        exercise::sort_usernames(&mut dataset);

        assert_eq!(dataset, vec![String::from("amy"), String::from("maheel"), String::from("MAhesh"), String::from("satya"), String::from("Satyajith"), String::from("Tom")]);
    }

    #[test]
    fn sort_usernames_empty() {
        let mut dataset = vec![]; 
        let checkset: Vec<String> = vec![];

        exercise::sort_usernames(&mut dataset);

        assert_eq!(dataset, checkset);
    }

    #[test]
    fn sort_usernames_one_element() {
        let mut dataset = vec![String::from("tom")]; 

        exercise::sort_usernames(&mut dataset);

        assert_eq!(dataset, vec![String::from("tom")]);
    }
}
