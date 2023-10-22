use std::cell::RefCell;
use std::rc::Rc;
use crate::entity::IdProvider;
use crate::miner::miner::{LocationType, MinerActions};
use crate::miner::quench_thirst::QuenchThirst;
use crate::state::State;

pub struct VisitBankAndDepositGold {

}


impl State<MinerActions> for VisitBankAndDepositGold {
    fn on_enter(&self, entity: &mut MinerActions) {
        todo!()
    }

    fn execute(&self, entity: &mut MinerActions) -> Option<Box<dyn State<MinerActions>>> {
        todo!()
    }

    fn on_exit(&self, entity: &mut MinerActions){
        todo!()
    }
}