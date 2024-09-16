use crate::pb::social::User;
use fake::faker::internet::en::SafeEmail;
use fake::faker::name::en::Name;
use fake::Fake;
use fake::Faker;
use uuid::Uuid;

use super::now_ts;

impl User {
    pub fn fake() -> Self {
        Self {
            user_id: Uuid::new_v4().to_string(),
            nick_name: Name().fake(),
            user_name: Name().fake(),
            avatar: Faker.fake::<String>(),
            bio: Faker.fake::<String>(),
            email: SafeEmail().fake(),
            web_site: Faker.fake::<String>(),
            birthday: Faker.fake::<String>(),
            created_at: Some(now_ts()),
            updated_at: Some(now_ts()),
        }
    }
}
