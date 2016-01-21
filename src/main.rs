extern crate ansi_term;
extern crate rand;


use std::io;
use std::thread::sleep_ms;
use std::io::{Write,stdout};
use rand::Rng;
use std::vec;
use ansi_term::Colour::Fixed;
use std::string;



struct Isotope {
    symbol :char,
    colour : u8,
    decay_prob : f64,

}

impl Isotope{
    fn new(symbol : char, colour: u8, decay_prob:f64) -> Isotope {
        Isotope {
            symbol : symbol,
            colour : colour,
            decay_prob : decay_prob,
        }
    }
}

struct Atom<'a>{
    chain : &'a Vec<Isotope>,
    current : usize,
}

impl<'a> Atom<'a>{
    fn new(atom_types : &Vec<Isotope>, current : usize) -> Atom {
        Atom {
            chain: atom_types,
            current : current
        }
    }

    fn step(&mut self) -> bool{//returns true if decayed
        let sim_limit: i32 = 1000000000;
        if self.current < self.chain.len() {
            let dp = self.chain[self.current].decay_prob;
            let rn = rand::thread_rng().gen_range(1,sim_limit);

            if dp*sim_limit as f64 > rn as f64{
                self.current += 1;
                return true
            }
        }
        false
    }
}

fn print_atoms(atoms:&Vec<Atom>){
    stdout().write_all("\x1b[2J\x1b[1;1H".as_bytes());//based on CLI lib at https://github.com/fengsp/cli
    for atom in atoms {
        let current_isotope = &atom.chain[atom.current];
        print!("{}",Fixed(current_isotope.colour).paint(current_isotope.symbol.to_string()).to_string());
    }
}

fn main(){
    let isotopes:Vec<_> = vec![Isotope::new('A',27,0.1f64),Isotope::new('B',28,0.2f64),Isotope::new('C',240,0f64)];
    let mut atoms:Vec<_> = Vec::new();
    for x in 0..8000 {
        atoms.push(Atom::new(&isotopes,0));
    }
    for steps in 0..500 {
        let mut decayCount = 0usize;
        for i in 0..atoms.len() {
            if atoms[i].step() {decayCount+=1;
            }
        }
        print_atoms(&atoms);
        println!(" ");
        println!("{} decayed",decayCount);
        sleep_ms(300);
    }
}
