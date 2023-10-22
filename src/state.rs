use std::cell::RefCell;
use std::rc::Rc;
use crate::entity::{EntityIdentity, IdProvider};

pub trait Entity {
    fn new(identity : EntityIdentity) -> Self;
    fn update(&mut self);

    fn get_id(&self) -> u32;
}

pub trait State<T: Entity> {
    fn on_enter(&self, entity: &mut T);
    fn execute(&self, entity: &mut T) -> Option<Box<dyn State<T>>>;
    fn on_exit(&self, entity: &mut T);
}

pub struct EntityWrapper<T: Entity> {
    entity: T,
    state: Box<dyn State<T>>,
}


impl<T: Entity> EntityWrapper<T> {
    pub fn new(id_provider : Rc<RefCell<IdProvider>>, label : &str , initial_state: Box<dyn State<T>>) -> Self {
        let identity= EntityIdentity::new(id_provider,label);
        let mut result=EntityWrapper::<T> {
            entity: T::new(identity),
            state: initial_state,
        };

        result.state.on_enter(&mut result.entity);
        return result;
    }

    pub fn update(&mut self) {
        self.entity.update();
        let result = self.state.execute(&mut self.entity);
        self.change_state(result);
    }

    pub fn change_state(&mut self, state_result: Option<Box<dyn State<T>>>) {
        match state_result {
            None => {}
            Some(new_state) => {
                self.state.on_exit(&mut self.entity);
                self.state = new_state;
                self.state.on_enter(&mut self.entity);
            }
        }
    }
}