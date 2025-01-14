use crate::data::context::Context;
use crate::data::Event;
use crate::data::{ast::*, Literal};

use curl::easy;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Data<'a> {
    pub ast: &'a Flow,
    pub context: &'a mut Context,
    pub event: &'a Event,
    pub curl: easy::Easy,
    pub step_vars: HashMap<String, Literal>,
}
