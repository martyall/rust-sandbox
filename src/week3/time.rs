const NOW: &str = "2019-06-26";
use time::macros::format_description;
use time::{Date, Month};

struct User {
    birthdate: Date,
}

impl User {
    fn with_birthdate(year: i32, month: u32, day: u32) -> Self {
        let month = Month::try_from(month as u8).unwrap();

        User {
            birthdate: Date::from_calendar_date(year, month, day as u8).unwrap(),
        }
    }

    /// Returns current age of [`User`] in years.
    fn age(&self) -> u16 {
        let format = format_description!("[year]-[month]-[day]");
        let now = Date::parse(NOW, format).unwrap();
        let a: time::Duration = now - self.birthdate;
        if a.is_positive() {
            (a.whole_days() / 365) as u16
        } else {
            0
        }
    }

    /// Checks if [`User`] is 18 years old at the moment.
    fn is_adult(&self) -> bool {
        self.age() > 18
    }
}

#[cfg(test)]
mod age_spec {
    use super::*;

    #[test]
    fn counts_age() {
        for ((y, m, d), expected) in vec![
            ((1990, 6, 4), 29),
            ((1990, 7, 4), 28),
            //((0, 1, 1), 2019),
            ((1970, 1, 1), 49),
            ((2019, 6, 25), 0),
        ] {
            let user = User::with_birthdate(y, m, d);
            assert_eq!(user.age(), expected);
        }
    }

    #[test]
    fn zero_if_birthdate_in_future() {
        for ((y, m, d), expected) in vec![
            ((2032, 6, 25), 0),
            ((2016, 6, 27), 2),
            ((3000, 6, 27), 0),
            ((9999, 6, 27), 0),
        ] {
            let user = User::with_birthdate(y, m, d);
            assert_eq!(user.age(), expected);
        }
    }
}
