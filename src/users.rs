use serde::Deserialize;
use crate::config;

#[derive(Clone)]
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
    let url = format!("https://hodis.datasektionen.se/users/{query}");

    let res = match reqwest::get(url).await {
        Ok(res) => res,
        Err(_) => return Err(UsersErrors::URLNotFound),
    };

    let mut users: Vec<HodisUser> = match res.json().await {
        Ok(users) => users,
        Err(_) => return Err(UsersErrors::SearchTimeout),
    };

    // latest year first
    users.sort_unstable_by(|a, b| b.year.cmp(&a.year));

    Ok(users)
}

fn hide_some_users(users: &[HodisUser]) -> &[HodisUser] {
    for (i, user) in users.iter().enumerate() {
        if user.year == 0 {
            return &users[..i];
        }
    }

    users
}

fn display_users(users: &[HodisUser]) {
    for (i, user) in users.iter().enumerate() {
        println!("[{i}] {} ({}), {}", user.display_name, user.uid, user.year);
    }
}

fn show_image(uid: &str) {
    open::that(format!("https://zfinger.datasektionen.se/user/{uid}/image")).unwrap();
}

pub fn select_users(config: &config::Config, mut users: &[HodisUser]) {
    if !config.all_users {
        users = hide_some_users(users);
    }

    if users.is_empty() {
        println!("No students found.");
        return;
    }

    if config.first {
        if !users.is_empty() {
            show_image(&users[0].uid);
        }
    }

    if !config.hide_result {
        display_users(users);
    } else {
        return;
    }

    if config.close {
        return;
    }

    println!("");

    loop {
        println!("Enter number next to student, or (q)uit the program:");

        let mut user_input = String::new();
        std::io::stdin().read_line(&mut user_input).unwrap();
        user_input.pop();

        if user_input == "q" {
            break;
        }

        let selected_num = match user_input.parse::<usize>() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match users.get(selected_num) {
            Some(user) => {
                let url = format!("https://zfinger.datasektionen.se/user/{}/image", user.uid);
                open::that(url).unwrap();
            },
            None => println!("{selected_num} out of range, try another number"),
        }
    }
}
