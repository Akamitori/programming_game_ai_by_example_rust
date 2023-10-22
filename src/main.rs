//use std::cell::RefCell;
//use game_dev_experiment::state2::Troll;


use game_dev_experiment::state::EntityWrapper;
use game_dev_experiment::entity::IdProvider;
use game_dev_experiment::miner::miner:: MinerActions;

fn main() {
    let mut id_provider=Rc::new(RefCell::new(IdProvider::new()));


    let mut z: EntityWrapper<MinerActions>= EntityWrapper::<MinerActions>::new(
      id_provider.clone(),"Bob", Box::new(EnterMineAndDigForNugget::new(id_provider))
    );

    z.update();



    // let x= enter_mine_and_dig_for_nugget::EnterMineAndDigForNugget::new(id_provider.clone());
    // let y=enter_mine_and_dig_for_nugget::EnterMineAndDigForNugget::new(id_provider.clone());


    println!("Hello, world!");

    //let mut x= EntityWrapper::new();

    //x.update();
    //x.update();

    //let mut x= Troll::new();
    //x.update();
    //x.update();

}

struct x{

}


/////////////////////

use std::cell::RefCell;
use std::rc::Rc;
use game_dev_experiment::miner::enter_mine_and_dig_for_nugget::EnterMineAndDigForNugget;

// Define a struct representing a person
#[derive(Debug)]
struct Person {
    name: String,
    age: u32,
}

impl Person {
    fn new(name: &str, age: u32) -> Self {
        Person {
            name: name.to_string(),
            age,
        }
    }

    fn change(&mut self, age: u32){
        self.age=age;
    }
}

// Define another struct that contains an Rc to Person
// #[derive(Debug)]
// struct PersonContainer {
//     person: Rc<RefCell<Person>>,
// }
//
// fn main() {
//     // Create shared data and wrap it in an Rc
//     let mut x=Person::new("Alice", 25);
//     let shared_person = Rc::new(RefCell::new(x));
//
//     // Create multiple struct owners that contain Rc<Person>
//     let person_container1 = PersonContainer {
//         person: Rc::clone(&shared_person),
//     };
//     let person_container2 = PersonContainer {
//         person: Rc::clone(&shared_person),
//     };
//
//
//     // Print the data from multiple struct owners
//     println!("PersonContainer 1: {:?}", person_container1);
//     shared_person.borrow_mut().change(5);
//     println!("PersonContainer 2: {:?}", person_container2);
//     println!("PersonContainer 1: {:?}", person_container1);
// }
