mod actor;
mod ball;
mod costume;
mod magic;
mod shoes;

use self::actor::*;
use self::ball::Ball;
use self::costume::{Costume, DressRoom};
use self::magic::Magic;
use self::Gender::*;
use std::sync::mpsc::channel;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time;

fn main() {
    let mut step_mother = Human::new("StepMother", 52, Woman);
    let mut sister_a = Human::new("SisterA", 23, Woman);
    let mut sister_b = Human::new("SisterB", 20, Woman);
    let mut cinderella = Human::new("ella", 18, Woman);

    step_mother.say("今日もいじめてやるw");
    sister_a.say("今日もいじめてやるw");
    sister_b.say("今日もいじめてやるw");
    cinderella.say("・・・");

    let mut ball = Ball::new(19, 27);

    {
        let mut dress_room = DressRoom::new();

        // 舞踏会用のドレスを用意します。
        dress_room.store(&step_mother);
        dress_room.store(&sister_a);
        dress_room.store(&sister_b);

        // シンデレラのドレスは用意しません。
        // 継母のドレスはある
        dress_room.get_dress(&mut step_mother);
        // 舞踏会参加
        ball.entry(&step_mother);
        // 姉Aのドレスもある
        dress_room.get_dress(&mut sister_a);
        ball.entry(&sister_a);
        // 姉Bのドレスもある
        dress_room.get_dress(&mut sister_b);
        ball.entry(&sister_b);

        // シンデレラだけドレスがない。。
        dress_room.get_dress(&mut cinderella);
        ball.entry(&cinderella);
    }

    let magic = Magic::new(cinderella.name.clone());
    cinderella.set_costume(magic.generate_dress());
    cinderella.set_shoes(magic.generate_glass_shoes());

    let (limit_tx, limit_rx) = channel();
    let magic = Arc::new(Mutex::new(magic));
    {
        let magic = magic.clone();
        thread::spawn(move || {
            magic.lock().unwrap().limit(&limit_rx);
        });
    }

    // シンデレラ舞踏会へ参加する
    ball.entry(&cinderella);

    let mut prince = Human::new("王子", 18, Man);
    let tailcoat = Costume::Tailcoart;
    prince.set_costume(tailcoat);
    ball.entry(&prince);

    // 舞踏会開催
    ball.start();
    let (finished_tx, finished_rx) = channel::<()>();

    let ball = Arc::new(Mutex::new(ball));
    {
        let ball = ball.clone();
        thread::spawn(move || {
            let mut ball = ball.lock().unwrap();
            while !ball.is_finished() {
                thread::sleep(time::Duration::from_secs(1));
                ball.dancing();

                if ball.clock == 24 {
                    limit_tx.send(()).unwrap();
                }
            }
            ball.finish();
            finished_tx.send(()).unwrap();
        });
    }

    magic.lock().unwrap().broken_rx.recv().unwrap();
    ball.lock().unwrap().exit(&cinderella);

    finished_rx.recv().unwrap();

    println!("=== 後日 ===");
    println!("王子の部下たちはガラスの靴の持ち主を探している。");
}
