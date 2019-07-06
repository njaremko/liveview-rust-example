use crate::template::ExTemplate;
use actix_web::{web, App, HttpServer};
use actix_web::{HttpRequest, HttpResponse};
use actix_web_actors::ws;
use live_view::StateSocket;
use live_view::LiveView;
use live_view::Template;
use live_view::BaseTemplate;
mod template;

pub type Result<T> = std::result::Result<T, failure::Error>;

fn initial_load(_req: HttpRequest) -> Result<HttpResponse> {
    let state = BaseTemplate {
        title: "Example".into(),
        body: ExTemplate::default().render()?,
        ..BaseTemplate::default()
    };
    Ok(HttpResponse::Ok().body(state.render()?))
}

fn start_socket(
    req: HttpRequest,
    stream: web::Payload,
) -> std::result::Result<HttpResponse, actix_web::Error> {
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
    live_view.on_submit("header-submit", |event, state| {
        if let Some(new_name) = &event.data {
            let split: Vec<&str> = new_name.split('=').collect();
            state.name = split[1].to_string() + "-submitted!";
            Some(state.render().unwrap())
        } else {
            None
        }
    });
    live_view.on_input("header", |event, state| {
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

fn main() -> Result<()> {
    HttpServer::new(move || {
        App::new()
            .service(web::resource("/ws/").route(web::get().to(start_socket)))
            .route("/", web::get().to(initial_load))
    })
    .bind("127.0.0.1:8000")?
    .run()?;
    Ok(())
}
