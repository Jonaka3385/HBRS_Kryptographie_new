mod prak1a1;
mod prak1a2;
mod biguint_functions;
mod prak1a1_small;

fn main() {
    println!("-------------------Start-------------------\n");
    println!("Aufgabe 1 small nums\n");
    prak1a1_small::run();
    println!("\n\nAufgabe 1 big nums\n");
    prak1a1::run();
    println!("\n\nAufgabe 2\n");
    prak1a2::run();
    println!("\n--------------------End--------------------");
}
