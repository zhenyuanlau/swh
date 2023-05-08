use std::convert::Infallible;
use std::net::SocketAddr;

use http_body_util::Full;
use hyper::body::Bytes;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{Request, Response};
use miette::Result;
use tokio::net::TcpListener;

use super::CommandHandler;
use crate::api::SWH_HTTP_API_PORT;
use async_trait::async_trait;
use clap::Args;

pub struct ServeCommand;

#[derive(Debug, Args)]
pub struct ServeArgs {}

#[async_trait]
impl CommandHandler<ServeArgs> for ServeCommand {
    async fn process(&self, _args: &ServeArgs) -> Result<()> {
        // let stdout = File::create("/tmp/daemon.out").unwrap();
        // let stderr = File::create("/tmp/daemon.err").unwrap();

        // let daemonize = Daemonize::new()
        //     .pid_file("/tmp/test.pid") // Every method except `new` and `start`
        //     .chown_pid_file(true) // is optional, see `Daemonize` documentation
        //     .working_directory("/tmp") // for default behaviour.
        //     .user("nobody")
        //     .group("daemon") // Group name
        //     .group(2) // or group id.
        //     .umask(0o777) // Set umask, `0o027` by default.
        //     .stdout(stdout) // Redirect stdout to `/tmp/daemon.out`.
        //     .stderr(stderr) // Redirect stderr to `/tmp/daemon.err`.
        //     .privileged_action(|| "Executed before drop privileges");

        // match daemonize.start() {
        //     Ok(_) => println!("Success, daemonized"),
        //     Err(e) => eprintln!("Error, {}", e),
        // }

        let addr: SocketAddr = ([127, 0, 0, 1], SWH_HTTP_API_PORT).into();
        let listener = TcpListener::bind(addr)
            .await
            .expect("create listener error");
        loop {
            let (stream, _) = listener.accept().await.expect("msg");

            // Spawn a tokio task to serve multiple connections concurrently
            tokio::task::spawn(async move {
                // Finally, we bind the incoming connection to our `hello` service
                if let Err(err) = http1::Builder::new()
                    // `service_fn` converts our function in a `Service`
                    .serve_connection(stream, service_fn(serve))
                    .await
                {
                    eprintln!("Error serving connection: {:?}", err);
                }
            });
        }
    }
}

async fn serve(_: Request<hyper::body::Incoming>) -> Result<Response<Full<Bytes>>, Infallible> {
    Ok(Response::new(Full::new(Bytes::from("imok"))))
}
