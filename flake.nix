{
  description = "CIM Module - Agent domain for CIM";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
        
        rustVersion = pkgs.rust-bin.nightly.latest.default.override {
          extensions = [ "rust-src" "rust-analyzer" ];
        };

        buildInputs = with pkgs; [
          openssl
          pkg-config
          protobuf
        ] ++ lib.optionals stdenv.isDarwin [
          darwin.apple_sdk.frameworks.Security
          darwin.apple_sdk.frameworks.SystemConfiguration
        ];

        nativeBuildInputs = with pkgs; [
          rustVersion
          cargo-edit
          cargo-watch
        ];
      in
      {
        packages.default = if builtins.pathExists ./Cargo.lock then
          pkgs.rustPlatform.buildRustPackage {
            pname = "cim-domain-agent";
            version = "0.3.0";
            src = ./.;
            cargoLock = { lockFile = ./Cargo.lock; };
            inherit buildInputs nativeBuildInputs;
            checkType = "debug";
            doCheck = false;
          }
        else
          pkgs.runCommand "cim-domain-agent-unbuilt" { } ''
            echo "Cargo.lock missing; skipping package build in flake check" > $out
          '';

        devShells.default = pkgs.mkShell {
          inherit buildInputs nativeBuildInputs;

          shellHook = ''
            echo "CIM Module development environment"
            echo "Rust version: $(rustc --version)"
          '';
        };
      });
}