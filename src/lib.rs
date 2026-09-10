use color_eyre::Result;
use eyre::eyre;
use steamworks::Client;

#[derive(serde::Serialize)]
pub struct Achievement {
    pub internal_name: String,
    pub display_name: String,
    pub description: String,
    pub is_hidden: bool,
    pub user_has_obtained: bool,
}

/// Returns an iterator over all Steam achievements registered under the provided Steam app id
pub fn get_achievements(client: &Client) -> impl Iterator<Item = Achievement> {
    let user_stats = client.user_stats();

    user_stats
        .get_achievement_names()
        .unwrap_or_default()
        .into_iter()
        .map(move |internal_name| {
            let achievement_helper = user_stats.achievement(&internal_name);

            let display_name = achievement_helper
                .get_achievement_display_attribute("name")
                .unwrap_or_default();
            let description = achievement_helper
                .get_achievement_display_attribute("desc")
                .unwrap_or_default();
            let is_hidden = achievement_helper
                .get_achievement_display_attribute("hidden")
                .unwrap_or_default()
                == "1";
            let is_obtained = achievement_helper.get().unwrap_or_default();

            Achievement {
                internal_name,
                display_name: display_name.to_string(),
                description: description.to_string(),
                is_hidden,
                user_has_obtained: is_obtained,
            }
        })
}

/// TODO
///
/// # Arguments
///
/// - `client` (`&Client`) - An initialized Steam client.
/// - `internal_name` (`&str`) - The internal name of the achievement.
///
/// # Returns
///
/// - `Result<()>` - Returns Ok if the  
///
/// # Errors
///
/// Describe possible errors.
///
/// # Examples
///
/// ```
/// use crate::...;
///
/// let _ = set_achievement();
/// ```
pub fn set_achievement(client: &Client, internal_name: &str) -> Result<()> {
    let user_stats = client.user_stats();
    let achievement = user_stats.achievement(internal_name);

    let result = achievement
        .set()
        .map_err(|()| eyre!("Failed to set achievement"));

    user_stats
        .store_stats()
        .map_err(|()| eyre!("Failed to store stats"))
        .and_then(|()| result.map(|()| println!("Successfully set achievement")))
}

/// TODO
///
/// # Arguments
///
/// - `client` (`&Client`) - Describe this parameter.
/// - `internal_name` (`&str`) - Describe this parameter.
///
/// # Returns
///
/// - `Result<()>` - Describe the return value.
///
/// # Errors
///
/// Describe possible errors.
///
/// # Examples
///
/// ```
/// use crate::...;
///
/// let _ = clear_achievement();
/// ```
pub fn clear_achievement(client: &Client, internal_name: &str) -> Result<()> {
    let user_stats = client.user_stats();
    let achievement = user_stats.achievement(internal_name);

    let result = achievement
        .clear()
        .map_err(|()| eyre!("Failed to clear achievement"));

    user_stats
        .store_stats()
        .map_err(|()| eyre!("Failed to store stats"))
        .and_then(|()| result.map(|()| println!("Successfully cleared achievement")))
}

/// TODO
///
/// # Arguments
///
/// - `client` (`&Client`) - Describe this parameter.
/// - `internal_name` (`&str`) - Describe this parameter.
///
/// # Returns
///
/// - `Result<bool>` - Describe the return value.
///
/// # Errors
///
/// Describe possible errors.
///
/// # Examples
///
/// ```
/// use crate::...;
///
/// let _ = get_achievement();
/// ```
pub fn get_achievement(client: &Client, internal_name: &str) -> Result<bool> {
    let user_stats = client.user_stats();
    let achievement = user_stats.achievement(internal_name);
    achievement
        .get()
        .map_err(|()| eyre!("Failed to get achievement state"))
}
