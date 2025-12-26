use clap::Parser;
use std::net::SocketAddr;
use std::path::PathBuf;

#[derive(Parser, Debug, Clone)]
#[command(author, version, about, long_about=None)]
pub struct Args {
    #[arg(long, help = "Target domain for redirecting and downloading")]
    pub domain: String,
    #[arg(
        long,
        default_value = "127.0.0.1:8080",
        help = "Address and port to bind"
    )]
    pub bind: SocketAddr,
    #[arg(long, default_value = ".", help = "Directory to save downloaded files")]
    pub download_dir: PathBuf,
}
