use futures::future::{BoxFuture, FutureExt};
use http::status::StatusCode;
use warp::{filters::{fs, path::{self, path}}, Filter, Rejection, reply::{self, Json}, Reply, any, reject};
use tokio::sync::RwLock;

use std::{future::Future, net::SocketAddr, sync::Arc, thread};

use nyanko_core::Nyanko;
use nyanko_anilist::Token;

type Runtime = Arc<RwLock<Nyanko>>;

pub fn serve(shutdown: impl Future<Output=()> + Send + 'static) -> SocketAddr {
	let mut tokio_runtime = tokio::runtime::Builder::default()
		.enable_all()
		.basic_scheduler()
		.build()
		.expect("Could not build backend server thread.");

	let runtime = Arc::new(RwLock::new(Nyanko::with_handle(tokio_runtime.handle().clone())));

	let (socket, future) = tokio_runtime.block_on(async {
		warp::serve(server(runtime.clone()))
			.try_bind_with_graceful_shutdown(([127, 0, 0, 1], 0), shutdown)
			.expect("Could not bind backend server.")
	});

	thread::spawn(move || {
		tokio_runtime
			.block_on(async {
				info!("Starting");
				future.await;
				info!("Shutting down");
				runtime.write().await.shutdown()
			});
	});

	socket
}

fn server(runtime: Runtime) -> impl Filter<Extract=impl Reply> + Clone {
	fs::dir("./nyanko/dist")
		.or(
			path("login").and(
				path("link").and_then(login_link(runtime.clone()))      // /login/link
					.or(path::param().and_then(login(runtime.clone()))) // /login
			).or(
				path("search").and(path::param()).and_then(search(runtime)) // /search/{query}
			)
		)
		.or(any().map(|| reply::with_status("Route not found.", StatusCode::NOT_FOUND)))
}

fn login_link(runtime: Runtime) -> impl Clone + Fn() -> BoxFuture<'static, Result<String, Rejection>> {
	move || {
		let runtime = runtime.clone();
		async move { Ok(runtime.read().await.anilist_login_link()) }.boxed()
	}
}

fn login(runtime: Runtime) -> impl Clone + Fn(String) -> BoxFuture<'static, Result<Json, Rejection>> {
	move |token| {
		let runtime = runtime.clone();
		async move {
			match Token::new(token) {
				Ok(token) => {
					Ok(runtime.write().await
						.anilist_create_account(token)
						.map(reply::json)
						.ok_or_else(reject::not_found)?)
				},
				Err(err) => {
					let msg = format!("Could not create account: {}", err);
					error!("{}", msg);
					Err(reject::not_found())
				}
			}
		}.boxed()
	}
}

fn search(runtime: Runtime) -> impl Clone + Fn(String) -> BoxFuture<'static, Result<Json, Rejection>> {
	move |query: String| {
		let runtime = runtime.clone();
		async move {
			Ok(runtime.read().await
				.search(query).await
				.ok().flatten()
				.map(|results| reply::json(&results))
				.expect("Could not search"))
		}.boxed()
	}
}
