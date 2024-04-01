use serde::{Deserialize, Serialize};

const USERS: &[InitialUserData] = &[
    InitialUserData {
        username: "nerdyjoe",
        id: "212041922150137857",
        role: Role::Admin,
    },
    InitialUserData {
        username: "Jasmine",
        id: "89395995351220224",
        role: Role::Admin,
    },
    InitialUserData {
        username: "deathbeam",
        id: "317319692823953409",
        role: Role::LeadResearcher,
    },
    InitialUserData {
        username: "kroIya",
        id: "182893458858442762",
        role: Role::LeadResearcher,
    },
    InitialUserData {
        username: "tikiheme",
        id: "636016169148481547",
        role: Role::Admin,
    },
    InitialUserData {
        username: "Zimzams",
        id: "157685236337999872",
        role: Role::LeadResearcher,
    },
    InitialUserData {
        username: "Elinvynia",
        id: "701549610795008080",
        role: Role::LeadResearcher,
    },
];

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Role {
    Admin,
    LeadResearcher,
    FellowScholar,
    JustBrowsing,
}

impl Role {
    pub fn color(&self) -> &'static str {
        match self {
            Role::Admin => "rgb(52, 152, 219)",
            Role::LeadResearcher => "rgb(46, 204, 113)",
            Role::FellowScholar => "rgb(31, 139, 76)",
            Role::JustBrowsing => "#fff",
        }
    }
}

pub async fn prepare_avatars_ts() -> String {
    let mut user_avatars = vec![];
    let futures = USERS
        .into_iter()
        .map(|user| fetch_user_avatar(user))
        .collect::<Vec<_>>();
    for fut in futures {
        user_avatars.push(fut.await.unwrap());
    }

    let entries = user_avatars
        .into_iter()
        .map(
            |UserAvatar {
                 username,
                 avatar_url,
                 color,
             }| {
                format!(
                    "\t{username}: {{username: '{username}', color: '{color}', url: '{avatar_url}'}},"
                )
            },
        )
        .collect::<Vec<_>>()
        .join("\n");

    format!("export type DiscordUsername = keyof typeof DISCORD_AVATARS;\nexport const DISCORD_AVATARS = {{\n{entries}\n}};")
}

pub async fn fetch_user_avatar(user: &InitialUserData) -> Result<UserAvatar, reqwest::Error> {
    let discord_user: DiscordUser = reqwest::Client::new()
        .get(format!("https://discord.com/api/users/{}", &user.id))
        .header(
            "Authorization",
            format!("Bot {}", std::env::var("DISCORD_TOKEN").unwrap()),
        )
        .send()
        .await?
        .json()
        .await?;

    let avatar_url = discord_user.avatar_url();
    Ok(UserAvatar {
        username: user.username.to_owned(),
        avatar_url,
        color: user.role.color(),
    })
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InitialUserData {
    pub username: &'static str,
    pub id: &'static str,
    pub role: Role,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscordUser {
    pub id: String,
    pub username: String,
    pub avatar: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UserAvatar {
    pub username: String,
    pub avatar_url: String,
    pub color: &'static str,
}

impl DiscordUser {
    pub fn avatar_url(&self) -> String {
        format!(
            "https://cdn.discordapp.com/avatars/{}/{}.webp",
            self.id, self.avatar
        )
    }
}

#[cfg(test)]
mod tests {
    use super::DiscordUser;

    #[test]
    pub fn avatar_url() {
        let avatar = "12321312321312".to_owned();
        let discord_user = DiscordUser {
            id: "1".to_owned(),
            username: "username".to_owned(),
            avatar: avatar.clone(),
        };

        assert_eq!(
            discord_user.avatar_url(),
            format!("https://cdn.discordapp.com/avatars/1/{avatar}.webp")
        );
    }
}
