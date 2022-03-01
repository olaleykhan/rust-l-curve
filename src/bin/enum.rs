enum Directions {
    Up,
    Down,
}

enum Colors {
    Blue,Green,Yellow,Red,
}

fn main() {
  

     fn print_color(color:Colors){

        match color{
            Colors::Blue=> println!( "the chosen color is Blue"),
            Colors::Green=> println!( "the chosen color is green"),
            Colors::Yellow=> println!( "the chosen color is Yellow"),
            Colors::Red=> println!( "the chosen color is green"),
        }
     }

     print_color(Colors::Blue)
}
