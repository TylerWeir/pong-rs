mod paddle;
mod ball;
mod game;
mod physics;
mod actor_utils;
mod broker;
mod screen;
mod utils;

use std::thread;
use crossbeam::channel::unbounded;
use crate::actor_utils::Messages;
use crate::actor_utils::Actor;
use crate::actor_utils::Point;

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
    let mut paddle = paddle::Paddle::new(Point::new(5, 5), broker_s.clone());
    let _handler = thread::spawn(move || {
       paddle.poll(paddle_r);
    });

    // TODO Need to set up a virtual screen
    let (ai_paddle_s, ai_paddle_r) = unbounded();
    let mut paddle = paddle::Paddle::new(Point::new(30, 5), broker_s.clone());
    let _handler = thread::spawn(move || {
       paddle.poll(ai_paddle_r);
    });

    let (screen_s, screen_r) = unbounded();
    let mut screen = screen::Screen::new(broker_s.clone());
    let _handler = thread::spawn(move || {
        screen.poll(screen_r);
    });

    let members = vec!(ai_paddle_s, paddle_s, ball_s, screen_s);
    let mut broker = broker::Broker::new(members);
    let _handler = thread::spawn(move || {
        broker.poll(broker_r);
    });

    loop {
        match broker_s.try_send(Messages::Tick) {
            Ok(_) => (),
            Err(_err) => panic!("Main failed to send a tick message"), 
        }
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
}
