#![allow(dead_code)]
use serde::{Deserialize, Serialize, Serializer, Deserializer};
use serde::de::{self, Visitor};
use std::fs::{self, *};
use std::io::{BufWriter, Write};
use std::fmt;
use crate::crawler;

include! {"structs.rs"}
include! {"movie.rs"}
include! {"collect.rs"}
include! {"object_id.rs"}
include! {"visitor.rs"}