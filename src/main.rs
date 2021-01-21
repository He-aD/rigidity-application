use actix_web::{App, HttpServer};
use rigidity_application::{middlewares, app_conf, new_websocket_lobby};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    app_conf::set_env();
    let conn = app_conf::connect_database();
    let ws_srv = new_websocket_lobby(conn.clone()); //important if clone in closure ref not properly tracked

    let http_server = HttpServer::new(move || {
        App::new()
            .data(conn.to_owned())
            .data(ws_srv.clone())
            .wrap(middlewares::CheckLogin)
            .wrap(app_conf::middleware_logger())
            .wrap(app_conf::middleware_identity_service())
            .route("/ws", app_conf::ws_routes::get())
            .service(app_conf::open_routes::get_all())
            .service(app_conf::api_routes::get_all())
            .service(app_conf::static_routes::get_all())
            .default_service(app_conf::static_routes::default_service()) // 404
    });

    if let Some(max_nb_workers) = app_conf::nb_worker() {
        http_server
            .workers(max_nb_workers as usize)
            .bind(app_conf::get_listen_address())?
            .run()
            .await
    } else {
        http_server.bind(app_conf::get_listen_address())?
            .run()
            .await
    }
}
