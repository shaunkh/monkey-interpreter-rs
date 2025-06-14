{
  description = "A simple Rust dev shell";
  inputs = {
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  };

  outputs =
    {
      self,
      nixpkgs,
      fenix,
      ...
    }@inputs:
    let
      inherit (nixpkgs) lib;
      forEachSystem = nixpkgs.lib.genAttrs nixpkgs.lib.systems.flakeExposed;
    in
    {
      devShells = forEachSystem (
        system:
        let
          pkgs = import nixpkgs {
            inherit system;
            overlays = [ fenix.overlays.default ];
          };
        in
        {
          default = pkgs.mkShell {
            nativeBuildInputs = [ pkgs.fenix.stable.toolchain ];
          };
        }
      );
    };
}
