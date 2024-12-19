use std::convert::Infallible;
use std::net::SocketAddr;

use axum::{
	http::{
		HeaderMap,
		StatusCode,
	},
	response::IntoResponse,
};
use crate::fb::fb_range;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper_util::rt::TokioIo;
use tokio::net::TcpListener;

async fn fizzbuzz_service(headers: HeaderMap) -> impl IntoResponse {
	let stop = headers
		.get("stop")
		.and_then(|value| value.to_str().ok())
		.and_then(|s| s.parse::<i32>().ok())
		.unwrap_or(100);

	let output = fb_range(1, stop);

	(
		StatusCode::OK,
		[
			("Content-Type", "text/plain")
		],
		output
	)
}

async fn handler(stream : tokio::net::TcpStream) {
	let io = TokioIo::new(stream);

	if let Err(err) = http1::Builder::new()
		.serve_connection(io, service_fn(|req| async move {
			let headers = req.headers().clone();
			let response = fizzbuzz_service(headers).await;
			Ok::<_, Infallible>(response.into_response())
		}))
		.await {
			eprintln!("Error: {:?}", err);
		}
}

pub async fn start() {
	let addr = SocketAddr::from((
		[0, 0, 0, 0],
		6969
	));
	let listener = TcpListener::bind(addr).await.unwrap();

	loop {
		let (stream, _) = listener.accept().await.unwrap();
		tokio::spawn(handler(stream));
	}
}
