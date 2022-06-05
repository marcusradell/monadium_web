use crate::domains::profiles::{Gig, Profile};

pub fn profile() -> Profile {
    Profile {
        name: "Marcus Rådell".into(),
        dream: "Build healthy tech teams, mob program, and program in Rust.".into(),
        phone_number: "+46725223325".into(),
        email: "marcus@radell.net".into(),
        video_presentation: "https://player.vimeo.com/video/613787819?h=10e4fcd6ed".into(),
        favorites: vec![
            "RUST".into(),
            "TDD".into(),
            "EVENT_SOURCING".into(),
            "COACHING".into(),
            "TYPESCRIPT".into(),
        ],
        gigs: vec![Gig{
        title: "Monadium.org".into(),
              start: "2021-07".into(),
              duration: "ONGOING".into(),
              description: "I founded a platform for programmers and tech teams to collaborate and improve.".into(),
              tags: vec![
                "GCP".into(),
                "KUBERNETES".into(),"RUST".into()
        ,"JAVASCRIPT".into()
        ,"ELEVENTY".into()
        ,"GITHUB_ACTIONS".into()
        ,"CI_CD".into()
        ,"GCP".into()
        ,"POSTGRES".into()
        ,"MENTORING".into()
        ,"EVENT_SOURCING".into()
              ],
              highlights: vec![
                "Setup a Kubernetes cluster in GCP.".into()
                ,"Build an authentication & authorization setup in Rust.".into()
                ,"Use an event sourced database setup via Postgres and Rust’s sqlx.".into()
                ,"Write highly modular code with reusable services and business logic that is simple to extract out to their own services.".into()
                ,"Conduct user research by interviewing companies and programmers on their recruitment needs.".into()
                ,"Mentoring junior developers on how to build a great portfolio to showcase their skills by using a project board, writing tests, and documenting properly.".into()
              ]
                      }],
    }
}
