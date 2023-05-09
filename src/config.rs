#[derive(Debug)]
pub enum ConfigErrors {
    TooFewArgs,
    NoSearchQuery,
    FlagNotFound,
}

#[derive(Default)]
#[derive(Debug)]
pub struct Config {
    pub program: String,
    pub query: String,
    pub first: bool,
    pub close: bool,
    pub hide_users: bool,
}

pub fn handle_args(mut args: &[String]) -> Result<Config, ConfigErrors> {
    if args.len() < 2 {
        return Err(ConfigErrors::TooFewArgs);
    }

    let mut config = Config {
        program: args[0].clone(),
        ..Default::default()
    };

    args = &args[1..];
    let mut index = 0;

    // capture flags
    for arg in args {
        let chars = arg.as_bytes();

        if arg.as_bytes()[0] == '-' as u8 {
            for &c in &chars[1..] {
                let c = c as char;

                if c == 'f' {
                    config.first = true;
                } else if c == 'c' {
                    config.close = true;
                } else if c == 'h' {
                    config.hide_users = true;
                } else {
                    return Err(ConfigErrors::FlagNotFound);
                }
            }

            index += 1;
        } else {
            break;
        }
    }

    config.query = args[index..].join(" ");

    if config.query.is_empty() {
        return Err(ConfigErrors::NoSearchQuery);
    }

    Ok(config)
}
