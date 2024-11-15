{
  description = "Build a cargo project with a custom toolchain";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";

    flake-utils.url = "github:numtide/flake-utils";

    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    astal = {
      url = "github:aylur/astal";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = {
    nixpkgs,
    flake-utils,
    rust-overlay,
    astal,
    ...
  }:
    flake-utils.lib.eachDefaultSystem (system: let
      pkgs = import nixpkgs {
        inherit system;
        overlays = [(import rust-overlay)];
      };
      astalPkgs = astal.packages.${system};

      toolchain = pkgs.rust-bin.stable.latest.default.override {
        extensions = ["rust-src" "rust-analyzer"];
      };

      gir-rs = pkgs.rustPlatform.buildRustPackage rec {
        pname = "gir-rs";
        version = "0.20.3";

        src = pkgs.fetchFromGitHub {
          owner = "gtk-rs";
          repo = "gir";
          rev = version;
          hash = "sha256-R5QNtkr0CvyF2IfC3dC7o2oEOiLvk9qBa2ED9gAXtOY=";
        };

        cargoLock = {
          lockFile = ./gir-Cargo.lock;
          outputHashes = {
            "rustdoc-stripper-0.1.19" = "sha256-QPqDiU8Y1yfoLi5fRvI9Q7YMsAOZ7oywkzAgH8sjCM0=";
          };
        };

        # taken from  https://github.com/NixOS/nixpkgs/blob/nixos-24.05/pkgs/development/tools/gir/default.nix#L24
        postPatch = ''
          rm build.rs
          sed -i '/build = "build\.rs"/d' Cargo.toml
          echo "pub const VERSION: &str = \"$version\";" > src/gir_version.rs
        '';

        meta = {
          description = "Tool to generate rust bindings and user API for glib-based libraries";
          homepage = "https://github.com/gtk-rs/gir";
          license = pkgs.lib.licenses.mit;
          maintainers = with pkgs.lib.maintainers; [];
          mainProgram = "gir-rs";
        };
      };

      # more will be added over time
      astalLibs = with astalPkgs; [
        astal4
        io
      ];

      astalDevLibs = pkgs.lib.map (pkg: pkg.dev) astalLibs;
    in rec {
      # for every astal package, copy the gir files to gir-astal
      # TODO: automatic fixup of gir files
      packages.updateGir = pkgs.writeShellApplication {
        name = "update-gir";
        runtimeInputs = [pkgs.xmlstarlet];
        text = ''
          ${(pkgs.lib.concatLines (pkgs.lib.map (pkg: ''
              cp -r ${pkg}/share/gir-1.0/* gir-astal
              chmod -R u+w gir-astal
            '')
            astalDevLibs))}

          xmlstarlet ed -L -u '/repository/@version' -v '4.0' gir-astal/Astal-4.0.gir
        '';
      };

      devShells.default = pkgs.mkShell {
        packages =
          [
            pkgs.nixd
            pkgs.alejandra
            pkgs.pkg-config
            packages.updateGir
            toolchain
            gir-rs

            # for testing/examples
            pkgs.gtk4
            pkgs.blueprint-compiler
          ]
          ++ astalLibs;
      };
    });
}
