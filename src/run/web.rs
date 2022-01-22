use futures::Future;
use thiserror::Error;
use tokio::net;
use warp::Filter;

use super::RunOpts;

#[derive(Debug, Error)]
pub enum Error {
    #[error("invalid bind address: {0}")]
    InvalidBindAddr(String),
    #[error("could not start server: {0}")]
    Web(#[from] warp::Error),
}

pub async fn run(
    opts: &RunOpts,
) -> Result<impl Future<Output = Result<(), Error>> + 'static, Error> {
    // Get the bind address
    let bind = net::lookup_host(&opts.bind)
        .await
        .ok()
        .and_then(|mut it| it.next())
        .ok_or_else(|| Error::InvalidBindAddr(opts.bind.clone()))?;

    let routes = warp::fs::dir(opts.serve_root.clone()).or(warp::path("healthz").map(warp::reply));

    // Bind the server to the address
    let (addr, server) = warp::serve(routes).try_bind_ephemeral(bind)?;
    tracing::info!("listening on http://{}", addr);

    Ok(async move {
        server.await;
        Ok(())
    })
}
