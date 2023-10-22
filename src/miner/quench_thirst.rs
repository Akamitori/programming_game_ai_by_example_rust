use crate::miner::miner::MinerActions;
use crate::state::State;

pub struct QuenchThirst;

impl State<MinerActions> for QuenchThirst {
    fn on_enter(&self, entity: &mut MinerActions) {
        todo!()
    }

    fn execute(&self, entity: &mut MinerActions) -> Option<Box<dyn State<MinerActions>>> {
        todo!()
    }

    fn on_exit(&self, entity: &mut MinerActions) {
        todo!()
    }
}