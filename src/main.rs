extern crate piston_window;
extern crate rand;

mod draw;
mod game;
mod snake;

fn multiplication_table() {
    let mut s = "".to_string();
    for r in 1..9 + 1 {
        for c in 1..r + 1 {
            s += &format!("{}x{}={}   ", c, r, r * c);
        }
        s += "\n";
    }

    print!("{}", s);
}

fn main() {
    multiplication_table();

    //    let mut s = "1".to_string();
    //    s += "1";
    //    print!("{}", s);
}
