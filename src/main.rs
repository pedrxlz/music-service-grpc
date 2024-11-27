use service::{Playlist, Song, User};
use tonic::{transport::Server, Request, Response, Status};

use serde::{Deserialize, Serialize};
use std::fs;

pub mod service {
    tonic::include_proto!("service");
}

#[derive(Debug, Default)]
pub struct MyMusicService {}

#[derive(Serialize, Deserialize)]
struct Data {
    users: Vec<User>,
    playlists: Vec<Playlist>,
    songs: Vec<Song>,
}

#[tonic::async_trait]
impl service::music_service_server::MusicService for MyMusicService {
    async fn get_users(&self, _: Request<()>) -> Result<Response<service::UserList>, Status> {
        let data = load_data();
        let users = data
            .users
            .into_iter()
            .map(|u| service::User {
                id: u.id,
                name: u.name,
                age: u.age,
                playlists: u.playlists,
            })
            .collect();
        Ok(Response::new(service::UserList { users }))
    }

    async fn get_playlists(
        &self,
        _: Request<()>,
    ) -> Result<Response<service::PlaylistList>, Status> {
        let data = load_data();
        let playlists = data
            .playlists
            .into_iter()
            .map(|p| service::Playlist {
                id: p.id,
                name: p.name,
                songs: p.songs,
            })
            .collect();
        Ok(Response::new(service::PlaylistList { playlists }))
    }

    async fn get_songs(&self, _: Request<()>) -> Result<Response<service::SongList>, Status> {
        let data = load_data();
        let songs = data
            .songs
            .into_iter()
            .map(|s| service::Song {
                id: s.id,
                name: s.name,
                artist: s.artist,
            })
            .collect();
        Ok(Response::new(service::SongList { songs }))
    }

    async fn get_user_playlists(
        &self,
        request: Request<service::UserRequest>,
    ) -> Result<Response<service::PlaylistList>, Status> {
        let data = load_data();
        let user_id = request.into_inner().user_id;
        let user = data.users.iter().find(|u| u.id == user_id);
        let playlists = if let Some(user) = user {
            data.playlists
                .iter()
                .filter(|p| user.playlists.contains(&p.id))
                .map(|p| service::Playlist {
                    id: p.id,
                    name: p.name.clone(),
                    songs: p.songs.clone(),
                })
                .collect()
        } else {
            vec![]
        };
        Ok(Response::new(service::PlaylistList { playlists }))
    }

    async fn get_playlist_songs(
        &self,
        request: Request<service::PlaylistRequest>,
    ) -> Result<Response<service::SongList>, Status> {
        let data = load_data();
        let playlist_id = request.into_inner().playlist_id;
        let playlist = data.playlists.iter().find(|p| p.id == playlist_id);
        let songs = if let Some(playlist) = playlist {
            data.songs
                .iter()
                .filter(|s| playlist.songs.contains(&s.id))
                .map(|s| service::Song {
                    id: s.id,
                    name: s.name.clone(),
                    artist: s.artist.clone(),
                })
                .collect()
        } else {
            vec![]
        };
        Ok(Response::new(service::SongList { songs }))
    }

    async fn get_playlists_containing_song(
        &self,
        request: Request<service::SongRequest>,
    ) -> Result<Response<service::PlaylistList>, Status> {
        let data = load_data();
        let song_id = request.into_inner().song_id;
        let playlists = data
            .playlists
            .iter()
            .filter(|p| p.songs.contains(&song_id))
            .map(|p| service::Playlist {
                id: p.id,
                name: p.name.clone(),
                songs: p.songs.clone(),
            })
            .collect();
        Ok(Response::new(service::PlaylistList { playlists }))
    }
}

fn load_data() -> Data {
    let data = fs::read_to_string("src/data.json").expect("Unable to read file");
    serde_json::from_str(&data).expect("JSON was not well-formatted")
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let music_service = MyMusicService::default();

    println!("Servidor gRPC rodando em {}", addr);

    Server::builder()
        .add_service(service::music_service_server::MusicServiceServer::new(
            music_service,
        ))
        .serve(addr)
        .await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use service::music_service_server::MusicService;
    use tonic::Request;

    #[tokio::test]
    async fn test_get_users() {
        let service = MyMusicService::default();
        let request = Request::new(());
        let response = service.get_users(request).await.unwrap();
        let user_list = response.into_inner();
        assert_eq!(user_list.users.len(), 3);
    }

    #[tokio::test]
    async fn test_get_playlists() {
        let service = MyMusicService::default();
        let request = Request::new(());
        let response = service.get_playlists(request).await.unwrap();
        let playlist_list = response.into_inner();
        assert_eq!(playlist_list.playlists.len(), 3);
    }

    #[tokio::test]
    async fn test_get_songs() {
        let service = MyMusicService::default();
        let request = Request::new(());
        let response = service.get_songs(request).await.unwrap();
        let song_list = response.into_inner();
        assert_eq!(song_list.songs.len(), 8);
    }

    #[tokio::test]
    async fn test_get_user_playlists() {
        let service = MyMusicService::default();
        let request = Request::new(service::UserRequest { user_id: 1 });
        let response = service.get_user_playlists(request).await.unwrap();
        let playlist_list = response.into_inner();
        assert_eq!(playlist_list.playlists.len(), 2);
    }

    #[tokio::test]
    async fn test_get_playlist_songs() {
        let service = MyMusicService::default();
        let request = Request::new(service::PlaylistRequest { playlist_id: 1 });
        let response = service.get_playlist_songs(request).await.unwrap();
        let song_list = response.into_inner();
        assert_eq!(song_list.songs.len(), 3);
    }

    #[tokio::test]
    async fn test_get_playlists_containing_song() {
        let service = MyMusicService::default();
        let request = Request::new(service::SongRequest { song_id: 1 });
        let response = service
            .get_playlists_containing_song(request)
            .await
            .unwrap();
        let playlist_list = response.into_inner();
        assert_eq!(playlist_list.playlists.len(), 1
    );
    }
}
