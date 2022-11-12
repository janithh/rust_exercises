use chrono::{Date, Local, TimeZone, LocalResult};

#[derive(PartialEq, Debug)]
pub struct ImportantEvent {
    event_name: String,
    date: Date<Local>
}

impl ImportantEvent {
    fn new(input_name: &str, input_date: (i32, u32, u32)) -> Result<Self, &str> {
        let string_name = String::from(input_name);
        let converted_date = Local.ymd_opt(input_date.0, input_date.1, input_date.2);

        if let LocalResult::None = converted_date {
            return Err("Invalid Date information");            
        }
        else if string_name.is_empty() {
            return Err("Event name cannot be empty");
        }

        Ok(ImportantEvent { event_name: string_name, date: converted_date.unwrap() })
    }    
}

trait Deadline {
    fn is_expired(&self) -> bool;
}

impl Deadline for ImportantEvent {
    fn is_expired(&self) -> bool {
        self.date < Local::today()
    }
    
}

 /*******************
 *  Exercise
 *******************/
pub mod exercise {
    use super::{ImportantEvent, Deadline};

    pub fn create_event(name: &str, date: (i32, u32, u32)) -> Option<ImportantEvent> {
        match ImportantEvent::new(name, date) {
            Ok(a) => Some(a),
            Err(b)          => {
                            println!("[check_deadline::create_event] ERROR : {}", b);
                            None
                        }
        }
    }

    pub fn check_status(event: ImportantEvent) -> bool {
        event.is_expired() == false
    } 
}

 /*******************
 *  Unit tests
 *******************/
 #[cfg(test)]
 mod test {
    use super::exercise::{create_event, check_status};

    #[test]
    fn ut_create_event_normal() {
        let event = create_event("Test Event", (2021, 1, 1));

        assert_ne!(event, None);
    }

    #[test]
    fn ut_create_event_name_empty() {
        let event = create_event("", (2021, 1, 1));

        assert_eq!(event, None);
    }

    #[test]
    #[should_panic]
     fn ut_create_event_year_invalid() {
        let event = create_event("Test Event", (-200000, 1, 1));

        assert_eq!(event, None);
    }

    #[test]
    fn ut_create_event_month_invalid() {
        let event = create_event("Test Event", (2021, 15, 1));

        assert_eq!(event, None);
    }

    #[test]
    fn ut_create_event_date_invalid() {
        let event = create_event("Test Event", (2021, 1, 35));

        assert_eq!(event, None);
    }

    #[test]
    fn ut_check_status_valid() {
        let event = create_event("Test Event", (2025, 1, 1));

        assert_eq!(check_status(event.unwrap()), true);
    }

    #[test]
    fn ut_check_status_expired() {
        let event = create_event("Test Event", (2021, 1, 1));

        assert_eq!(check_status(event.unwrap()), false);
    }
}