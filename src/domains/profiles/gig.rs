#[derive(Debug, Clone)]
pub struct Gig {
    pub title: &'static str,
    pub start: &'static str,
    pub duration: &'static str,
    pub description: &'static str,
    pub tags: Vec<&'static str>,
    pub highlights: Vec<&'static str>,
}
