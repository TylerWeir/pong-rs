use crate::actor_utils::*;

pub struct AI {
    paddle: crossbeam::channel::Sender<Messages>,
}

impl AI {
    pub fn new(paddle: crossbeam::channel::Sender<Messages>) -> AI {
        AI{paddle}
    }

    fn handle_message(&mut self, msg:Messages) {
        match msg {
            Messages::Tick => self.send_cmd(),
            _ => return,
        }
    }

    fn send_cmd(&self) {
        match self.paddle.try_send(Messages::UpCmd) { _ => ()}
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
}
