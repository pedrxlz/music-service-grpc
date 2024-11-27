# MyMusicService

MyMusicService is a gRPC-based music service that provides APIs to manage users, playlists, and songs. This service is built using Rust and `tonic`.

## Prerequisites

- Rust (latest stable version)
- `protoc` (Protocol Buffers compiler)

## Project Structure

```
├── src
│   ├── main.rs
│   └── data.json
├── proto
│   └── service.proto
├── build.rs
├── Cargo.toml
└── README.md
```

## Setup

1. **Clone the repository**:

   ```sh
   git clone https://github.com/pedrxlz/music-service-grpc.git
   cd mymusicservice
   ```

2. **Install dependencies**:

   Ensure you have Rust installed. If not, install it from [rustup.rs](https://rustup.rs/).

   ```sh
   rustup update
   ```

3. **Install `protoc`**:

   Follow the instructions to install `protoc` from the [Protocol Buffers documentation](https://grpc.io/docs/protoc-installation/).

4. **Build the project**:

   ```sh
   cargo build
   ```

## Running the Service

To run the gRPC server:

```sh
cargo run
```

The server will start and listen on `[::1]:50051`.

## Data File

The `data.json` file should be located in the `src` directory and contain the initial data for users, playlists, and songs. Here is an example structure:

```json
{
  "users": [
    {
      "id": 1,
      "name": "Alice",
      "age": 30,
      "playlists": [1, 2]
    }
  ],
  "playlists": [
    {
      "id": 1,
      "name": "Favorites",
      "songs": [1, 2]
    }
  ],
  "songs": [
    {
      "id": 1,
      "name": "Song 1",
      "artist": "Artist 1"
    }
  ]
}
```

## gRPC API

The service provides the following gRPC APIs:

- `GetUsers`: Retrieve the list of users.
- `GetPlaylists`: Retrieve the list of playlists.
- `GetSongs`: Retrieve the list of songs.
- `GetUserPlaylists`: Retrieve the playlists of a specific user.
- `GetPlaylistSongs`: Retrieve the songs in a specific playlist.
- `GetPlaylistsContainingSong`: Retrieve the playlists containing a specific song.

## Testing

To run the tests:

```sh
cargo test
```

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
