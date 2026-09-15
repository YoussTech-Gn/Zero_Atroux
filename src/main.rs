enum Command {
    Move { x: i32, y: i32 },
    ChangeColor(u8 , u8 , u8),
}



fn main() {
    let command = Command::Move { x: 10, y: 20 };
    let command = Command::ChangeColor(255, 233, 0);
    // it's shadowed the last command, so the first one is not used anymore

    match command {
        Command::Move { x, y } => {
            println!("Moving to coordinates: ({}, {})", x, y);
        }
        Command::ChangeColor(r, g, b) => {
            println!("Changing color to RGB({}, {}, {})", r, g, b);
        }
    }
}