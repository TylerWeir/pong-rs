mod paddle;
mod command;
mod ball;
mod game;

extern crate ncurses;
use ncurses::*;

fn main() {
    initscr();      /* Put the terminal in curses mode */
    let mut my_paddle = paddle::Paddle::new();

    loop {
        // Move
        my_paddle.do_cmd(command::MoveRight{});
        my_paddle.do_cmd(command::MoveUp{});

        // Draw the scene
        clear();        /* Clear the scene before drawing*/
        my_paddle.draw(my_paddle.get_y() as i32, my_paddle.get_x() as i32);

        // Draw to the screen and wait
        refresh();
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
}
