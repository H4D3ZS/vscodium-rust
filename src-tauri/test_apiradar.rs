use reqwest::Client;

#[tokio::main]
async fn main() {
    println!("Testing APIRadar");
    let resp = reqwest::get("https://apiradar.live/api/leaks?limit=50").await;
    match resp {
        Ok(res) => {
            let body = res.text().await.unwrap();
            println!("Length of body: {}", body.len());
            if body.len() < 1000 {
                println!("Response: {}", body);
            }
        },
        Err(e) => println!("Error: {}", e)
    }
}
