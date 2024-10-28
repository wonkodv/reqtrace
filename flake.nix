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

        packages.default = pkgs.mkDerivation {
          pname = "reqtrace";
          version = "0.1.0";
          src = ./.;

          buildInputs = buildDeps;
        };

        devShells.default = pkgs.mkShell { buildInputs = deps ++ buildDeps; };

        checks = {
          test =
            pkgs.runCommand "test"
              {
                nativeBuildInputs = [ pkgs.cargo ];
                src = self;
              }
              ''
                cargo test
              '';
        };

        #       checks = {
        #         lint =
        #           pkgs.runCommand "lint"
        #             {
        #               nativeBuildInputs = [
        #                 pkgs.cargo
        #                 pkgs.clippy
        #               ];
        #               src = ./.;
        #             }
        #             ''
        #               cargo clippy
        #               mkdir $out
        #             '';
        #
        ##         test =
        #           pkgs.runCommand "test"
        #             {
        #               nativeBuildInputs = [ pkgs.cargo ];
        #               src = ./.;
        #             }
        #             ''
        #               cargo test
        #             '';
        #
        #         tmx = pkgs.runCommand "test"
        #             {
        #               nativeBuildInputs = [ pkgs.cargo ];
        #               src = ./.;
        #             }
        #             ''
        #               cargo run tmx
        #             '';
        #
        #         format =
        #           pkgs.runCommand "format"
        #             {
        #               nativeBuildInputs = [
        #                 pkgs.nixfmt-rfc-style
        #                 pkgs.cargo
        #                 pkgs.rustfmt
        #               ];
        #               src = ./.;
        #             }
        #             ''
        #               nixfmt *.nix
        #               cargo fmt
        #             '';
        #       };
      }
    );
}
