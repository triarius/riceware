{
  description = "Riceware, a passphrase generator in rust";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";

    crane.url = "github:ipetkov/crane";

    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
      inputs.rust-analyzer-src.follows = "";
    };

    flake-utils.url = "github:numtide/flake-utils";

    advisory-db = {
      url = "github:rustsec/advisory-db";
      flake = false;
    };
  };

  outputs = { self
  , nixpkgs
  , crane
  , fenix
  , flake-utils
  , advisory-db
  , ...
  }: flake-utils.lib.eachDefaultSystem (system:
    let
      pkgs = nixpkgs.legacyPackages.${system};

      inherit (pkgs) lib;

      craneLib = crane.mkLib pkgs;

      fixturesFilter = path: _type: builtins.match ".*/fixtures/.*" path != null;
      fixturesOrCargo = path: type: (fixturesFilter path type) || (craneLib.filterCargoSources path type);

      src = lib.cleanSourceWith { src = craneLib.path ./.; filter = fixturesOrCargo; };

      commonArgs = {
        inherit src;
        strictDeps = true;

        buildInputs = lib.optionals pkgs.stdenv.isDarwin [
          pkgs.libiconv
        ];
      };

      craneLibLLvmTools = craneLib.overrideToolchain
        (fenix.packages.${system}.complete.withComponents [
          "cargo"
          "llvm-tools"
          "rustc"
        ]);

      cargoArtifacts = craneLib.buildDepsOnly commonArgs;

      riceware = craneLib.buildPackage (commonArgs // {
        inherit cargoArtifacts;
        doCheck = false;
      });
    in {
      checks = {
        inherit riceware;

        riceware-clippy = craneLib.cargoClippy (commonArgs // {
          inherit cargoArtifacts;
          cargoClippyExtraArgs = "--all-targets -- --deny warnings --deny clippy::pedantic --deny clippy::all";
        });

        riceware-doc = craneLib.cargoDoc (commonArgs // {
          inherit cargoArtifacts;
        });

        riceware-fmt = craneLib.cargoFmt {
          inherit src;
        };

        riceware-toml-fmt = craneLib.taploFmt {
          src = pkgs.lib.sources.sourceFilesBySuffices src [ ".toml" ];
        };

        riceware-audit = craneLib.cargoAudit {
          inherit src advisory-db;
        };

        # Audit licenses
        riceware-deny = craneLib.cargoDeny {
          inherit src;
        };

        riceware-nextest = craneLib.cargoNextest (commonArgs // {
          inherit cargoArtifacts;
          partitions = 1;
          partitionType = "count";
          cargoNextestPartitionsExtraArgs = "--no-tests=pass";
        });
      };

      packages = {
        default = riceware;
      } // lib.optionalAttrs (!pkgs.stdenv.isDarwin) {
        riceware-llvm-coverage = craneLibLLvmTools.cargoLlvmCov (commonArgs // {
          inherit cargoArtifacts;
        });
      };

      apps.default = flake-utils.lib.mkApp {
        drv = riceware;
      };

      devShells.default = craneLib.devShell {
        checks = self.checks.${system};

        packages = [
          pkgs.taplo
        ];
      };
    }
  );
}
