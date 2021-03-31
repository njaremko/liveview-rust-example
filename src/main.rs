use crate::template::ExTemplate;
use actix_web::{error, get, web, App, HttpServer, Responder};
use actix_web::{HttpRequest, HttpResponse};
use actix_web_actors::ws;
use live_view::LiveView;
use live_view::StateSocket;
use live_view::Template;
use template::AppTemplate;
mod template;

#[get("/")]
async fn initial_load(_req: HttpRequest) -> impl Responder {
    let state = AppTemplate {
        body: ExTemplate {
            touching_text: "Look at that button under me".into(),
            ..ExTemplate::default()
        }
        .render()
        .unwrap(),
        ..AppTemplate::default()
    };
    state
        .render()
        .map(|b| HttpResponse::Ok().body(b))
        .map_err(|e| error::ErrorInternalServerError(e))
}

async fn start_socket(req: HttpRequest, stream: web::Payload) -> impl Responder {
    let mut live_view: LiveView<ExTemplate> = LiveView::default();
    live_view.on_click("inc", |_event, state| {
        state.count += 1;
        Some(state.render().unwrap())
    });
    live_view.on_click("dec", |_event, state| {
        state.count -= 1;
        Some(state.render().unwrap())
    });
    live_view.on_click("header", |_event, state| {
        state.name = "You changed the header!".into();
        Some(state.render().unwrap())
    });
    live_view.on_mouseover("touching-button", |_event, state| {
        state.touching_text = "You're touching my button!".into();
        Some(state.render().unwrap())
    });
    live_view.on_mouseout("touching-button", |_event, state| {
        state.touching_text = "Thank's for not touching my button :)".into();
        Some(state.render().unwrap())
    });
    live_view.on_submit("header-submit", |event, state| {
        if let Some(new_name) = &event.data {
            let split: Vec<&str> = new_name.split('=').collect();
            state.name = split[1].to_string() + "-submitted!";
            Some(state.render().unwrap())
        } else {
            None
        }
    });
    live_view.on_keydown("header", |event, state| {
        event.data.clone().map(|new_name| {
            state.name = new_name;
            state.render().unwrap()
        })
    });
    let actor = StateSocket {
        state: ExTemplate::default(),
        live_view,
    };
    ws::start(actor, &req, stream)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .service(web::resource("/ws/").route(web::get().to(start_socket)))
            .service(initial_load)
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
