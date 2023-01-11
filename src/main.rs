mod paddle;
mod command;
mod ball;
mod game;
mod physics;
mod ipc;

use std::{thread};
use crossbeam::channel::unbounded;
use crate::ipc::Messages;
use crate::ipc::Actor;

fn main() {
    let (broker_s, _broker_r) = unbounded();
    
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



    loop {
        match ball_s.try_send(Messages::TickMsg) {
            Ok(_) => println!("tick message sent"),
            Err(_err) => println!("error sending message"),
        }

        match paddle_s.try_send(Messages::TickMsg) {
            Ok(_) => println!("tick message sent"),
            Err(_err) => println!("error sending message"),
        }
        std::thread::sleep(std::time::Duration::from_millis(500));
    }
}
