{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-23.05";
    nixpkgs-mozilla.url = "github:mozilla/nixpkgs-mozilla/master";
  };

  outputs = { nixpkgs, nixpkgs-mozilla, ... }:
    let
      inherit (nixpkgs) lib;

      makePackage = (system: dev:
        let
          pkgs = import nixpkgs {
            inherit system;
            overlays = [ nixpkgs-mozilla.overlays.rust ];
          };

          rustPlatform =
            let
              rust = (pkgs.rustChannelOf {
                channel = "1.70.0";
                sha256 = "sha256-gdYqng0y9iHYzYPAdkC/ka3DRny3La/S5G8ASj0Ayyc=";
              }).rust.override {
                extensions = if dev then [ "rust-src" ] else [ ];
              };
            in
            pkgs.makeRustPlatform {
              cargo = rust;
              rustc = rust;
            };
        in
        rec {
          web = pkgs.buildNpmPackage {
            name = "wifi-visualizer-web";
            src = lib.cleanSourceWith {
              src = ./.;
              filter = path: type:
                let
                  baseName = builtins.baseNameOf (builtins.toString path);
                in
                (
                  baseName == "package.json"
                  || baseName == "package-lock.json"
                );
            };
            npmDepsHash = "sha256-KA2+bVrEANMejMhBhsy1VsXkMTBygYT7kh3YVdTA/m4=";
            dontNpmBuild = true;

            nativeBuildInputs = with pkgs; [
              python3
            ];
          };

          default = rustPlatform.buildRustPackage {
            name = "wifi-visualizer";
            src = lib.cleanSourceWith rec {
              src = ./.;
              filter = path: type:
                lib.cleanSourceFilter path type
                && (
                  let
                    baseName = builtins.baseNameOf (builtins.toString path);
                    relPath = lib.removePrefix (builtins.toString ./.) (builtins.toString path);
                  in
                  lib.any (re: builtins.match re relPath != null) [
                    "/\.cargo"
                    "/\.cargo/.*"
                    "/build.rs"
                    "/build.ts"
                    "/Cargo.lock"
                    "/Cargo.toml"
                    "/dist"
                    "/dist/.*"
                    "/package-lock.json"
                    "/package.json"
                    "/src"
                    "/src/.*"
                    "/types"
                    "/types/.*"
                    "/web"
                    "/web/.*"
                    "/tsconfig.json"
                  ]
                );
            };

            preConfigure = ''
              ln -s ${web}/lib/node_modules/wifi-visualizer-web/node_modules ./node_modules
              stat ./node_modules/
            '';

            cargoLock = {
              lockFile = ./Cargo.lock;
              outputHashes = {
                "ieee80211-0.1.0" = "sha256-GCy3t+N9Gz3j6X8zgfgfEsKbWhmfYUE1yZQ28MGzXEM=";
                "nodejs-bundler-3.0.0" = "sha256-B0Rj8npZ2YM7uh1eW+CSxbE8QD+8jNV8+NuGVSb3BBM=";
                "pcap-0.7.0" = "sha256-7cVw+GGF00DjP4rtuPmczuS5wh+NwqtuXmAD6luhGns=";
                "pcap-sys-0.1.0" = "sha256-i+O3xL1GPH+WnB8ZCoLX0WOOj3HWLuRkKmIuC8G4BsQ=";
              };
            };

            nativeBuildInputs = with pkgs; [
              nodejs
              pkg-config
              rustPlatform.bindgenHook
            ];

            buildInputs = with pkgs; [
              # glib
              # openssl
              libpcap
            ];

            doCheck = false;
          };
        }
      );
    in
    builtins.foldl' lib.recursiveUpdate { } (builtins.map
      (system: {
        devShells.${system} = makePackage system true;
        packages.${system} = makePackage system false;
      })
      lib.systems.flakeExposed);
}
