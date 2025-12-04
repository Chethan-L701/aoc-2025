{
  description = "A very basic flake";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
  };

  outputs =
    { self, nixpkgs }:
    let
      pkgs = import nixpkgs {
        system = "x86_64-linux";
      };
      name = "aoc2025";
    in
    {
      devShells.${pkgs.system}.default = pkgs.mkShell {
        nativeBuildInputs = [
          pkgs.rustc
          pkgs.cargo
          pkgs.rust-analyzer
          pkgs.rustfmt
          pkgs.gdb
        ];
        shellHook = ''
          export NIX_DEV_SHELL=${name}
        '';
      };
    };
}
