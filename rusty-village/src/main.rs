//External crates
extern crate rustc_serialize;
extern crate rand;

//What I'm using from the crates
use serde::{Serialize, Deserialize};
use rand::Rng;
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
    
    //Display state
    fn show_village(village: Village) -> Village{
        println!("The Village of {}'s Current State:", village.name);
        println!();
        println!("Population: {}", village.population);
        println!("Deceased: {}", village.deceased);
        println!("Gold: {}", village.gold);
        println!("Silver: {}", village.silver);
        println!("Copper: {}", village.copper);
        println!("Sick: {}", village.sick);
        println!("Homeless: {}", village.homeless);
        println!("Employed: {}", village.employed);
        println!("Crop Health Level: {}", village.crop_health);
        println!();
        village
    }

    //Choose x number of events and execute them
    fn execute_events(mut village: Village) -> Village{
        let mut rng  = rand::thread_rng();
        let mut number_of_events: i32 = rng.gen_range(0,31);

        while number_of_events > 0{
            let event_number: i32 = rng.gen_range(0, 15);
            let number_of_effect: i32 = rng.gen_range(0, 40);
            println!("The event is: {}", event_number);
            println!();
            match event_number{
                0 => village = Village::members_expired(village, number_of_effect),
                1 => village = Village::members_born(village, number_of_effect),
                2 => village = Village::lost_gold(village, number_of_effect),
                3 => village = Village::gained_gold(village, number_of_effect),
                4 => village = Village::lost_silver(village, number_of_effect),
                5 => village = Village::gained_silver(village, number_of_effect),
                6 => village = Village::lost_copper(village, number_of_effect),
                7 => village = Village::gained_copper(village, number_of_effect),
                8 => village = Village::contracted_illness(village, number_of_effect),
                9 => village = Village::recovered_illness(village, number_of_effect),
                10 => village = Village::lost_home(village, number_of_effect),
                11 => village = Village::gained_home(village, number_of_effect),
                12 => village = Village::lost_job(village, number_of_effect),
                13 => village = Village::gained_job(village, number_of_effect),
                14 => village = Village::crop_health_incr(village, number_of_effect),
                15 => village = Village::crop_health_decr(village, number_of_effect),
                _ => () 
            }
            village = Village::show_village(village);
            number_of_events -= 1;
        }


        village    
    }

    fn members_expired(mut village: Village, number_effected: i32) -> Village{
        if village.population > 0{
            village.deceased += number_effected;
        }

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
        5, 
        0, 
        2, 
        1, 
        7,
        2,
        ); 

 

    village = Village::show_village(village); 
    village = Village::execute_events(village);
    village = Village::show_village(village);
    let encoded = serde_json::to_string(&village).unwrap();

    println!("serialized = {}", encoded);

    let decoded: Village = serde_json::from_str(&encoded).unwrap();

    println!("deserialized = {:?}", decoded);

}
