{
  description = "A tool to download copies of fanfics and upload them to an ereader";

  inputs = {
    utils.url = "github:numtide/flake-utils";
    naersk.url = "github:nmattia/naersk";
  };

  outputs = { self, nixpkgs, utils, naersk }:
    utils.lib.eachDefaultSystem (system: let
      pkgs = nixpkgs.legacyPackages."${system}";
      naersk-lib = naersk.lib."${system}";
    in rec {
      packages.archive_of_amys_own = naersk-lib.buildPackage {
        pname = "archive_of_amys_own";
        root = ./.;
        doCheck = false; # Tests make real requests to AO3 as an integration test
      };
      packages.default = packages.archive_of_amys_own;

      devShells.default = pkgs.mkShell {
        nativeBuildInputs = with pkgs; [
          cargo
          cargo-edit
          cargo-outdated
          clippy
          rustc
          rustfmt
          rust-analyzer
        ];

        RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
      };
    });
}
