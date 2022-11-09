#![allow(dead_code)]
use std::io::{BufWriter, Write, BufReader, Read};
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::objectid::ObjectId;
use std::fs::{self, *};
use std::ops::Deref;
use crate::crawler;
use crate::Result;

include! {"structs.rs"}
include! {"movie.rs"}
include! {"collect.rs"}