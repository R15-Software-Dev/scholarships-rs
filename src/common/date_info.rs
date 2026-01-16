use chrono::{DateTime, FixedOffset, Local};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DateInfo {
    pub id: String,
    pub title: String,
    pub date: DateRange,
    pub description: String,
}

impl DateInfo {
    pub fn get_start_date(&self) -> DateTime<FixedOffset> {
        match self.date {
            DateRange::Single(date) | DateRange::Range(date, _) => date,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DateRange {
    Single(DateTime<FixedOffset>),
    Range(DateTime<FixedOffset>, DateTime<FixedOffset>),
}

impl DateRange {
    pub fn get_status(&self) -> DateStatus {
        match self {
            DateRange::Single(_) => DateStatus::Blank,
            DateRange::Range(start_date, end_date) => {
                let local = Local::now();
                // Check the amount of time from now to the start date. Values that are negative
                // indicate that the start date has already passed.
                if dbg!(start_date.signed_duration_since(local).num_days()) > 0 {
                    DateStatus::Upcoming
                } else {
                    // Check the amount of time from now to the end date. The value will be positive if
                    // the end date is in the future. We will be specific to the minute.
                    let duration_until = dbg!(end_date).signed_duration_since(local);
                    let num_days = dbg!(duration_until.num_days());
                    let num_mins = dbg!(duration_until.num_minutes());

                    if num_mins <= 0 {
                        DateStatus::Closed
                    } else if num_days <= 5 {
                        DateStatus::Deadline
                    } else {
                        DateStatus::Open
                    }
                }
            }
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, PartialOrd, Eq, Ord)]
pub enum DateStatus {
    Closed,
    Deadline,
    Open,
    Upcoming,
    Blank,
}
