use std::time::{Duration, Instant};

use actix::prelude::*;
use actix_web_actors::ws;

use bytestring::ByteString;

use rand::distributions::{Uniform};
use rand::prelude::*;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct RandomInteger {
    integer: u8
}

/// How often heartbeat pings are sent
const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);

/// How long before lack of client response causes a timeout
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

/// How often random numbers are sent
const RANDOM_NUMBER_INTERVAL: Duration = Duration::from_secs(1);

/// websocket connection is long running connection, it easier
/// to handle with an actor
pub struct MyWebSocket {
    /// Client must send ping at least once per 10 seconds (CLIENT_TIMEOUT),
    /// otherwise we drop connection.
    hb: Instant,
}

fn string_to_static_str(s: String) -> &'static str {
    Box::leak(s.into_boxed_str())
}

impl MyWebSocket {
    pub fn new() -> Self {
        Self { hb: Instant::now() }
    }

    /// helper method that sends ping to client every second.
    ///
    /// also this method checks heartbeats from client
    fn hb(&self, ctx: &mut <Self as Actor>::Context) {
        ctx.run_interval(HEARTBEAT_INTERVAL, |act, ctx| {
            // check client heartbeats
            if Instant::now().duration_since(act.hb) > CLIENT_TIMEOUT {
                // heartbeat timed out
                println!("Websocket Client heartbeat failed, disconnecting!");

                // stop actor
                ctx.stop();

                // don't try to send a ping
                return;
            }

            ctx.ping(b"");
        });
    }

    /// helper method that sends random integer (1..10) to client every 2 seconds.
    fn schedule_random_integer(&self, ctx: &mut <Self as Actor>::Context) {
        let step = Uniform::new(1 as u8, 10 as u8);
        let mut rng = rand::thread_rng();
        ctx.run_interval(RANDOM_NUMBER_INTERVAL,move |_, ctx| {
            let random_integer = RandomInteger {integer: step.sample(&mut rng)};
            let message = serde_json::to_string(&random_integer).unwrap();
            ctx.text(ByteString::from_static(string_to_static_str(message)));
        });
    }
}

impl Actor for MyWebSocket {
    type Context = ws::WebsocketContext<Self>;

    /// Method is called on actor start. We start the heartbeat process here.
    fn started(&mut self, ctx: &mut Self::Context) {
        self.hb(ctx);
        self.schedule_random_integer(ctx);
    }
}

// impl Handler<ws::Connect> for MyWebSocket {

// }

/// Handler for `ws::Message`
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for MyWebSocket {
    

    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        // process websocket messages
        println!("WS: {:?}", msg);
        match msg {
            Ok(ws::Message::Ping(msg)) => {
                self.hb = Instant::now();
                ctx.pong(&msg);
            }
            Ok(ws::Message::Pong(_)) => {
                self.hb = Instant::now();
            }
            Ok(ws::Message::Text(text)) =>
            // Process text messages
            {
                if text == ByteString::from_static("Hello") {
                    ctx.text("Hello World! from Actix Web");
                } else {
                    ctx.text(text);
                }
            }
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            Ok(ws::Message::Close(reason)) => {
                ctx.close(reason);
                ctx.stop();
            }
            _ => ctx.stop(),
        }
    }
}
