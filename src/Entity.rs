use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use crate::state::Entity;

pub struct IdProvider {
    next_valid_id: u32,
    entities: HashMap<u32, String>,
}

impl IdProvider {
    pub fn new() -> Self {
        IdProvider {
            next_valid_id: 1,
            entities: HashMap::new(),
        }
    }

    fn increase_next_valid_id(&mut self) {
        self.next_valid_id += 1;
    }

    fn get_next_valid_id(&self) -> u32 {
        return self.next_valid_id;
    }

    pub fn get_name(&self, id: &u32) -> &str {
        return &self.entities[id];
    }

    pub fn generate_id_for_label(&mut self, label: &str) -> u32 {
        let id = self.get_next_valid_id();
        self.entities.insert(id, label.to_string());
        self.increase_next_valid_id();
        return id;
    }
}

pub struct EntityIdentity {
    id: u32,
}

impl EntityIdentity {
    pub fn new(id_provider: Rc<RefCell<IdProvider>>, label: &str) -> Self {
        let id = id_provider.borrow_mut().generate_id_for_label(label);
        EntityIdentity {
            id,
        }
    }

    pub fn id(&self) -> u32 {
        self.id
    }
}
