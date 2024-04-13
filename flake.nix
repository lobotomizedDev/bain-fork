{
  description = "Battery indicator";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = {
    self,
    nixpkgs,
    flake-utils,
    ...
  }:
    flake-utils.lib.eachDefaultSystem (
      system: let
        pkgs = import nixpkgs {
          inherit system;
        };
        rustEnv = pkgs.mkShell {
          buildInputs = with pkgs; [
            rustc
            cargo
            clippy
            pkg-config
            openssl
            rustfmt
            rust-analyzer
            nodejs_21
          ];
        };
      in {
        devShell = rustEnv;
        packages = {
          ruin = pkgs.stdenv.mkDerivation {
            name = "ruin";
            src = ./.;
            buildInputs = with pkgs; [rustc cargo];
            buildPhase = ''
              cargo build --release
            '';
            installPhase = ''
              mkdir -p $out/bin
              cp target/release/ruin $out/bin/
            '';
          };
        };
      }
    );
}
