use crate::actor_utils::*;

pub struct AI {
    paddle: crossbeam::channel::Sender<Messages>,
    last_ball_pos: Option<Messages>,
    last_paddle_pos: Option<Messages>,
}

impl AI {
    pub fn new(paddle: crossbeam::channel::Sender<Messages>) -> AI {
        AI{paddle, last_ball_pos:None, last_paddle_pos:None}
    }

    fn handle_message(&mut self, msg:Messages) {
        match msg {
            Messages::Tick => self.send_cmd(self.get_cmd()),
            Messages::BallPos(..) => self.last_ball_pos = Some(msg),
            Messages::PaddlePos(..) => self.last_paddle_pos = Some(msg),
            _ => return,
        }
    }

    fn send_cmd(&self, msg: Messages) {
        match self.paddle.try_send(msg) { _ => ()}
    }

    fn get_cmd(&self) -> Messages {
        Messages::UpCmd  
    }
}

impl Actor for AI {
    fn poll(&mut self, r: crossbeam::channel::Receiver<Messages>) {
        loop {
            match r.recv() {
                Ok(Messages::Terminate) => return,
                Ok(msg) => self.handle_message(msg),
                Err(_err) => println!("AI experiencing errors"),
            }
        }

    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_construction() {
        let (s, _) = crossbeam::channel::unbounded();
        let _my_ai = AI::new(s);
    }

    #[test]
    fn test_terminate() {
        let (s, r) = crossbeam::channel::unbounded();
        let mut my_ai = AI::new(s.clone());
        let handler = std::thread::spawn(move || {
           my_ai.poll(r);
        });
        
        match s.try_send(Messages::Terminate) {
            Ok(_) =>  (),
            Err(_) => panic!("Failed to send message"),
        }

        handler.join().unwrap();
    }
}
