use near_sdk::*;
use num_traits::cast::ToPrimitive;
use windows::*;


#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize, Debug, Serialize, Deserialize)]
#[serde(crate="near_sdk::serde")]
pub struct Buses {
    // SETUP CONTRACT STATE
    registration_no : String,
    route : String,
    bus_capacity : i8,
    booked_seat: bool,
}

#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct Contract{
    buz : Vec<Buses>,
}

#[near_bindgen]
// #[derive(ToPrimitive)]
impl Contract {
    // ADD CONTRACT METHODS HERE
    fn new_bus() -> Self{
        let buz = Vec::new();
        Contract{
            buz
        }
    }
    pub fn bus_count(&mut self) -> usize{
        self.buz.len()
    }
    pub fn add_bus(&mut self, registration_no: String,
        route: String,
        bus_capacity: i8,
        booked_seat: bool){
            let buz1 = Buses{
                registration_no: registration_no.to_string(),
                route: route.to_string(),
                bus_capacity: bus_capacity.to_i8(),
                booked_seat: booked_seat.as_bool(),
            };
            self.buz.push(buz1);
            env::log_str("Bus added");
        }

    pub fn show_bus(&mut self) -> &Vec<Buses>{
        &self.buz
    }
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
    use near_sdk::test_utils::{get_logs, VMContextBuilder};
    use near_sdk::{testing_env, AccountId};

    // part of writing unit tests is setting up a mock context
    // provide a `predecessor` here, it'll modify the default context
    fn get_context(predecessor: AccountId) -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder.predecessor_account_id(predecessor);
        builder
    }

    // TESTS HERE
}
