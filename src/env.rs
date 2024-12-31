use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::object::*;

#[derive(Debug, Default)]
pub struct Env {
    parent: Option<Rc<RefCell<Env>>>,
    vars: HashMap<String, Object>,
}

impl Env {
    pub fn new() -> Self {
        Default::default()
    }
}
