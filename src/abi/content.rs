use fake::{Fake, Faker};

use crate::pb::social::Content;

impl Content {
    pub fn fake() -> Self {
        Self {
            text: Faker.fake::<String>(),
            images: vec![],
            videos: vec![],
        }
    }
}
