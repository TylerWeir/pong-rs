use crate::utils::sprite::Sprite;
use crate::actor_utils::Point;
use crate::actor_utils::Messages;
use crate::actor_utils::Actor;

pub struct Paddle {
    pos: Point,
    sprite: Sprite,
    broker: crossbeam::channel::Sender<Messages>,
}

impl Actor for Paddle {
    fn poll (&mut self, r: crossbeam::channel::Receiver<Messages>) {
        loop {
            match r.recv() {
                Ok(Messages::Terminate) => return,
                Ok(msg) => self.handle_message(msg),
                Err(_err) => println!("paddle experiencing errors"),
            }
        }
    }
}

impl Paddle {
    // Creates a new paddle with zero valued fields
    pub fn new(init_pos: Point, s: crossbeam::channel::Sender<Messages>) -> Paddle {
        Paddle {
            pos: init_pos,
            sprite: Sprite::new(2, 6,"##\n##\n##\n##\n##\n##"),
            broker: s
        }
    }

    fn tick(&self) {
        match self.broker.try_send(Messages::Draw(self.pos, self.sprite)) {
            Ok(_) => println!("draw send"),
            Err(_err) => println!("draw not send"),
        }
    }

    fn handle_message(&mut self, msg:Messages) {
        match msg {
            Messages::Tick => self.tick(),
            Messages::UpCmd => self.pos.add_y(1),
            Messages::DownCmd => self.pos.add_y(-1),
            _ => return, 
        }
    }
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_construction() {
        let (sender, _) = crossbeam::channel::unbounded();
        let my_paddle = Paddle::new(Point::new(0, 0), sender.clone());

        assert_eq!(my_paddle.pos, Point::new(0, 0));
    }

    #[test]
    fn test_move_up() {
        let (sender, r) = crossbeam::channel::unbounded();
        let mut my_paddle = Paddle::new(Point::new(0, 10), sender.clone());

        match sender.try_send(Messages::UpCmd) { _ => () }
        match sender.try_send(Messages::Terminate) { _ => () }
        my_paddle.poll(r);

        assert_eq!(my_paddle.pos, Point::new(0, 11));
    }

    #[test]
    fn test_move_down() {
        let (sender, r) = crossbeam::channel::unbounded();
        let mut my_paddle = Paddle::new(Point::new(0, 10), sender.clone());

        match sender.try_send(Messages::DownCmd) { _ => () }
        match sender.try_send(Messages::Terminate) { _ => () }
        my_paddle.poll(r);

        assert_eq!(my_paddle.pos, Point::new(0, 9));
    }

}

