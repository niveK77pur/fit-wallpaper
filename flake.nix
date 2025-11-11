{
  description = "Flake for the background utility";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
  };

  outputs = {nixpkgs, ...}: let
    inherit (nixpkgs) lib;
    forEachSystem = lib.genAttrs lib.systems.flakeExposed;
  in {
    packages = forEachSystem (system: rec {
      pkgs = import nixpkgs {inherit system;};
      default = pkgs.callPackage ({rustPlatform}: let
        cargo-toml = builtins.fromTOML (builtins.readFile ./Cargo.toml);
      in
        rustPlatform.buildRustPackage (finalAttrs: {
          pname = cargo-toml.package.name;
          inherit (cargo-toml.package) version;

          src = ./.;

          cargoHash = "sha256-Odv9yJrQovA+2csSmr9Mq6cF9Aq3BOFrNS+74T2F4ls=";

          meta = {
            mainProgram = cargo-toml.package.name;
          };
        })) {};
    });
  };
}
