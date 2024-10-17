{
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
        deps = with pkgs; [
          cargo
          git
        ];
        devDeps = with pkgs; [
          cargo-expand
          clippy
          rustc
          rustfmt
          rust-analyzer
          pre-commit
        ];
      in
      {
        devShells.default = pkgs.mkShell { buildInputs = deps ++ devDeps; };
        formatter = pkgs.nixpkgs-fmt;
      }
    );
}
