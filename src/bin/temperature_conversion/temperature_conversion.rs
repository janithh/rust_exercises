use core::fmt;

#[derive(Debug, PartialEq)]
pub enum Scale {
    Fahrenheit,
    Celcius
}

impl Scale {
    fn to_string(&self) -> &str {
        if let Scale::Celcius = self {
            "Celcius"
        }
        else {
            "Fahrenheit"
        }
    } 
}

pub struct Temperature {
    degrees: f32,
    scale: Scale
}

impl Temperature {
    fn new(input_temp: f32, input_scale: Scale) -> Self {
        Temperature {
            degrees: input_temp,
            scale: input_scale
        }
    }

    fn to_celcius(&mut self) {
        if let Scale::Fahrenheit = self.scale {
            self.degrees = (self.degrees - 32.0) * 5.0 / 9.0;
            self.scale = Scale::Celcius;
        }
    }

    fn to_fahrenheit(&mut self) {
        if let Scale::Celcius = self.scale { 
            self.degrees = (self.degrees * 9.0 / 5.0) + 32.0 ;
            self.scale = Scale::Fahrenheit;            
        }
    }
}

impl fmt::Display for Temperature {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Temperature - {}, Scale - {}", self.degrees, self.scale.to_string())
    }
}

pub mod exercise {
    use super::{Scale, Temperature};

    pub fn create_temperature(temperature: f32, scale: Scale) -> Temperature {
        Temperature::new(temperature, scale)
    }

    pub fn convert(temperature: &mut Temperature, scale: Scale) {
        if let Scale::Fahrenheit = scale { temperature.to_fahrenheit() } else { temperature.to_celcius() }; 
    }
}

/*******************
 *  Unit tests
 *******************/
#[cfg(test)]
mod test {
    use super::{Scale, exercise};

    #[test]
    fn ut_create_temperature_celcius() {
        let temperature = exercise::create_temperature(40.5, Scale::Celcius);

        assert_eq!(temperature.degrees, 40.5);
        assert_eq!(temperature.scale, Scale::Celcius);        
    }

    #[test]
    fn ut_create_temperature_fahrenheit() {
        let temperature = exercise::create_temperature(40.5, Scale::Fahrenheit);

        assert_eq!(temperature.degrees, 40.5);
        assert_eq!(temperature.scale, Scale::Fahrenheit);        
    }

    #[test]
    fn ut_convert_celcius_to_fahrenheit() {
        let mut temperature = exercise::create_temperature(40.5, Scale::Celcius);

        exercise::convert(&mut temperature, Scale::Fahrenheit);

        assert_eq!(temperature.degrees, 104.9);
        assert_eq!(temperature.scale, Scale::Fahrenheit);        
    }
    
    #[test]
    fn ut_convert_fahrenheit_to_celcius() {
        let mut temperature = exercise::create_temperature(40.5, Scale::Fahrenheit);

        exercise::convert(&mut temperature, Scale::Celcius);

        assert_eq!(temperature.degrees, 4.7222223);
        assert_eq!(temperature.scale, Scale::Celcius);        
    }  
}