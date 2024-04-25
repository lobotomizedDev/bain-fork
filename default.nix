{pkgs}: let
  manifest = (pkgs.lib.importTOML ./Cargo.toml).package;
in
  pkgs.rustPlatform.buildRustPackage {
    pname = manifest.name;
    version = manifest.version;
    cargoLock.lockFile = ./Cargo.lock;
    src = pkgs.lib.cleanSource ./.;

    buildInputs = with pkgs; [
      openssl
      cargo
      rustc
    ];

    nativeBuildInputs = with pkgs; [
      pkg-config
    ];
  }
