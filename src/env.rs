use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::object::*;

#[derive(Debug, Default, PartialEq)]
pub struct Env {
    parent: Option<Rc<RefCell<Env>>>,
    vars: HashMap<String, Object>,
}

impl Env {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn extend(parent: Rc<RefCell<Env>>) -> Self {
        Env {
            parent: Some(parent),
            vars: Default::default(),
        }
    }

    pub fn set(&mut self, name: String, value: Object) {
        self.vars.insert(name, value);
    }

    pub fn get(&self, name: &str) -> Option<Object> {
        match self.vars.get(name) {
            Some(value) => Some(value.clone()),
            None => self
                .parent
                .as_ref() // as_ref() 经常与 map, and_then()配合使用，避免move, 有点类似match时的应用操作
                .and_then(|parent| parent.borrow().get(name)),
        }
    }
}
