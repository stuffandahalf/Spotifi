extern crate rspotify;

use rspotify::client::Spotify;
use rspotify::oauth2::SpotifyClientCredentials;
use rspotify::senum::Country;

fn main() {
    //println!("Hello, world!");
    let client_credential = SpotifyClientCredentials::default().build();

    let spotify = Spotify::default()
        .client_credentials_manager(client_credential)
        .build();

    println!("{:?}", spotify);
}
