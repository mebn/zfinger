mod config;
mod users;

#[tokio::main]
async fn main() {
    let how_to_use = "
        Usage: zfinger [options] query

        [options] can also be combined, e.g. zfinger -fch hello world
        -f : Open the first result and exit.
        -c : Close prompt, disable interactive loop.
        -h : Hide results.
        query : The search query, no quotes around query.
    ";

    let args: Vec<String> = std::env::args().collect();

    let config = config::handle_args(&args).unwrap_or_else(|_err| {
        println!("{how_to_use}");
        std::process::exit(1);
    });

    let users = users::get_users(&config.query).await.unwrap_or_else(|err| {
        match err {
            users::UsersErrors::URLNotFound => println!("Could not reach url"),
            users::UsersErrors::SearchTimeout => println!("Search timeout, try again"),
        }

        std::process::exit(1);
    });

    users::select_users(&config, &users);
}
