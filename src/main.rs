extern crate dotenv;
extern crate futures;
extern crate hex;
extern crate librespot;

use dotenv::dotenv;

use futures::{Async, Future, Poll, Stream};

use librespot::core::authentication::Credentials;
use librespot::core::config::SessionConfig;
use librespot::core::session::Session;
use librespot::core::version;
use librespot::playback::audio_backend;
use librespot::playback::config::{Bitrate, PlayerConfig};
use librespot::playback::player::Player;

use sha1::{Digest, Sha1};

use tokio_core::reactor::{Core, Handle};

struct Main {
    handle: Handle,
    session_config: SessionConfig,
    credentials: Credentials,
    player_config: PlayerConfig
}

impl Main {
    
}

impl Future for Main {
    type Item = ();
    type Error = ();
    
    fn poll(&mut self) -> Poll<(), ()> {
        match Session::connect(self.session_config.clone(), self.credentials.clone(), None, self.handle.clone()).poll() {
            Ok(Async::Ready(session)) => {
                
                let audio_filter = None;
                //let backend = audio_backend::mk_sink::<AlsaSink>;
                //let backend = audio_backend::find(Some(String::from("alsa")));
                let backend = audio_backend::find(Some(String::from("alsa"))).unwrap();
                let device = None;
                
                let (player, event_channel) = Player::new(
                    self.player_config.clone(),
                    session.clone(),
                    audio_filter,
                    move || {
                        (backend)(device)
                    }
                );
                
                //println!("{}", player);
            },
            Ok(Async::NotReady) => eprintln!("Session not ready"),
            Err(err) => eprintln!("Could not connect to server: {}", err)
        };
        
        Ok(Async::NotReady)
    }
}

fn main() {
    dotenv().ok();
    
    let mut core = Core::new().unwrap();
    let handle = core.handle();
    
    let session_config = SessionConfig {
        user_agent: version::version_string(),
        device_id: hex::encode(Sha1::digest("spotifi".as_bytes())),
        proxy: None,
        ap_port: None
    };
    let username = std::env::var("SPOTIFY_USERNAME").expect("SPOTIFY_USERNAME environment variable not set");
    let password = std::env::var("SPOTIFY_PASSWORD").expect("SPOTIFY_PASSWORD environment veriable not set");
    let credentials = Credentials::with_password(username, password);
    
    let player_config = PlayerConfig {
        bitrate: Bitrate::Bitrate320,
        normalisation: true,
        normalisation_pregain: PlayerConfig::default().normalisation_pregain,
        gapless: false
    };
    
    core.run(Main {
        handle: handle,
        session_config: session_config,
        credentials: credentials,
        player_config: player_config
    }).unwrap()
}
