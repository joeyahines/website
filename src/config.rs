use std::net::SocketAddr;
use structopt::StructOpt;

#[derive(StructOpt)]
pub struct SiteArgs {
    pub serve_at: SocketAddr,
}
