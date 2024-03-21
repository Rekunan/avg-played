// For convenience sake, all types can be found in the prelude module
use rosu_v2::prelude::*;
use std::env;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    let client_id: u64 = env::var("CLIENT_ID")
        .expect("CLIENT_ID environment variable not set")
        .parse()
        .expect("CLIENT_ID must be a valid u64");
    let client_secret: String = env::var("CLIENT_SECRET")
        .expect("CLIENT_SECRET environment variable not set");

    let osu: Osu = match Osu::new(client_id, client_secret).await {
        Ok(client) => client,
        Err(why) => panic!(
            "Failed to create client or make initial osu!api interaction: {}",
            why
        ),
    };

    let pp_lb: Rankings = osu
        .performance_rankings(GameMode::Osu)
        .await
        .unwrap_or_else(|why| panic!("Failed to get top 100 players: {}", why));
    
    let mut total_playcount: u32 = 0;

    for entry in pp_lb.ranking.iter() {
        let user: UserExtended = osu
            .user(entry.user_id)
            .await
            .unwrap_or_else(|why| panic!("Failed to get user: {}", why));
        match user.beatmap_playcounts_count {
            Some(playcount) if playcount > 0 => total_playcount += playcount,
            Some(0) => panic!("beatmap_playcounts_count is 0 for user {}", user.user_id),
            None => panic!("beatmap_playcounts_count is not available for user {}", user.user_id),
            _ => (), // Handle other cases if necessary
        }
        sleep(Duration::from_secs(1)).await;
    }
    let average_playcount: f32 = (total_playcount as f32) / (pp_lb.ranking.len() as f32);
    println!("Average beatmap_playcounts_count: {}", average_playcount);
}
