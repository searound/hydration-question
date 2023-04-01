use app::{App,AppProps};
use axum::Extension;
use server::file_and_error_handler;

use std::sync::Arc;

use axum::{Router, routing::post};
use leptos_axum::{generate_route_list, LeptosRoutes};
use leptos::{view, get_configuration, log};

#[tokio::main(flavor = "current_thread")]
async fn main() {

    let conf = get_configuration(None).await.unwrap();
    let addr = conf.leptos_options.site_addr;

    let leptos_options = conf.leptos_options;
    let routes = generate_route_list(|cx| view! { cx, <App/> }).await;

    let app = Router::new()
        .route("/api/*fn_name", post(leptos_axum::handle_server_fns))
        .leptos_routes(leptos_options.clone(), routes, |cx| view! { cx, <App/> })
        .fallback(file_and_error_handler)
        .layer(Extension(Arc::new(leptos_options)));

    log!("listening on http://{}", &addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap()
}
