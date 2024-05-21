
{
  description = "Development flake";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { nixpkgs, rust-overlay, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
      in with pkgs;
      {
        devShells.default = pkgs.mkShell {
          buildInputs = [
            rust-bin.nightly.latest.default
          ];

          packages = [
            bashInteractive
            just
          ];
        };
      });
}
