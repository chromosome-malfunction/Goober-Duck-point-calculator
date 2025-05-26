//I am better than you [I am the stigma]

use std::collections::HashMap;
use std::io;

fn main() {
    let mut duckmap: HashMap<String, u32> = HashMap::new();
    let mut goobermap: HashMap<String, u32> = HashMap::new();

    println!("Hello, world!");
    menu(&mut duckmap, &mut goobermap);
}

fn menu(duckmap: &mut HashMap<String, u32>, goobermap: &mut HashMap<String, u32>) {
    loop {
        let result = prompt("Duck or Goober point system? (Duck/Goober or quit)").trim().to_string();

        match result.to_lowercase().as_str() {
            "duck" => { pointcalculations(duckmap, goobermap, "duck"); },
            "goober" => { pointcalculations(duckmap, goobermap, "goober"); },
            "quit" => { std::process::exit(0); },
            _ => { println!("Unknown command."); continue; },
        }
    }
}

fn prompt(prompt: &str) -> String {
    println!("{}", prompt);

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line.");
    input.to_string();
    input
}

fn user(duckmap: &mut HashMap<String, u32>, goobermap: &mut HashMap<String, u32>, dg: &str) -> String {
    loop {
        let user: String = prompt("What user? (all or quit)").trim().to_string();

        if dg == "duck" {
            if user == "all" {
                let result = format!("{:?}", duckmap);
                println!("{}", result);
                continue;
            }
            if user == "quit" {
                menu(duckmap, goobermap);
            }
            if !duckmap.contains_key(&user) {
                duckmap.insert(user.clone(), 0);
            }
        } else if dg == "goober" {
            if user == "all" {
                let result = format!("{:?}", goobermap);
                println!("{}", result);
                continue;
            }
            if user == "quit" {
                menu(duckmap, goobermap);
            }
            if !goobermap.contains_key(&user) {
                goobermap.insert(user.clone(), 0);
            }
        }

        return user;
    }
}

fn normal(prompt: &str, user: String, prompt2: &str) -> u32 {
    loop {
        let question = format!("How many {} did {} {}?", prompt, user, prompt2);
        match crate::prompt(&question).trim().parse() {
            Ok(amount) => return amount,
            Err(_) => continue
        }
    }
}
fn pointcalculations(duckmap: &mut HashMap<String, u32>, goobermap: &mut HashMap<String, u32>, dg: &str) {
    loop {
        let user: String = user(duckmap, goobermap, dg);
        loop {
            let question = format!("What point value would you like to add to {}? (define/points/CU/[(R/MoP) shared]/[(RB/CTO/PH/PD/PEO/MMA) duck]/[(EB/LR/CR/N) goober])", user);

            match prompt(&question.trim()).trim().to_lowercase().as_str() {
                "define" => { println!("\n 'define' is to define all abbreviations \n 'points' is to see the users points \n 'CU' is Change User \n 'R' is Raiders killed \n 'RB' is Rule Breakers killed \n 'CTO' is Corpses Thrown Out \n 'PH' is People Healed \n 'PD' is People Defibrillated \n 'PEO' is People Experimented On \n 'MoP' is Minutes of Playtime \n 'MMA' is Minutes Managing Airlock \n 'EB' is Employee Bonus \n 'LR' is levers rest \n 'CR' is people let in through the airlock \n 'N' is nuisances killed \n ('RB', 'CTO', 'PH', 'PD', 'PEO', 'MMA') are exclusive to Duck SafeHouse \n ('EB', 'LR', 'CR') are exclusive to Goober SafeHouse \n ('R' 'MoP') are shared by both. \n"); },
                "r" => {
                    let result = raider(duckmap, goobermap, user.clone(), normal("raiders", user.clone(), "kill"), dg);
                    println!("{}", result);
                },

                "rb" => {
                    let result = rulebreaker(duckmap, user.clone(), normal("rule breakers", user.clone(), "kill"), "rule breaker killed".to_string(), dg);
                    println!("{}", result);
                },
                "cto" => {
                    let result = corpse(duckmap, user.clone(), normal("corpses", user.clone(), "throw out"), "thrown out corpses".to_string(), dg);
                    println!("{}", result);
                },
                "ph" | "pd" => {
                    let result = corpse(duckmap, user.clone(), normal("people", user.clone(), "heal/defibrillate"), "people healed/defibrillated".to_string(), dg);
                    println!("{}", result);
                },
                "peo" => {
                    let result = rulebreaker(duckmap, user.clone(), normal("people", user.clone(), "experiment on"), "people experimented on".to_string(), dg);
                    println!("{}", result);
                },
                "mop" => {
                    let result = playtime(duckmap, goobermap, user.clone(), normal("minutes", user.clone(), "play in the session"), dg);
                    println!("{}", result);
                },
                "mma" => {
                    let result = airlock(duckmap, user.clone(), normal("minutes", user.clone(), "manage the airlock"), dg);
                    println!("{}", result);
                },
                "cu" => {break;},
                "points" => { println!("{}", points(duckmap, goobermap, user.clone(), dg)) },
                "eb" => {
                    let result = bonus(goobermap, user.clone(), normal("points", user.clone(), "as a bonus"), dg);
                    println!("{}", result);
                },
                "lr" => {
                    let result = lever(goobermap, user.clone(), normal("levers", user.clone(), "rest"), dg);
                    println!("{}", result);
                },
                "cr" => {
                    let result = cr(goobermap, user.clone(), normal("people", user.clone(), "let in through the airlock"), dg);
                    println!("{}", result);
                },
                "n" => {
                    let result = nuisances(goobermap, user.clone(), normal("nuisances", user.clone(), "kill"), dg);
                    println!("{}", result);
                }
                _ => {
                    println!("Invalid command. Type 'define' to check all commands.");
                    continue;
                }
            }
        }
    }
}

fn points(duckmap: &mut HashMap<String, u32>, goobermap: &mut HashMap<String, u32>, user: String, dg: &str) -> String {
    if dg == "duck" {
        match duckmap.get(&user) {
            Some(value) => {
                format!("{} has {} points in {}", user, value, dg.to_string())
            }

            None => { "user not found in map".to_string()}
        }
    } else if dg == "goober" {
        match goobermap.get(&user) {
            Some(value) => {
                format!("{} has {} points in {}", user, value, dg.to_string())
            }

            None => { "user not found in map".to_string()}
        }
    } else {
        format!("This command is not available to {}", dg)
    }
}
fn raider(duckmap: &mut HashMap<String, u32>, goobermap: &mut HashMap<String, u32>, user: String, amount: u32, dg: &str) -> String {
    if dg == "duck" {
        match duckmap.get_mut(&user) {
            Some(value) => {
                *value += 4 * amount;

                format!("{} updated by {} points, because of {} raider kills.", user, 4 * amount, amount)
            }

            None => { "user not found in map".to_string()}
        }
    } else if dg == "goober" {
        match goobermap.get_mut(&user) {
            Some(value) => {
                *value += 5 * amount;

                format!("{} updated in {} by {} points, because of {} raider kills.", user, dg, 5 * amount, amount)
            }

            None => { "user not found in map".to_string()}
        }
    } else {
        format!("This point value is not available for {}", dg)
    }
}

fn rulebreaker(duckmap: &mut HashMap<String, u32>, user: String, amount: u32, reason: String, dg: &str) -> String {
    if dg == "duck" {
        match duckmap.get_mut(&user) {
            Some(value) => {
                *value += 2 * amount;

                format!("{} updated by {} points, because of {} {}.", user, 2 * amount, amount, reason)
            }

            None => { "user not found in map".to_string()}
        }
    } else {
         format!("This point value is not available for {}", dg)
    }
}

fn corpse(duckmap: &mut HashMap<String, u32>, user: String, mut amount: u32, reason: String, dg: &str) -> String {
    if dg == "duck" {
        if amount >= 10 { amount = 10; }
        match duckmap.get_mut(&user) {
            Some(value) => {
                *value += amount;

                format!("{} updated by {} points, because of {} {}.", user, amount, amount, reason)
            }

            None => { "user not found in map".to_string()}
        }
    } else {
        format!("This point value is not available for {}", dg)
    }
}

fn playtime(duckmap: &mut HashMap<String, u32>, goobermap: &mut HashMap<String, u32>, user: String, amount: u32, dg: &str) -> String {
    if dg == "duck" {
        match duckmap.get_mut(&user) {
            Some(value) => {
                let time: u32 = (0.1667 * amount as f64).round() as u32;
                *value += time;

                format!("{} updated by {} points, because of {} minutes played in session.", user, time, amount)
            }

            None => { "user not found in map".to_string() }
        }
    } else if  dg == "goober" {
        match goobermap.get_mut(&user) {
            Some(value) => {
                let time: u32 = (amount/60)*10;
                *value += time;

                format!("{} updated by {} points, because of {} minutes played in session.", user, time, amount)
            }

            None => { "user not found in map".to_string() }
        }
    } else {
        format!("This point value is not available for {}", dg)
    }
}

fn airlock(duckmap: &mut HashMap<String, u32>, user: String, amount: u32, dg: &str) -> String {
    if dg == "duck" {
        match duckmap.get_mut(&user) {
            Some(value) => {
                let time: u32 = (0.25 * amount as f64).round() as u32;
                *value += time;

                format!("{} updated by {} points, because of {} minutes managing the airlock.", user, time, amount)
            }

            None => { "user not found in map".to_string()}
        }
    } else {
        format!("This point value is not available for {}", dg)
    }
}

fn bonus(goobermap: &mut HashMap<String, u32>, user: String, amount: u32, dg: &str) -> String {
    if dg == "goober" {
        match goobermap.get_mut(&user) {
            Some(value) => {
                *value += amount;

                format!("{} updated by {} points, because of a {} point bonus.", user, amount, amount)
            }

            None => { "user not found in map".to_string()}
        }
    } else {
        format!("This point value is not available for {}", dg)
    }
}

fn lever(goobermap: &mut HashMap<String, u32>, user: String, amount: u32, dg: &str) -> String {
    if dg == "goober" {
        match goobermap.get_mut(&user) {
            Some(value) => {
                *value += 3*amount;

                format!("{} updated by {} points, because {} rest {} levers.", user, 3*amount, user, amount)
            }

            None => { "user not found in map".to_string()}
        }
    } else {
        format!("This point value is not available for {}", dg)
    }
}

fn cr(goobermap: &mut HashMap<String, u32>, user: String, amount: u32, dg: &str) -> String {
    if dg == "goober" {
        match goobermap.get_mut(&user) {
            Some(value) => {
                *value += amount;

                format!("{} updated by {} points, because {} let in {} people.", user, amount, user, amount)
            }

            None => { "user not found in map".to_string()}
        }
    } else {
        format!("This point value is not available for {}", dg)
    }
}

fn nuisances(goobermap: &mut HashMap<String, u32>, user: String, amount: u32, dg: &str) -> String {
    if dg == "goober" {
        match goobermap.get_mut(&user) {
            Some(value) => {
                *value += 3*amount;

                format!("{} updated by {} points, because {} killed {} nuisances.", user, 3*amount, user, amount)
            }

            None => { "user not found in map".to_string()}
        }
    } else {
        format!("This point value is not available for {}", dg)
    }
}
//I am better than you [I am the stigma]