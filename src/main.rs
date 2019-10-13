#[macro_use]
extern crate html5ever;

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
    }else{
        println!("{} wins!", b.name);
    }
}

fn main() {

    let mut file = get_hltv::update();
    let hm = parser::parse(&mut file); 

    let a = Team {name:"BIG".to_string(), hltv:1.3, strafe:200};
    let b = Team {name:"Sprout".to_string(), hltv:1.2, strafe:250};
    winner(a,b);
}
