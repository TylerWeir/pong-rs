mod paddle;
mod command;
mod ball;
mod game;
mod physics;
mod ipc;
mod broker;
mod screen;

use std::thread;
use crossbeam::channel::unbounded;
use crate::ipc::Messages;
use crate::ipc::Actor;

fn main() {
    let (broker_s, broker_r) = unbounded();
    
    // Spin up actors
    //
    // Each actor should: 
    // - poll on its respective channel
    // - send messages out through the broker sender
    
    let (ball_s, ball_r) = unbounded();
    let mut ball = ball::Ball::new(broker_s.clone());
    let _handler = thread::spawn(move || {
        ball.poll(ball_r);
    });

    let (paddle_s, paddle_r) = unbounded();
    let mut paddle = paddle::Paddle::new(broker_s.clone());
    let _handler = thread::spawn(move || {
       paddle.poll(paddle_r);
    });

    let (screen_s, screen_r) = unbounded();
    let mut screen = screen::Screen::new(broker_s.clone());
    let _handler = thread::spawn(move || {
        screen.poll(screen_r);
    });

    let members = vec!(paddle_s, ball_s, screen_s);
    let mut broker = broker::Broker::new(members);
    let _handler = thread::spawn(move || {
        broker.poll(broker_r);
    });


    loop {
        match broker_s.try_send(Messages::Tick) {
            Ok(_) => println!("tick message sent"),
            Err(_err) => println!("error sending message"),
        }

        std::thread::sleep(std::time::Duration::from_millis(100));
    }
}
