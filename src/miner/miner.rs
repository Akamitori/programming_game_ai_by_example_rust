use std::cell::RefCell;
use std::rc::Rc;
use crate::entity::{EntityIdentity, IdProvider};
use crate::state::Entity;
use crate::state::EntityWrapper;

pub struct MinerActions{
    identity : EntityIdentity,
    location : LocationType,
    gold_carried : i32,
    money_in_bank : i32,
    thirst: i32,
    fatigue : i32,
}


#[derive(PartialEq, Copy, Clone)]
pub enum LocationType {
    GoldMine,
    Bank,
    Saloon,
    HomeSweetHome
}

impl MinerActions{
    const THRIST_MAX: i32=5;
    const GOLD_TO_BE_COMFORTABLE: i32       = 5;
    const MAX_NUGGETS:i32     = 3;
    const FATIQUE_MAX:i32 = 5;


    pub fn location(&self)-> LocationType {
        return self.location
    }

    pub fn change_location(&mut self, location : LocationType) {
        self.location=location;
    }

    pub fn add_to_gold_carried(&mut self, gold_to_add:i32){
        self.gold_carried+=gold_to_add;
    }

    pub fn increase_fatigue(&mut self){
        self.fatigue+=1;
    }

    pub fn thirsty(&self) -> bool {
        return self.thirst >= MinerActions::THRIST_MAX;
    }

    pub fn pocketsFull(&self) -> bool {
        return self.gold_carried >= MinerActions::MAX_NUGGETS;
    }
}

impl Entity for MinerActions {
    fn new(identity : EntityIdentity ) -> Self {
        MinerActions {
            identity ,
            location: LocationType::HomeSweetHome,
            gold_carried: 0,
            money_in_bank: 0,
            thirst: 0,
            fatigue: 0,
        }
    }

    fn update(&mut self) {
        self.thirst += 1;
    }

    fn get_id(&self) -> u32 {
        return self.identity.id()
    }
}