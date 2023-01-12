mod paddle;
mod command;
mod ball;
mod game;
mod physics;
mod actor_utils;
mod board;
mod broker;

use std::thread;
use crossbeam::channel::unbounded;
use crate::actor_utils::Messages;
use crate::actor_utils::Actor;

fn main() {
    let (broker_s, broker_r) = unbounded();
    
    // Spin up actors
    //
    // Each actor should: 
    // - poll on its respective channel
    // - send messages out through the broker sender
    let (ball_s, ball_r) = unbounded();
    let ball = ball::Ball::new(broker_s.clone());
    let _handler = thread::spawn(move || {
        ball.poll(ball_r);
    });

    let (paddle_s, paddle_r) = unbounded();
    let paddle = paddle::Paddle::new(broker_s.clone());
    let _handler = thread::spawn(move || {
       paddle.poll(paddle_r);
    });

    let (board_s, board_r) = unbounded();
    let board = board::Board::new(broker_s.clone());
    let _handler = thread::spawn(move || {
       board.poll(board_r);
    });

    let members = vec!(paddle_s, ball_s, board_s);
    let broker = broker::Broker::new(members);
    let _handler = thread::spawn(move || {
        broker.poll(broker_r);
    });

    loop {
        match broker_s.try_send(Messages::TickMsg) {
            Ok(_) => println!("tick message sent"),
            Err(_err) => println!("error sending message"),
        }

        std::thread::sleep(std::time::Duration::from_millis(500));
    }
}
