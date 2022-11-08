#![allow(dead_code)]
use serde::{Deserialize, Serialize, Serializer, Deserializer};
use serde::de::{self, Visitor};
use std::fmt;

include! {"id.rs"}
include! {"visitor.rs"}

#[derive(Default, Clone, Serialize)]
pub struct ObjectId {
    pub id:[u8; 12]
}
struct ObjectIdVisitor;