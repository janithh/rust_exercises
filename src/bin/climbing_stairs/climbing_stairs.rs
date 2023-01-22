
pub mod exercise {
    pub fn calculate_ways(steps: u32) -> u128 {
        let mut ret = steps as u128;
        
        if steps > 4 {
            let mut count = 4;
            let mut value_2: u128 = 2;
            let mut value_1:u128 = 3;
            let mut value: u128 = 0;
            
            while count <= steps {
                value = value_2 + value_1;
                value_2 = value_1;
                value_1 = value;

                count += 1;
            }

            ret = value as u128;
        }

        ret
    }
}

#[cfg(test)]
mod test {
    use super::exercise;

    #[test]
    fn ut_calculate_steps_normal() {
        let input = 32;

        assert_eq!(exercise::calculate_ways(input), 3524578);
    }

    #[test]
    fn ut_calculate_steps_one() {
        let input = 1;

        assert_eq!(exercise::calculate_ways(input), 1);
    }

    #[test]
    fn ut_calculate_steps_zero() {
        let input = 0;

        assert_eq!(exercise::calculate_ways(input), 0);
    }

    #[test]
    fn ut_calculate_steps_hundred() {
        let input = 100;

        assert_eq!(exercise::calculate_ways(input), 573147844013817084101);
    }

}