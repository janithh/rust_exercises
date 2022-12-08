use chrono::{DateTime, Local, TimeZone, LocalResult};

#[derive(PartialEq, Debug)]
pub struct ImportantEvent {
    event_name: String,
    date: DateTime<Local>
}

impl ImportantEvent {
    fn new(input_name: &str, input_date: (i32, u32, u32, u32, u32, u32)) -> Result<Self, &str> {
        let event_name = String::from(input_name);
        let converted_date = Local.with_ymd_and_hms(input_date.0, input_date.1, input_date.2, input_date.3, input_date.4, input_date.5);

        if let LocalResult::None = converted_date {
            return Err("Invalid Date information");            
        }
        else if event_name.is_empty() {
            return Err("Event name cannot be empty");
        }

        Ok(ImportantEvent { event_name, date: converted_date.unwrap() })
    }    
}

trait Deadline {
    fn is_expired(&self) -> bool;
}

impl Deadline for ImportantEvent {
    fn is_expired(&self) -> bool {
        self.date < Local::now()
    }
    
}

 /*******************
 *  Exercise
 *******************/
pub mod exercise {
    use super::{ImportantEvent, Deadline};

    pub fn create_event(name: &str, date: (i32, u32, u32, u32, u32, u32)) -> Option<ImportantEvent> {
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
        let event = create_event("Test Event", (2021, 1, 1, 19, 0, 0));

        assert_ne!(event, None);
    }

    #[test]
    fn ut_create_event_name_empty() {
        let event = create_event("", (2021, 1, 1, 19, 0, 0));

        assert_eq!(event, None);
    }

    #[test]
     fn ut_create_event_year_invalid() {
        let event = create_event("Test Event", (-200000, 1, 1, 30, 0, 0));

        assert_eq!(event, None);
    }

    #[test]
    fn ut_create_event_month_invalid() {
        let event = create_event("Test Event", (2021, 15, 1, 19, 0, 0));

        assert_eq!(event, None);
    }

    #[test]
    fn ut_create_event_date_invalid() {
        let event = create_event("Test Event", (2021, 1, 35, 19, 0, 0));

        assert_eq!(event, None);
    }

    #[test]
    fn ut_create_event_hour_invalid() {
        let event = create_event("Test Event", (2021, 1, 1, 30, 0, 0));

        assert_eq!(event, None);
    }

    #[test]
    fn ut_create_event_minute_invalid() {
        let event = create_event("Test Event", (2021, 1, 1, 19, 100, 0));

        assert_eq!(event, None);
    }

    #[test]
    fn ut_create_event_second_invalid() {
        let event = create_event("Test Event", (2021, 1, 1, 19, 0, 100));

        assert_eq!(event, None);
    }

    #[test]
    fn ut_check_status_valid() {
        let event = create_event("Test Event", (2025, 1, 1, 19, 0, 0));

        assert_eq!(check_status(event.unwrap()), true);
    }

    #[test]
    fn ut_check_status_expired() {
        let event = create_event("Test Event", (2021, 1, 1, 19, 0, 0));

        assert_eq!(check_status(event.unwrap()), false);
    }
}