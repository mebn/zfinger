#[derive(Debug)]
pub enum ConfigErrors {
    NoArgs,
    NoSearchQuery,
    FlagNotFound(char),
}

#[derive(Default)]
#[derive(Debug)]
pub struct Config {
    pub program: String,
    pub query: String,
    pub first: bool,
    pub close: bool,
    pub hide_result: bool,
    pub all_users: bool,
}

pub fn handle_args(mut args: &[String]) -> Result<Config, ConfigErrors> {
    if args.len() == 1 {
        return Err(ConfigErrors::NoArgs);
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

        if chars[0] != '-' as u8 {
            break;
        }

        for &c in &chars[1..] {
            match c as char {
                'f' => config.first = true,
                'c' => config.close = true,
                'h' => config.hide_result = true,
                'a' => config.all_users = true,
                _ => return Err(ConfigErrors::FlagNotFound(c as char))
            }
        }

        index += 1;
    }

    config.query = args[index..].join(" ");

    if config.query.is_empty() {
        return Err(ConfigErrors::NoSearchQuery);
    }

    Ok(config)
}
