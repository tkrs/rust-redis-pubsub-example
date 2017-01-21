use std::thread;
use std::time::Duration;

extern crate redis;

struct Ctx {
    pub client: redis::Client,
}

impl Ctx {
    fn new() -> Ctx {
        let client = redis::Client::open("redis://localhost/").unwrap();
        Ctx { client: client }
    }
    fn pubsub(&self) -> redis::PubSub {
        self.client.get_pubsub().unwrap()
    }
}

fn subscribe(ctx: Ctx) -> thread::JoinHandle<()> {
    thread::spawn(move || {
        let mut ps = ctx.pubsub();

        ps.subscribe("boo").unwrap();

        println!("Subscriber is ready.");

        loop {
            let msg = ps.get_message().unwrap();
            let ch = msg.get_channel_name();
            let payload: String = msg.get_payload().unwrap();
            match payload.as_ref() {
                "10" => break,
                a => println!("Channel: '{}' received '{}'.", ch, a),
            }
        }
    })
}

fn publish(ctx: Ctx) {
    thread::spawn(move || {
        let con = ctx.client.get_connection().unwrap();

        for x in 0..11 {
            redis::cmd("PUBLISH")
                .arg("boo")
                .arg(x)
                .execute(&con);
        }
    });
}

fn main() {
    let handle = subscribe(Ctx::new());

    thread::sleep(Duration::from_millis(500));

    publish(Ctx::new());

    handle.join().unwrap();

}
