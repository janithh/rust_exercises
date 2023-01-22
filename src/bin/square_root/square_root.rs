
pub mod exercise {
    pub fn calculate(value: i32, precision: f32) -> Option<f32> {
        let mut ret = None;
        
        if value == 0  {
            ret = Some(0.0);
        }
        else if value > 0 && precision >= 0.0 {
            let value = value as f32;
            let mut guess = value;
            let mut root: f32;

            loop {
                root = 0.5 * (guess + (value / guess));

                println!("[square_root::exercise::calculate] guess - {}, root - {}", guess, root);

                if (root - guess).abs() < precision {
                    break;
                }
                
                guess = root;
            }

            ret = Some(root);
        }

        ret        
    }
}

#[cfg(test)]
mod test {
    use crate::square_root::exercise::calculate;

    use super::exercise;

    #[test]
    fn ut_calculate_normal() {
        let input = 256;
        let precision = 0.001;

        assert_eq!(calculate(input, precision), Some(16.0));
    }

    #[test]
    fn ut_calculate_input_negative() {
        let input = -256;
        let precision = 0.001;

        assert_eq!(calculate(input, precision), None);
    }

    #[test]
    fn ut_calculate_input_zero() {
        let input = 0;
        let precision = 0.001;

        assert_eq!(calculate(input, precision), Some(0.0));
    }

    #[test]
    fn ut_calculate_input_decimal_precision_four_decimal() {
        let input = 300;
        let precision = 0.001;

        assert_eq!(calculate(input, precision), Some(17.320507));
    }

    #[test]
    fn ut_calculate_input_decimal_precision_one() {
        let input = 300;
        let precision = 1.0;

        assert_eq!(calculate(input, precision), Some(17.341732));
    }

    #[test]
    fn ut_calculate_input_decimal_precision_negative() {
        let input = 300;
        let precision = -0.00001;

        assert_eq!(calculate(input, precision), None);
    }
}
