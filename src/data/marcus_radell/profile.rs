use crate::domains::profiles::{Gig, Profile};

pub fn profile() -> Profile {
    Profile {
        name: "Marcus Rådell",
        dream: "Build healthy tech teams, mob program, and program in Rust.",
        phone_number: "+46725223325",
        email: "marcus@radell.net",
        video_presentation: "https://player.vimeo.com/video/613787819?h=10e4fcd6ed",
        favorites: vec![
            "RUST",
            "TDD",
            "EVENT_SOURCING",
            "COACHING",
            "TYPESCRIPT",
        ],
        gigs: vec![
          Gig{
        title: "Monadium.org",
              start: "2021-07",
              duration: "ONGOING",
              description: "I founded a platform for programmers and tech teams to collaborate and improve.",
              tags: vec![
                "GCP",
                "KUBERNETES","RUST"
        ,"JAVASCRIPT"
        ,"ELEVENTY"
        ,"GITHUB_ACTIONS"
        ,"CI_CD"
        ,"GCP"
        ,"POSTGRES"
        ,"MENTORING"
        ,"EVENT_SOURCING"
              ],
              highlights: vec![
                "Setup a Kubernetes cluster in GCP."
                ,"Build an authentication & authorization setup in Rust."
                ,"Use an event sourced database setup via Postgres and Rust’s sqlx."
                ,"Write highly modular code with reusable services and business logic that is simple to extract out to their own services."
                ,"Conduct user research by interviewing companies and programmers on their recruitment needs."
                ,"Mentoring junior developers on how to build a great portfolio to showcase their skills by using a project board, writing tests, and documenting properly."
              ]
                      },
                      Gig{
                        title: "CAG Mälardalen",
                        start: "2008-01",
                        duration: "6",
                        description: "Java e-commerce mixed with learning PHP.",
                        tags:
                          vec!["JAVA",
                          "PHP"],
                          highlights: vec![]
                      }],
    }
}
