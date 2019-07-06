use actix::prelude::*;
use actix_web::{web, App, Error, HttpRequest, HttpResponse, HttpServer};
use actix_web_actors::ws;
use askama::Template;

use crate::template::{ExTemplate, BaseTemplate};

/// Define http actor
struct MyWs {
    state: ExTemplate,
}

impl Actor for MyWs {
    type Context = ws::WebsocketContext<Self>;
}

/// Handler for ws::Message message
impl StreamHandler<ws::Message, ws::ProtocolError> for MyWs {
    fn handle(&mut self, msg: ws::Message, ctx: &mut Self::Context) {
        match msg {
            ws::Message::Ping(msg) => ctx.pong(&msg),
            ws::Message::Text(text) => {
                match text.as_ref() {
                    "inc" => {
                        self.state.count += 1;
                        let rendered = self.state.render().unwrap();
                        ctx.text(rendered)
                    },
                    "dec" => {
                        self.state.count -= 1;
                        let rendered = self.state.render().unwrap();
                        ctx.text(rendered)
                    },
                    _ => (),
                }
            },
            ws::Message::Binary(bin) => {
                dbg!(ctx.binary(bin))
            },
            ws::Message::Close(_) => {
                ctx.stop();
            }
            _ => (),
        }
    }
}

pub(crate) fn start_socket(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    let actor = MyWs {
        state: ExTemplate {
            name: "apples".into(),
            count: 0,
        }
    };
    let resp = ws::start(actor, &req, stream);
    println!("{:?}", resp);
    resp
}