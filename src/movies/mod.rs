#![allow(dead_code)]
use serde::{Deserialize, Serialize, Serializer, Deserializer};
use std::io::{BufWriter, Write, BufReader, Read};
use serde::de::{self, Visitor};
use std::fs::{self, *};
use crate::crawler;
use std::fmt;

include! {"structs.rs"}
include! {"movie.rs"}
include! {"collect.rs"}
include! {"object_id.rs"}
include! {"visitor.rs"}