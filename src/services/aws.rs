use rusoto_gamelift::GameLiftClient;
use rusoto_core::credential::{EnvironmentProvider};
use rusoto_core::request::HttpClient;
use rusoto_core::region::Region;


pub async fn get_gamelift_client() -> GameLiftClient {
    let cred = EnvironmentProvider::default();
    let client = HttpClient::new().unwrap();
    GameLiftClient::new_with(client, cred, Region::EuWest1)
}