mod constants;
use constants::*;
use rand::Rng;
use serde_json::json;
use twitch_comment_stream::TwitchCommentStream;

#[derive(PartialEq, Clone)]
struct ClipComment {
    body: String
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  let mut stream = TwitchCommentStream::new(TWITCH_CHANNEL.to_string());
  stream.connect().await?;
  let mut clips: Vec<ClipComment> = Vec::new();
 
  while let Ok(comment) = stream.next().await {
    if comment.body.contains(TWITCH_URL) && !clips.contains(&ClipComment { body: comment.body.clone() }) {
      clips.push(ClipComment { body: String::from(&comment.body)});
      let hook = String::from(&comment.body);
      send_webhook(hook).await.unwrap_or_else(|e| println!("{}", e));
    }
    
  }
  Ok(())
}

async fn send_webhook(body: String) -> Result<(), Box<dyn std::error::Error>> {
  // Extract the clip slug from the URL
  let clip_slug = body.split('/').last().unwrap_or("");
  
  // Create the Twitch clips URL
  let clip_url = format!("{}{}", DEFAULT_CLIP_URL, clip_slug);

  let mut rng = rand::thread_rng();
  let title_rng = rng.gen_range(0..PHRASES.len());

  let embedded = json!({
      "content": format!(" \n{}🗸 {} \n🢩❲{}❳🢨 ", PHRASES[title_rng], clip_url, EMBEDDED_FOOTER),
  });

  let client = reqwest::Client::new();
  let _ = client
      .post(WEBHOOK_URL)
      .header("Content-Type", "application/json")
      .body(embedded.to_string())
      .send()
      .await?;
  Ok(())
}