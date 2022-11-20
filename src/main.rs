mod paddle;
mod command;
mod ball;
mod game;

fn main() {
    println!("Hello, world!");

    let player1 = paddle::Paddle::new();
    
    loop {
        // Get Input
        let my_cmd = command::MoveUp{};          
        player1.do_cmd(my_cmd);

        // Render
        
        // Render Game
        println!("x reads: {}", player1.get_x());
        println!("y reads: {}", player1.get_y());
        println!("width reads: {}", player1.get_width());
        println!("height reads: {}", player1.get_height());
        println!();
    }
}
