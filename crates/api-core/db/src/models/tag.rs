use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct Tag {
    pub tag_id: i32,
    pub tagname: String,
}
