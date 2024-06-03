{
  description = "Development flake";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs =
    {
      nixpkgs,
      rust-overlay,
      flake-utils,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { inherit system overlays; };
      in
      with pkgs;
      rec {
        devShells.default = mkShell rec {
          buildInputs = with pkgs; [
            rust-bin.nightly.latest.default
            # NOTE: Required for winit event loop
            libxkbcommon
            libGL
            xorg.libXcursor
            xorg.libXrandr
            xorg.libXi
            xorg.libX11
          ];

          LD_LIBRARY_PATH = "${lib.makeLibraryPath buildInputs}";

          packages = [
            just
            rust-analyzer
            rustfmt
            nixd
            nixfmt-rfc-style
            taplo-lsp
          ];
        };
      }
    );
}
