use std::{io, path::PathBuf, rc::Rc};

use serde::ser::{Serialize, SerializeTuple, Serializer};

use crate::models::{Location, Requirement};
