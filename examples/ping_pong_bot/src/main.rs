use teloxide::prelude::*;

#[tokio::main]
async fn main() {
    std::env::set_var("RUST_LOG", "ping_pong_bot=trace");
    pretty_env_logger::init();
    log::info!("Starting the ping-pong bot!");

    let bot = Bot::new("MyAwesomeToken").set_dispather(Dispatcher::new([
        Observer::new(MessageResive, |ctx| async move {
            ctx.reply("pong").await
        })
    ]));

    Session::default().asociate_bot(bot).idle();
}
