{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  };

  outputs = {
    self,
    nixpkgs,
  }: let
    forAllSystems = function:
      nixpkgs.lib.genAttrs [
        "x86_64-linux"
      ] (system: function nixpkgs.legacyPackages.${system});
  in {
    overlays.default = final: prev: {
      ruin =
        final.callPackage ./default.nix {
        };
    };

    packages = forAllSystems (pkgs: rec {
      ruin =
        pkgs.callPackage ./default.nix {
        };
      default = ruin;
    });

    devShells = forAllSystems (pkgs: {
      default = pkgs.mkShell {
        strictDeps = true;
        nativeBuildInputs = with pkgs; [
          cargo
          rustc
          rust-analyzer-unwrapped
          rustfmt
          clippy
        ];
      };
    });
  };
}
