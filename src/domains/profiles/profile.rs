use std::error::Error;

use super::Gig;
use chrono::{Date, Datelike, TimeZone, Utc};

#[derive(Debug)]
pub struct Profile {
    pub name: String,
    pub dream: String,
    pub phone_number: String,
    pub email: String,
    pub video_presentation: String,
    pub favorites: Vec<String>,
    pub gigs: Vec<Gig>,
}

fn first_gig_date(gigs: Vec<Gig>) -> Option<Date<Utc>> {
    let first_gig = gigs.get(0)?;

    let (year, month) = first_gig.start.split_once("-")?;

    let year = year.parse::<i32>().ok()?;
    let month = month.parse::<u32>().ok()?;

    let result = Utc.ymd(year, month, 1);

    Some(result)
}

impl Profile {
    pub fn since(&self) -> i64 {
        let mut gigs = self.gigs.clone();
        gigs.sort_by_key(|g| g.start.clone());

        let now = Utc::now();
        let now = Utc.ymd(now.year(), now.month(), 1);

        let first_gig = first_gig_date(gigs).unwrap_or(now);

        let diff = now.signed_duration_since(first_gig);

        diff.num_days() / 365
    }
}
