extern crate redis;

use redis::{Client, ControlFlow, PubSubCommands};
use std::thread;
use std::time::Duration;

struct Ctx {
    pub client: Client,
}

impl Ctx {
    fn new() -> Ctx {
        let client = Client::open("redis://localhost/").unwrap();
        Ctx { client: client }
    }
}

fn subscribe(ctx: Ctx) -> thread::JoinHandle<()> {
    thread::spawn(move || {
        let mut conn = ctx.client.get_connection().unwrap();

        conn.subscribe(&["boo"], |msg| {
            let ch = msg.get_channel_name();
            let payload: String = msg.get_payload().unwrap();
            match payload.as_ref() {
                "10" => ControlFlow::Break(()),
                a => {
                    println!("Channel: '{}' received '{}'.", ch, a);
                    ControlFlow::Continue
                }
            }
        }).unwrap();
    })
}

fn publish(ctx: Ctx) {
    thread::spawn(move || {
        let con = ctx.client.get_connection().unwrap();

        for x in 0..11 {
            redis::cmd("PUBLISH").arg("boo").arg(x).execute(&con);
        }
    });
}

fn main() {
    let handle = subscribe(Ctx::new());

    thread::sleep(Duration::from_millis(500));

    publish(Ctx::new());

    handle.join().unwrap();
}
