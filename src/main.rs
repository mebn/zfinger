mod config;
mod users;

const HOW_TO_USE: &str = "
Usage: zfinger [options] query
List all students at KTH and see a picture of some of them.
The query should not have any quotes.

Flags can also be combined, e.g. zfinger -fch hello world
    -f : Open the first result.
    -c : Close prompt, disable interactive loop.
    -h : Hide results.
    -a : Include all student, even those with no year set.
";

#[tokio::main]
async fn main() {
    let args: Vec<String> = std::env::args().collect();

    let config = config::handle_args(&args).unwrap_or_else(|_err| {
        println!("{HOW_TO_USE}");
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
