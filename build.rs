fn main() {
    tonic_build::configure()
        .type_attribute("User", "#[derive(serde::Serialize, serde::Deserialize)]")
        .type_attribute(
            "Playlist",
            "#[derive(serde::Serialize, serde::Deserialize)]",
        )
        .type_attribute("Song", "#[derive(serde::Serialize, serde::Deserialize)]")
        .compile(&["proto/service.proto"], &["proto"])
        .unwrap_or_else(|e| panic!("Failed to compile protos {:?}", e));
}
