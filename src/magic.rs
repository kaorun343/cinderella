use crate::costume::Costume;
use crate::shoes::Shoes;
use std::sync::mpsc::{channel, Receiver, Sender};

pub struct Magic {
    target_name: String,
    broken_tx: Sender<()>,
    pub broken_rx: Receiver<()>,
}

impl Magic {
    pub fn new(target_name: String) -> Self {
        let (broken_tx, broken_rx) = channel();
        Magic {
            target_name,
            broken_tx,
            broken_rx,
        }
    }

    pub fn generate_dress(&self) -> Costume {
        println!(
            "{} は魔法でドレスを作ってもらった！",
            self.target_name
        );
        Costume::Dress
    }

    pub fn generate_glass_shoes(&self) -> Shoes {
        println!(
            "{} は魔法でガラスの靴を作ってもらった！",
            self.target_name
        );
        Shoes
    }

    pub fn limit(&mut self, limit: &Receiver<()>) {
        limit.recv().unwrap();
        println!("0時が近づいている");
        println!("魔法が解けそう！！");
        self.broken_tx.send(()).unwrap();
    }
}
