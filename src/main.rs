//WORK IN PROGRESS
// Initiative tracker for table top gaming
// (program to learn rust)
use std::io::stdin;

#[derive(Debug)]
struct Creature {
    name: String,
    initiative: i8,
    alive: bool,
}

impl Creature {
    fn new(name: String, initiative: i8, alive: bool) -> Self {
        Creature {
            name,
            initiative,
            alive,
        }
    }
    fn kill(&mut self) {
        self.alive = false;
    }
}

fn main() {
    let mut creatures: Vec<Creature> = Vec::new();
    println!("\n\n\n~~~Initiative Tracker! (Rust edition)~~~\n");
    let mut creature_count: i8 = 0;
    println!("Enter the name and initiative roll for creature/player (seperated by a comma).\nWhen finished, type 'done'.\n(Tpe 'help' for more information)\n");
    loop {
        let mut input = String::new();
        stdin().read_line(&mut input).expect("error");

        if input.trim().to_lowercase() == "done" {
            break;
        }
        if input.trim().to_lowercase() == "help" {
            println!("Input the name of the creature or player, then a comma, then the inititiative roll.\nFor example, if a player (called Norman) rolled 14, you should enter 'Norman, 14'.\nWhen you've finished entering creatures/players, enter 'done'.\n");
        }

        let parts: Vec<&str> = input.trim().split(',').collect();
        if parts.len() != 2 {
            println!("You made a whoopsie :(");
            continue;
        }
        let name = parts[0].trim().to_string();
        //let rand_int: String = parts[1].trim().to_string();
        let initiative: i8 = match parts[1].trim().parse() {
            Ok(val) => val,
            Err(_) => {
                println!("invalid initiative value");
                continue;
            }
        };
        println!(
            "\n[Creature #{}] {} added successfully. Enter another, or type 'done' to finish.\n",
            creature_count + 1,
            name
        );

        creatures.push(Creature::new(name, initiative, true));
        creature_count += 1;
    }

    creatures.sort_by(|a, b| b.initiative.cmp(&a.initiative));
    let mut round_number: i8 = 1;
    let mut init_order: i8 = 1;
    println!("\n-----------------------------------\nGet ready!\nFull combatant order:\n");
    for creat in &mut creatures {
        println!("{} - {}", creat.name, creat.initiative);
    }

    //TO DO ---> Have options for editing/deleting values

    println!("\n-----------------------------------");
    'combat: loop {
        println!("\nRound {}!\n", round_number);
        for creature in &mut creatures {
            if creature_count <= 1 {
                println!("Combat ended!");
                break 'combat;
            }

            if creature.alive {
                println!(
                    "{}) {} - (initiative: {})",
                    init_order, creature.name, creature.initiative
                );
                //}
                let mut input_combat = String::new();
                stdin().read_line(&mut input_combat).expect("error");
                let input_combat = input_combat.trim().to_lowercase();
                if input_combat == "exit" {
                    break 'combat;
                }
                if input_combat == "x" {
                    &creature.kill();
                    //creatures.remove(creature);
                    println!("{} has perished :(\n", creature.name);
                    creature_count -= 1;
                    init_order += 1;
                    continue;
                }
                init_order += 1;
            }
        }
        init_order = 1;
        round_number += 1;
    }
    for creatureAlive in &mut creatures {
        if creatureAlive.alive == true {
            println!("{} is the victor!", creatureAlive.name);
        }
    }
}
