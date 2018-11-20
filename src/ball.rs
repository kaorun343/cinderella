use crate::actor::Human;

pub struct Ball {
    entries: Vec<Human>,
    /// 時刻
    pub clock: u32,
    finished_at: u32,
}

impl Ball {
    pub fn new(started_at: u32, finished_at: u32) -> Self {
        Ball {
            entries: Vec::new(),
            clock: started_at,
            finished_at,
        }
    }

    pub fn start(&self) {
        println!("舞踏会開始");
    }

    pub fn dancing(&mut self) {
        println!("現在 {}時", self.clock);

        self.entries.iter().for_each(|human| {
            println!("{} は踊っている", &human.name);
        });

        self.clock += 1;
    }

    pub fn finish(&self) {
        println!("舞踏会は終了");
    }

    pub fn is_finished(&self) -> bool {
        self.clock >= self.finished_at
    }

    pub fn entry(&mut self, human: &Human) {
        match human.cos {
            Some(_) => {
                self.entries.push(human.clone());
                println!("{} は舞踏会に参加します。", human.name);
            }
            None => {
                println!("衣装を持っていないと参加できません。");
                println!("{} は舞踏会に参加できない", human.name);
            }
        }
    }

    pub fn exit(&mut self, human: &Human) {
        self.entries.retain(|entry| entry != human);
        println!(
            "{} は舞踏会から抜け出し、帰宅した。",
            human.name
        );
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::actor::Gender;
    use crate::costume::Costume;

    #[test]
    fn test_exit() {
        let mut ball = Ball::new(0, 24);
        let mut human = Human::new("", 0, Gender::Man);
        human.set_costume(Costume::Dress);
        ball.entry(&human);
        ball.exit(&human);
        assert_eq!(ball.entries, vec![]);
    }
}
