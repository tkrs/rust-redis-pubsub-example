extern crate redis;

pub struct Ctx {
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

// fn fetch_an_integer() -> redis::RedisResult<i32> {
//     let con = try!(client.get_connection());
//     let _ : () = try!(con.set("my_key", 42));
//     let my_val : i32 = try!(con.get("my_key"));
//     Ok(my_val)
// }
//
// fn recieve() -> redis::RedisResult<()> {
//     let client = try!(redis::Client::open("redis://localhost/"));
//     let mut pubsub = try!(client.get_pubsub());
//     try!(pubsub.subscribe("channel_1"));
//     try!(pubsub.subscribe("channel_2"));
//
//     loop {
//         let msg = try!(pubsub.get_message());
//         let payload : String = try!(msg.get_payload());
//         println!("channel '{}': {}", msg.get_channel_name(), payload);
//     }
//
// }

fn main() {
    // let r = fetch_an_integer();
    // println!("{}", r.ok().unwrap());
    // let _ : () = recieve()
    let ctx = Ctx::new();
    let mut ps = ctx.pubsub();

    ps.subscribe("channel_1").unwrap();
    ps.subscribe("channel_2").unwrap();

    loop {
        println!("loop...");
        let msg = ps.get_message().unwrap();
        let payload : String = msg.get_payload().unwrap();
        println!("channel '{}': {}", msg.get_channel_name(), payload);
    }

}
