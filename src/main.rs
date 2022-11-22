mod paddle;
mod command;
mod ball;
mod game;
mod physics;

extern crate ncurses;
use ncurses::*;
use crate::physics::SolidBody;

fn main() {
    let game_window = initscr();      /* Put the terminal in curses mode */
    let mut game_ball = ball::Ball::new(game_window);

    loop {
        // Move
        game_ball.update(100);

        // Draw the scene
        clear();        /* Clear the scene before drawing*/
        game_ball.draw();

        // Draw to the screen and wait
        refresh();
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
}
