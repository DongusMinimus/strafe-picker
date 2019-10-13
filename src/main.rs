#[macro_use]
extern crate html5ever;

use std::env;

mod get_hltv;
mod parser;

struct Team{
    name: String,
    hltv: f32,
    strafe: u16
}

fn winner(a: Team, b:Team) {
    let g = a.hltv * f32::from(a.strafe);
    let h = b.hltv * f32::from(b.strafe);
    println!("{}:{} vs {}:{}", a.name, g, b.name, h);
    
    if g > h{
        println!("{} wins!", a.name);
    }else if g < h{
        println!("{} wins!", b.name);
    }else{
        println!("Stalemate!");
    }
}

fn main() {

    let args:Vec<String> = env::args().collect();

    if args.len() != 3 {
        panic!("Programm has less then 3 args!");
    }

    get_hltv::update();
    let hm = parser::parse();

    if !hm.contains_key(&args[1]) || !hm.contains_key(&args[2]) {
        panic!("One of the teams is not included in the HashMap!");
    }
    
    let a = Team {name:args[1].clone(), hltv:*hm.get(&args[1]).unwrap(), strafe:100};
    let b = Team {name:args[2].clone(), hltv:*hm.get(&args[2]).unwrap(), strafe:100};
    winner(a,b);
}
