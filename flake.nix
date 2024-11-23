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
        mpris
        cava
        notifd
        apps
      ];

      astalDevLibs = pkgs.lib.map (pkg: pkg.dev) astalLibs;
    in {
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

            # remove repository -> namespace -> constant where name is VERSION
            # this is a workaround for the fact that gir-rs does not support constants with utf-8 characters
            echo "Removing constants with name VERSION"
            xmlstarlet ed -L -d '//_:repository/_:namespace/_:constant[@name="VERSION"]' gir-astal/*.gir

            # Astal-4.0.gir
            echo "Updating Astal-4.0.gir"
            # get constants with *_VERSION as name, and add type attribute
            xmlstarlet ed -L -i '//_:repository/_:namespace/_:constant[@name="MAJOR_VERSION"]' -t attr -n type -v ASTAL_MAJOR_VERSION gir-astal/Astal-4.0.gir
            xmlstarlet ed -L -i '//_:repository/_:namespace/_:constant[@name="MINOR_VERSION"]' -t attr -n type -v ASTAL_MINOR_VERSION gir-astal/Astal-4.0.gir
            xmlstarlet ed -L -i '//_:repository/_:namespace/_:constant[@name="MICRO_VERSION"]' -t attr -n type -v ASTAL_MICRO_VERSION gir-astal/Astal-4.0.gir
            # add repository -> include (name = AstalIO version = 0.1) (make sure it's at the start of the include list)
            xmlstarlet ed -L -i '//_:repository/_:include[1]' -t elem -n include -v ${"''"} gir-astal/Astal-4.0.gir
            xmlstarlet ed -L -i '//_:repository/_:include[1]' -t attr -n name -v AstalIO gir-astal/Astal-4.0.gir
            xmlstarlet ed -L -i '//_:repository/_:include[1]' -t attr -n version -v 0.1 gir-astal/Astal-4.0.gir
            # add shared-library to namespace
            xmlstarlet ed -L -i '//_:repository/_:namespace' -t attr -n shared-library -v libastal-4.so gir-astal/Astal-4.0.gir

            # AstalIO-0.1.gir
            echo "Updating AstalIO-0.1.gir"
            # get constants with *_VERSION as name, and add type attribute
            xmlstarlet ed -L -i '//_:repository/_:namespace/_:constant[@name="MAJOR_VERSION"]' -t attr -n type -v ASTAL_IO_MAJOR_VERSION gir-astal/AstalIO-0.1.gir
            xmlstarlet ed -L -i '//_:repository/_:namespace/_:constant[@name="MINOR_VERSION"]' -t attr -n type -v ASTAL_IO_MINOR_VERSION gir-astal/AstalIO-0.1.gir
            xmlstarlet ed -L -i '//_:repository/_:namespace/_:constant[@name="MICRO_VERSION"]' -t attr -n type -v ASTAL_IO_MICRO_VERSION gir-astal/AstalIO-0.1.gir
            # add shared-library to namespace
            xmlstarlet ed -L -i '//_:repository/_:namespace' -t attr -n shared-library -v libastal-io.so gir-astal/AstalIO-0.1.gir

            # AstalMpris-0.1.gir
            echo "Updating AstalMpris-0.1.gir"
            # get constants with *_VERSION as name, and add type attribute
            xmlstarlet ed -L -i '//_:repository/_:namespace/_:constant[@name="MAJOR_VERSION"]' -t attr -n type -v ASTAL_MPRIS_MAJOR_VERSION gir-astal/AstalMpris-0.1.gir
            xmlstarlet ed -L -i '//_:repository/_:namespace/_:constant[@name="MINOR_VERSION"]' -t attr -n type -v ASTAL_MPRIS_MINOR_VERSION gir-astal/AstalMpris-0.1.gir
            xmlstarlet ed -L -i '//_:repository/_:namespace/_:constant[@name="MICRO_VERSION"]' -t attr -n type -v ASTAL_MPRIS_MICRO_VERSION gir-astal/AstalMpris-0.1.gir
            # add shared-library to namespace
            xmlstarlet ed -L -i '//_:repository/_:namespace' -t attr -n shared-library -v libastal-mpris.so gir-astal/AstalMpris-0.1.gir
            # remove repository -> namespace -> record (name = PlayerClass) -> field (name = appeared) and field (name = closed)
            xmlstarlet ed -L -d '//_:repository/_:namespace/_:record[@name="PlayerClass"]/_:field[@name="appeared"]' gir-astal/AstalMpris-0.1.gir
            xmlstarlet ed -L -d '//_:repository/_:namespace/_:record[@name="PlayerClass"]/_:field[@name="closed"]' gir-astal/AstalMpris-0.1.gir

            # AstalNotifd-0.1.gir
            echo "Updating AstalNotifd-0.1.gir"
            # get constants with *_VERSION as name, and add type attribute
            xmlstarlet ed -L -i '//_:repository/_:namespace/_:constant[@name="MAJOR_VERSION"]' -t attr -n type -v ASTAL_NOTIFD_MAJOR_VERSION gir-astal/AstalNotifd-0.1.gir
            xmlstarlet ed -L -i '//_:repository/_:namespace/_:constant[@name="MINOR_VERSION"]' -t attr -n type -v ASTAL_NOTIFD_MINOR_VERSION gir-astal/AstalNotifd-0.1.gir
            xmlstarlet ed -L -i '//_:repository/_:namespace/_:constant[@name="MICRO_VERSION"]' -t attr -n type -v ASTAL_NOTIFD_MICRO_VERSION gir-astal/AstalNotifd-0.1.gir
            # add shared-library to namespace
            xmlstarlet ed -L -i '//_:repository/_:namespace' -t attr -n shared-library -v libastal-notifd.so gir-astal/AstalNotifd-0.1.gir

            # AstalApps-0.1.gir
            echo "Updating AstalApps-0.1.gir"
            # get constants with *_VERSION as name, and add type attribute
            xmlstarlet ed -L -i '//_:repository/_:namespace/_:constant[@name="MAJOR_VERSION"]' -t attr -n type -v ASTAL_APPS_MAJOR_VERSION gir-astal/AstalApps-0.1.gir
            xmlstarlet ed -L -i '//_:repository/_:namespace/_:constant[@name="MINOR_VERSION"]' -t attr -n type -v ASTAL_APPS_MINOR_VERSION gir-astal/AstalApps-0.1.gir
            xmlstarlet ed -L -i '//_:repository/_:namespace/_:constant[@name="MICRO_VERSION"]' -t attr -n type -v ASTAL_APPS_MICRO_VERSION gir-astal/AstalApps-0.1.gir
            # add shared-library to namespace
            xmlstarlet ed -L -i '//_:repository/_:namespace' -t attr -n shared-library -v libastal-apps.so gir-astal/AstalApps-0.1.gir
        '';
      };

      devShells.default = pkgs.mkShell {
        packages =
          [
            pkgs.nixd
            pkgs.alejandra
            pkgs.pkg-config
            pkgs.glib
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
