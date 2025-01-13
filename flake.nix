{

  description = "A slim Requirement Tracer";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils?ref=c1dfcf08411b08f6b8615f7d8971a2bfa81d5e8a";
  };

  outputs =
    {
      self,
      nixpkgs,
      flake-utils,
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
        buildDeps = with pkgs; [
          cargo
          rustc
        ];
        deps = with pkgs; [
          cargo-expand
          gdb
          clippy
          rustfmt
          rust-analyzer
          pre-commit
          ninja
          jq
        ];
      in
      {

        packages.default = pkgs.rustPlatform.buildRustPackage {
          pname = "reqtrace";
          version = "0.2.0";  # keep in synch with Cargo.toml version
          src = ./.;

          cargoLock = {
            lockFile = ./Cargo.lock;
          };

          buildInputs = buildDeps;

          meta = with pkgs.lib; {
            description = "A slim Requirement Tracer";
            homepage = "https://github.com/wonkodv/reqtrace";
          };
        };

        devShells.default = pkgs.mkShell { buildInputs = deps ++ buildDeps; };

      }
    );
}
