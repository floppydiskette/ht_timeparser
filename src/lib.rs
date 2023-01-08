use std::fmt::{Display, Formatter};
use std::num::Wrapping;
use ht_cal::datetime::{HDateTime, Month, MonthStatus};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HTDate {
    pub year: u128,
    pub month: (MonthStatus, Month),
    pub day: u8,
    pub second: u128,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HTParseError {
    MalformedString,
    TooManyDays,
    OtherwiseInvalidDate,
}

pub fn parse_month_from_gl_and_m(gl_str: &str, month_str: &str) -> Result<(MonthStatus, Month), HTParseError> {
    Ok(match gl_str {
        "G" => (MonthStatus::Greater, match month_str {
            "Z" => Month::Zero,
            "N" => Month::Niktvirin,
            "A" => Month::Apress,
            "S" => Month::Smosh,
            "F" => Month::Funny,
            _ => return Err(HTParseError::MalformedString),
        }),
        "L" => (MonthStatus::Lesser, match month_str {
            "Z" => Month::Zero,
            "N" => Month::Niktvirin,
            "A" => Month::Apress,
            "S" => Month::Smosh,
            "F" => Month::Funny,
            _ => return Err(HTParseError::MalformedString),
        }),
        _ => return Err(HTParseError::MalformedString),
    })
}

impl Display for HTDate {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let gl = if self.month.0 == MonthStatus::Greater { "G" } else { "L" };
        let month = match self.month.1 {
            Month::Zero => "Z",
            Month::Niktvirin => "N",
            Month::Apress => "A",
            Month::Smosh => "S",
            Month::Funny => "F",
        };
        let year_padded = format!("{:0>4}", self.year); // todo: add more digits when year is greater than 9999
        let day_padded = format!("{:0>2}", self.day);
        let sks = self.second / 6000;
        let rem = self.second % 6000;
        let sks_padded = format!("{:0>2}", sks);
        let rem_padded = format!("{:0>4}", rem);
        format!("{}-{}{}-{}T{}S{}R", year_padded, gl, month, day_padded, sks_padded, rem_padded).fmt(f)
    }
}

impl HTDate {
    pub fn new(year: u128, month_status: MonthStatus, month: Month, day: u8, second: u128) -> HTDate {
        HTDate {
            year,
            month: (month_status, month),
            day,
            second,
        }
    }
    pub fn to_hdatetime(&self) -> HDateTime {
        let mut hdt = HDateTime::new();
        hdt.year = self.year;
        hdt.month = (self.month.0, self.month.1);
        hdt.day = self.day;
        hdt.second = Wrapping(self.second);
        hdt
    }

    pub fn from_hdatetime(htdatetime: &HDateTime) -> Self {
        HTDate {
            year: htdatetime.year,
            month: htdatetime.month,
            day: htdatetime.day,
            second: htdatetime.second.0,
        }
    }

    pub fn to_string_no_secs(&self) -> String {
        let gl = if self.month.0 == MonthStatus::Greater { "G" } else { "L" };
        let month = match self.month.1 {
            Month::Zero => "Z",
            Month::Niktvirin => "N",
            Month::Apress => "A",
            Month::Smosh => "S",
            Month::Funny => "F",
        };
        let year_padded = format!("{:0>4}", self.year); // todo: add more digits when year is greater than 9999
        let day_padded = format!("{:0>2}", self.day);
        format!("{}-{}{}-{}", year_padded, gl, month, day_padded)
    }

    pub fn interpret_string(input: &str) -> Result<Self, HTParseError> {
        // string may be in the format of "YYYY-GM-DDTSSSRRRRR" or "YYYY-GM-DD"
        // or it may not have dashes, in which case assume it's either "YYYYGMDDTSSSRRRRR" or "YYYYGMDD"
        let mut year = 0;
        let mut month = (MonthStatus::Greater, Month::Zero);
        let mut day = 0;
        let mut second = 0u128;

        if input.len() > 8 { // cannot be YYYYGMDD
            match input.len() {
                17 => { // YYYYGMDDTNNSNNNNR
                    let year_str = &input[0..4]; // 4
                    let gl_str = &input[4..5]; // 1
                    let month_str = &input[5..6]; // 1
                    let day_str = &input[6..8]; // 2
                    let sks_str = &input[9..11]; // 2
                    let rem_str = &input[12..16]; // 4
                    year = year_str.parse().map_err(|_| HTParseError::MalformedString)?;
                    month = parse_month_from_gl_and_m(gl_str, month_str)?;
                    day = day_str.parse().map_err(|_| HTParseError::MalformedString)?;
                    second = sks_str.parse().map_err(|_| HTParseError::MalformedString)?;
                    second *= 6000;
                    second += rem_str.parse::<u128>().map_err(|_| HTParseError::MalformedString)?;
                }
                10 => { // YYYY-GM-DD
                    let year_str = &input[0..4];
                    let gl_str = &input[5..6];
                    let month_str = &input[6..7];
                    let day_str = &input[8..10];
                    year = year_str.parse().map_err(|_| HTParseError::MalformedString)?;
                    month = parse_month_from_gl_and_m(gl_str, month_str)?;
                    day = day_str.parse().map_err(|_| HTParseError::MalformedString)?;
                }
                19 => { // YYYY-GM-DDTNNSNNNNR
                    let year_str = &input[0..4]; // 4 -
                    let gl_str = &input[5..6]; // 1
                    let month_str = &input[6..7]; // 1 -
                    let day_str = &input[8..10]; // 2
                    let sks_str = &input[11..13]; // 2 T
                    let rem_str = &input[14..18]; // 5
                    year = year_str.parse().map_err(|_| HTParseError::MalformedString)?;
                    month = parse_month_from_gl_and_m(gl_str, month_str)?;
                    day = day_str.parse().map_err(|_| HTParseError::MalformedString)?;
                    second = sks_str.parse().map_err(|_| HTParseError::MalformedString)?;
                    second *= 6000;
                    second += rem_str.parse::<u128>().map_err(|_| HTParseError::MalformedString)?;
                }

                _ => {
                    return Err(HTParseError::MalformedString);
                }
            }

            Ok(HTDate {
                year,
                month,
                day,
                second,
            })
        } else { // most likely YYYYGMDD
            let year_str = &input[0..4];
            let gl_str = &input[4..5];
            let month_str = &input[5..6];
            let day_str = &input[6..8];
            year = year_str.parse().map_err(|_| HTParseError::MalformedString)?;
            month = parse_month_from_gl_and_m(gl_str, month_str)?;
            day = day_str.parse().map_err(|_| HTParseError::MalformedString)?;
            if day > 24 {
                return Err(HTParseError::TooManyDays);
            }
            Ok(HTDate {
                year,
                month,
                day,
                second,
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_its_own_strings() {
        let date = HTDate::new(2019, MonthStatus::Greater, Month::Niktvirin, 1, 0);
        let date_str = date.to_string();
        let date2 = HTDate::interpret_string(&date_str).unwrap();
        assert_eq!(date, date2);
    }

    #[test]
    fn accepts_simple_strings() {
        let date = "2019GA01";
        let date2 = HTDate::interpret_string(date).unwrap();
        assert_eq!(date2.year, 2019);
        assert_eq!(date2.month.0, MonthStatus::Greater);
        assert_eq!(date2.month.1, Month::Apress);
        assert_eq!(date2.day, 1);
    }

    #[test]
    fn accepts_simple_strings_with_dashes() {
        let date = "2019-GA-01";
        let date2 = HTDate::interpret_string(date).unwrap();
        assert_eq!(date2.year, 2019);
        assert_eq!(date2.month.0, MonthStatus::Greater);
        assert_eq!(date2.month.1, Month::Apress);
        assert_eq!(date2.day, 1);
    }

    #[test]
    fn accepts_simple_strings_with_dashes_and_time() {
        let date = "2019-GA-01T31S2000R";
        let date2 = HTDate::interpret_string(date).unwrap();
        assert_eq!(date2.year, 2019);
        assert_eq!(date2.month.0, MonthStatus::Greater);
        assert_eq!(date2.month.1, Month::Apress);
        assert_eq!(date2.day, 1);
        assert_eq!(date2.second, 31 * 6000 + 2000);
    }

    #[test]
    fn accepts_simple_strings_with_time() {
        let date = "2019GA01T31S2000R";
        let date2 = HTDate::interpret_string(date).unwrap();
        assert_eq!(date2.year, 2019);
        assert_eq!(date2.month.0, MonthStatus::Greater);
        assert_eq!(date2.month.1, Month::Apress);
        assert_eq!(date2.day, 1);
        assert_eq!(date2.second, 31 * 6000 + 2000);
    }

    #[test]
    fn doesnt_accept_invalid_strings() {
        let date = "2019GA01T31S2000";
        let date2 = HTDate::interpret_string(date);
        assert!(date2.is_err());
    }

    #[test]
    fn doesnt_allow_days_over_24() {
        let date = "2019GA25";
        let date2 = HTDate::interpret_string(date);
        assert!(date2.is_err());
    }
}
