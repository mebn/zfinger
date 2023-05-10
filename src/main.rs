mod config;
mod users;

const HOW_TO_USE: &str = "
Usage: zfinger [options] query
List all students at KTH and see a picture of some of them.
The query should not have any quotes.
Flags can be combined, e.g. zfinger -fch firstname lastname

Options:
    -f : Open the first result.
    -c : Close prompt, disable interactive loop.
    -h : Hide results and close prompt. Equivalent to -ch.
    -a : Include all students, even those with no year set.
";

#[tokio::main]
async fn main() {
    let args: Vec<String> = std::env::args().collect();

    let config = config::handle_args(&args).unwrap_or_else(|err| {
        match err {
            config::ConfigErrors::NoArgs => {},
            config::ConfigErrors::FlagNotFound(c) => eprintln!("-{c} is not a valid flag"),
            config::ConfigErrors::NoSearchQuery => eprintln!("No search query provided"),
        }

        eprintln!("{HOW_TO_USE}");
        std::process::exit(1);
    });

    let users = users::get_users(&config.query).await.unwrap_or_else(|err| {
        match err {
            users::UsersErrors::URLNotFound => eprintln!("Could not reach url"),
            users::UsersErrors::SearchTimeout => eprintln!("Search timeout, try again"),
        }

        std::process::exit(1);
    });

    users::select_users(&config, &users);
}
