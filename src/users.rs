use serde::Deserialize;
use crate::config;

#[derive(Debug)]
#[allow(dead_code)]
#[derive(Deserialize)]
pub struct HodisUser {
    #[serde(rename = "givenName")]
    pub given_name: String,
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[serde(rename = "ugKthid")]
    pub ug_kthid: String,
    pub uid: String,
    pub cn: String,
    pub mail: String,
    pub year: i32,
    pub tag: String,
}

#[derive(Debug)]
pub enum UsersErrors {
    URLNotFound,
    SearchTimeout,
}

pub async fn get_users(query: &str) -> Result<Vec<HodisUser>, UsersErrors> {
    let url = format!("https://hodis.datasektionen.se/users/{}", query);

    let res = reqwest::get(url).await.or_else(|_| {
        return Err(UsersErrors::URLNotFound);
    }).unwrap();

    let mut users: Vec<HodisUser> = res.json().await.or_else(|_| {
        return Err(UsersErrors::SearchTimeout);
    }).unwrap();

    users = users.into_iter().filter(|u| u.year != 0).collect();
    // latest year first
    users.sort_unstable_by(|a, b| b.year.cmp(&a.year));

    Ok(users)
}

fn display_users(users: &Vec<HodisUser>) {
    for (i, user) in users.iter().enumerate() {
        println!("{i}) {}, {}, {}, {}", user.display_name, user.uid, user.year, user.tag);
    }
}

fn show_image(uid: &str) {
    open::that(format!("https://zfinger.datasektionen.se/user/{uid}/image")).unwrap();
}

pub fn select_users(config: &config::Config, users: &Vec<HodisUser>) {
    if !config.hide_users {
        display_users(users);
    }

    if config.first {
        show_image(&users[0].uid);
    }

    if config.close {
        return;
    }

    println!("");

    loop {
        println!("Enter number next to user, or q to quit the program: ");

        let mut user_input = String::new();
        let stdin = std::io::stdin(); // We get `Stdin` here.
        stdin.read_line(&mut user_input).unwrap();
        user_input = user_input[..user_input.len() - 1].to_string();

        if user_input == "q" {
            break;
        }

        match user_input.parse::<usize>() {
            Ok(num) => {
                if let Some(user) = users.get(num) {
                    open::that(format!("https://zfinger.datasektionen.se/user/{}/image", user.uid)).unwrap();
                } else {
                    println!("{num} out of range, try another number...");
                }
            },
            Err(_) => {}
        }
    }
}