use std::cell::RefCell;
use std::rc::Rc;
use crate::entity::IdProvider;
use crate::miner::miner::{LocationType, MinerActions};
use crate::miner::quench_thirst::QuenchThirst;
use crate::miner::visit_bank_and_deposit_gold::VisitBankAndDepositGold;
use crate::state::{Entity, State};

pub struct EnterMineAndDigForNugget {
    id_provider: Rc<RefCell<IdProvider>>,
}

impl EnterMineAndDigForNugget {
    pub fn new(id_provider: Rc<RefCell<IdProvider>>) -> Self {
        EnterMineAndDigForNugget{
            id_provider
        }
    }
}

impl State<MinerActions> for EnterMineAndDigForNugget {
    fn on_enter(&self, entity: &mut MinerActions) {
        if entity.location() != LocationType::GoldMine
        {
            let red = "\x1b[31m";
            // ANSI escape code for resetting text color to the default
            let reset = "\x1b[0m";

            let x=self.id_provider.borrow().get_name(&entity.get_id()).to_string();
            let result=format!("{x} : Walking' to the goldmine");
            println!("{red}{result}{reset}");
            entity.change_location(LocationType::GoldMine);
        }
    }

    fn execute(&self, entity: &mut MinerActions) -> Option<Box<dyn State<MinerActions>>> {
        entity.add_to_gold_carried(1);
        entity.increase_fatigue();

        let name=self.id_provider.borrow().get_name(&1).to_string();
        println!("{name} : Picking up a nugger");

        if entity.thirsty() {
            return Some(Box::new(QuenchThirst{ }));
        }

        if entity.pocketsFull(){
            return Some(Box::new(VisitBankAndDepositGold{}));
        }

        return None;
    }

    fn on_exit(&self, entity: &mut MinerActions) {

    }
}