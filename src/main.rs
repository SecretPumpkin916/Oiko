mod readconfig;
mod login;
mod app;
mod ipfs;
mod catalog;
#[tokio::main]
async fn main() {
    let _ = readconfig::readconfig();
    login::login();
    app::app().await;
}