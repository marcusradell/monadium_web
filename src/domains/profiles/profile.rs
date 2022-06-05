use super::Gig;

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
