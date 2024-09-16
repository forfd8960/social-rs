use uuid::Uuid;

use crate::pb::social::{Content, Post, User};

use super::now_ts;

impl Post {
    pub fn fake() -> Self {
        Self {
            post_id: Uuid::new_v4().to_string(),
            user: Some(User::fake()),
            content: Some(Content::fake()),
            created_at: Some(now_ts()),
            updated_at: Some(now_ts()),
        }
    }
}
