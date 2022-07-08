use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::near_bindgen;
use near_sdk::serde::*;
use near_sdk::env;

#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize, Debug, Serialize, Deserialize)]
#[serde(crate="near_sdk::serde")]
pub struct Buses {
    // Struct to hold info on buses.
    registration_no : String,
    route : String,
    bus_capacity : i8,
    all_seats : Vec<i8>,
    bus_status: String,
}

#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct Contract{
    // set up contract struct
    buz : Vec<Buses>,
}

#[near_bindgen]

// #[derive(ToPrimitive)]
impl Contract {
    // ADD CONTRACT METHODS HERE
    // function to add a new vector bus.
    pub fn new_bus() -> Self{
        let buz = Vec::new();
        Contract{
            buz
        }
    }
    // function to count the vector of Buses
    pub fn bus_count(&mut self) -> usize{
        self.buz.len()
    }

    // adding information held at the Buses Struct
    pub fn add_bus(&mut self, registration_no: String,
        route: String,
        bus_capacity: i8,
        all_seats: Vec<i8>,
        booked_seat: String){
            let buz1 = Buses{
                registration_no: registration_no.to_string(),
                route: route.to_string(),
                bus_capacity: bus_capacity,
                all_seats: all_seats.clone(),
                bus_status: booked_seat.to_string(),
            };
            self.buz.push(buz1);
            env::log_str("Bus added");
    }

    // display data from the vector Buses
    pub fn show_bus(&mut self) -> &Vec<Buses>{
        &self.buz
    }
    // delete a bus from the vector
    pub fn delete_bus(&mut self){
        self.buz.pop();
        env::log_str("Bus deleted");
    }
    
}

/*
 * the rest of this file sets up unit tests
 * to run these, the command will be:
 * cargo test --package rust-template -- --nocapture
 * Note: 'rust-template' comes from Cargo.toml's 'name' key
 */

// use the attribute below for unit tests
#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::test_utils::{VMContextBuilder};
    use near_sdk::{AccountId};

    // part of writing unit tests is setting up a mock context
    // provide a `predecessor` here, it'll modify the default context
    fn get_context(predecessor: AccountId) -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder.predecessor_account_id(predecessor);
        builder
    }

    // TESTS HERE
    #[test]
    // test checking the existence of data in the vector
    fn bus_existence(){
        let user = AccountId::new_unchecked("mashkariz_charles.testnet".to_string());
        let _context = get_context(user.clone());
        let mut buz = Contract::new_bus();
        buz.add_bus("KBB 012A".to_string(), "Mombasa-Nairobi".to_string(), 62, (1..62).collect::<Vec<i8>>(),  "Active".to_string());
        let counting = buz.bus_count();
        assert_eq!(counting, 1);
    }
    #[test]
    //test adding data to vector.
    fn add_bus(){
        let user = AccountId::new_unchecked("mashkariz_charles.testnet".to_string());
        let _context = get_context(user.clone());

        let mut buz = Contract::new_bus();
        
        buz.add_bus("KCA 452Z".to_string(), "Mombasa-Nairobi".to_string(), 62, (1..62).collect::<Vec<i8>>(), "Booked".to_string());
        buz.add_bus("KDH 789D".to_string(), "Kisumu-Siaya".to_string(), 30,  (1..30).collect::<Vec<i8>>(), "Active".to_string());
        buz.add_bus("KBB 012A".to_string(), "Nakuru-Nairobi".to_string(), 20,  (1..20).collect::<Vec<i8>>(), "Active".to_string());
        let counts = buz.bus_count();
        assert_eq!(counts, 3);
    }

    // Test for getting data from vector
    #[test]
    fn get_bus(){
        let user = AccountId::new_unchecked("mashkariz_charles.testnet".to_string());
        let _context = get_context(user.clone());
        let mut buz = Contract::new_bus();
        buz.add_bus("KBB 012A".to_string(), "Mombasa-Nairobi".to_string(), 62,  (1..62).collect::<Vec<i8>>(), "Active".to_string());
        let counts = buz.show_bus();
        assert_eq!(counts.len(), 1);
    }

    // test for the delete of job from vector
    #[test]
    fn delete_bus(){
        let user = AccountId::new_unchecked("mashkariz_charles.testnet".to_string());
        let _context = get_context(user.clone());
        let mut buz = Contract::new_bus();
        buz.add_bus("KBB 012A".to_string(), "Mombasa-Nairobi".to_string(), 62,  (1..62).collect::<Vec<i8>>(), "Active".to_string());
        buz.delete_bus();
        let counts = buz.show_bus();
        assert_eq!(counts.len(), 0);
    }
}
