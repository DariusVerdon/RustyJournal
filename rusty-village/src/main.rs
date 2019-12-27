//External crates
extern crate rustc_serialize;

//What I'm using from the crates
use serde::{Serialize, Deserialize};

//The struct I will be manipulating. The derive flag is set so the data can be written to a file
//easier as Serialized data, and then Deserialized when I need to use it in the program.
#[derive(Serialize, Deserialize, Debug)]
struct Village {
    name: String,
    population: i32,
    deceased: i32,
    gold: i32,
    silver: i32,
    copper: i32,
    sick: i32,
    homeless: i32,
    employed: i32,
    crop_health: i32,
}

//implementations of my struct
impl Village {
    fn create_village(new_name: String, new_pop: i32, new_deceased: i32, new_gold: i32, new_silver: i32, new_copper: i32, new_sick: i32, new_homeless: i32, new_employed: i32, new_crop_health: i32) -> Village{
        Village {
            name: new_name,
            population: new_pop,
            deceased: new_deceased,
            gold: new_gold,
            silver: new_silver,
            copper: new_copper,
            sick: new_sick,
            homeless: new_homeless,
            employed: new_employed,
            crop_health: new_crop_health
        }
    }
    
    //SAVE VILLAGE TO FILE
    
    //LOAD VILLAGE FROM FILE

    fn members_expired(mut village: Village, number_effected: i32) -> Village{
        village.deceased += number_effected;
        if (village.population - number_effected) < 0{
            village.population = 0;
        }
        else{
            village.population -= number_effected;
        }
        village
    }
    
    fn members_born(mut village: Village, number_incr: i32) -> Village{
        village.population += number_incr;
        village
    }
    
    fn lost_gold(mut village: Village, number_decr: i32) -> Village{
        if (village.gold - number_decr) < 0{
            village.gold = 0;
        }
        else{
            village.gold -= number_decr;
        }
        village
    }

    fn gained_gold(mut village: Village, number_incr: i32) -> Village{
        village.gold += number_incr;
        village
    }

    fn lost_silver(mut village: Village, number_decr: i32) -> Village{
        if (village.silver - number_decr) < 0{
            village.silver = 0;
        }
        else{
            village.silver -= number_decr;
        }
        village
    }

    fn gained_silver(mut village: Village, number_incr: i32) -> Village{
        village.silver += number_incr;
        village
    }

    fn lost_copper(mut village: Village, number_decr: i32) -> Village{
        if (village.copper - number_decr) < 0{
            village.copper = 0;
        }
        else{
            village.copper -= number_decr;
        }
        village
    }

    fn gained_copper(mut village: Village, number_incr: i32) -> Village{
        village.copper += number_incr;
        village
    }

    fn contracted_illness(mut village: Village, number_incr: i32) -> Village{
        village.sick += number_incr;
        village
    }

    fn recovered_illness(mut village: Village, number_decr: i32) -> Village{
        if (village.sick - number_decr) < 0 {
            village.sick = 0;
        }
        else{
            village.sick -= number_decr;
        }
        village
    }
    
    fn lost_home(mut village: Village, number_incr: i32) -> Village{
        village.homeless += number_incr;
        village
    }

    fn gained_home(mut village: Village, number_decr: i32) -> Village{
        if (village.homeless - number_decr) < 0{
            village.homeless = 0;
        }
        else{
            village.homeless -= number_decr;
        }
        village
    }

    fn lost_job(mut village: Village, number_decr: i32) -> Village{
        if (village.employed - number_decr) < 0{
            village.employed = 0;
        }
        else{
            village.employed -= number_decr;
        }
        village
    }

    fn gained_job(mut village: Village, number_incr: i32) -> Village{
        village.employed += number_incr;
        village
    }

    fn crop_health_incr(mut village: Village, number_incr: i32) -> Village{
        village.crop_health += number_incr;
        village
    }

    fn crop_health_decr(mut village: Village, number_decr: i32) -> Village{
        if (village.crop_health - number_decr) < 0{
            village.crop_health = 0;
        }
        else{
            village.crop_health -= number_decr;
        }
        village
    }
}

fn main() {
    //creates a village so I can play around with data
    let mut village: Village = Village::create_village(
        "Elora".to_string(),
        12, 
        4, 
        2, 
        54, 
        0, 
        2, 
        1, 
        7,
        2,
        ); 

    println!("{}", village.name);
    println!("Current Employed: {}", village.employed);
    println!("Oh no, someone lost a job!");
 
    village = Village::lost_job(village, 7);

    println!("Current Employed: {}", village.employed);

    let encoded = serde_json::to_string(&village).unwrap();

    println!("serialized = {}", encoded);

    let decoded: Village = serde_json::from_str(&encoded).unwrap();

    println!("deserialized = {:?}", decoded);

}
