{
  description = "Blue Blood Sports: College Hoops";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, rust-overlay }:
    let
      system = "x86_64-linux";
      pkgs = import nixpkgs {
        inherit system;
        overlays = [ rust-overlay.overlays.default ];
      };

      rustToolchain = pkgs.rust-bin.stable.latest.default.override {
        extensions = [ "rust-src" "clippy" "rustfmt" "rust-analyzer" ];
      };

    in
    {
      devShells.${system}.default = pkgs.mkShell {
        buildInputs = [
          rustToolchain

          # iced uses wgpu/Vulkan for rendering on Linux
          pkgs.pkg-config
          pkgs.vulkan-loader
          pkgs.vulkan-headers

          # Wayland + X11 windowing support (iced targets both)
          pkgs.wayland
          pkgs.libxkbcommon
          pkgs.libx11
          pkgs.libxcursor
          pkgs.libxrandr
          pkgs.libxi

        ];

        shellHook = ''
          # winit/wgpu dlopen these at runtime — they must be on LD_LIBRARY_PATH
          # even though they are in buildInputs (Nix doesn't auto-add them).
          export LD_LIBRARY_PATH="${pkgs.vulkan-loader}/lib:${pkgs.wayland}/lib:${pkgs.libxkbcommon}/lib:$LD_LIBRARY_PATH"

          alias z=cd
          alias fd=find
          alias eza=ls
          alias vi=nvim
          alias yz=yazi
        '';
      };
    };
}
