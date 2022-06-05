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

impl Profile {
    pub fn since(&self) -> i64 {
        let mut gigs = self.gigs.clone();
        gigs.sort_by_key(|g| g.start.clone());

        let now = Utc::now();
        let now = Utc.ymd(now.year(), now.month(), 1);
        let now_year = now.year().to_string();
        let now_month = now.month().to_string();

        let first_gig: Date<Utc> = gigs.get(0).map_or(now, |g| {
            let (year, month) = g.start.split_once("-").unwrap_or((&now_year, &now_month));
            let year: i32 = year.parse().unwrap_or(now.year());
            let month: u32 = month.parse().unwrap_or(now.month());
            Utc.ymd(year, month, 1)
        });

        let diff = now.signed_duration_since(first_gig);

        diff.num_days() / 365
    }
}
