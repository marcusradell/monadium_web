#[derive(Debug, Clone)]
pub struct Gig {
    pub title: String,
    pub start: String,
    pub duration: String,
    pub description: String,
    pub tags: Vec<String>,
    pub highlights: Vec<String>,
}
