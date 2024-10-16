{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";
  };

  outputs =
    { self
    , nixpkgs
    }:
    let
      system = "x86_64-linux";
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
      devShells.${system}.default = pkgs.mkShell {
        buildInputs = deps ++ devDeps;
      };
    };
}
