use std::thread;
use std::time::Duration;
extern crate redis;

struct Ctx {
    pub client: redis::Client,
}

impl Ctx {
    fn new() -> Ctx {
        let client = redis::Client::open("redis://localhost/").unwrap();
        Ctx {
            client: client,
        }
    }
    fn pubsub(&self) -> redis::PubSub {
        self.client.get_pubsub().unwrap()
    }
}

fn main() {
    let handle = thread::spawn(|| {
        let ctx = Ctx::new();
        let mut ps = ctx.pubsub();

        ps.subscribe("channel_1").unwrap();
        ps.subscribe("channel_2").unwrap();

        for x  in 0..10 {
            println!("Subscriber: {} times...", x);
            let msg = ps.get_message().unwrap();
            let payload : String = msg.get_payload().unwrap();
            println!("channel '{}': {}", msg.get_channel_name(), payload);
        }
    });

    thread::sleep(Duration::from_millis(500));

    thread::spawn(|| {
        let ctx = Ctx::new();
        let con = ctx.client.get_connection().unwrap();

        for x  in 0..10 {
            redis::cmd("PUBLISH").arg("channel_1").arg(x).execute(&con);
        }
    });

    handle.join().unwrap();

}
