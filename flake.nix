{
  description = "Universal Language Connector - LSP-based universal plugin architecture";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };

        rustToolchain = pkgs.rust-bin.stable.latest.default.override {
          extensions = [ "rust-src" "rust-analyzer" ];
        };

        nativeBuildInputs = with pkgs; [
          rustToolchain
          pkg-config
          just
          nodejs_20
        ];

        buildInputs = with pkgs; [
          openssl
        ] ++ lib.optionals stdenv.isDarwin [
          darwin.apple_sdk.frameworks.Security
          darwin.apple_sdk.frameworks.CoreFoundation
        ];

      in
      {
        # Development shell
        devShells.default = pkgs.mkShell {
          inherit buildInputs nativeBuildInputs;

          shellHook = ''
            echo "🚀 Universal Language Connector Development Environment"
            echo ""
            echo "Rust toolchain: $(rustc --version)"
            echo "Cargo: $(cargo --version)"
            echo "Just: $(just --version)"
            echo "Node.js: $(node --version)"
            echo ""
            echo "Available commands:"
            echo "  just build    - Build server"
            echo "  just test     - Run tests"
            echo "  just dev      - Development server with auto-reload"
            echo "  just validate - RSR compliance check"
            echo ""
            echo "For full list: just --list"
            echo ""
          '';

          RUST_SRC_PATH = "${rustToolchain}/lib/rustlib/src/rust/library";
          RUST_LOG = "info";
        };

        # Package the server
        packages.default = pkgs.rustPlatform.buildRustPackage {
          pname = "universal-connector-server";
          version = "0.1.0";

          src = ./server;

          cargoLock = {
            lockFile = ./server/Cargo.lock;
            outputHashes = {
              # Add any git dependencies here if needed
            };
          };

          nativeBuildInputs = nativeBuildInputs;
          buildInputs = buildInputs;

          meta = with pkgs.lib; {
            description = "LSP-based universal plugin architecture for document conversion";
            homepage = "https://github.com/hyperpolymath/universal-language-server-plugin";
            license = with licenses; [ mit agpl3Plus ];
            maintainers = [ ];
            mainProgram = "universal-connector-server";
          };
        };

        # Server application
        apps.default = {
          type = "app";
          program = "${self.packages.${system}.default}/bin/universal-connector-server";
        };

        # Development tools shell
        devShells.tools = pkgs.mkShell {
          buildInputs = with pkgs; [
            rustToolchain
            cargo-watch
            cargo-audit
            cargo-outdated
            cargo-edit
            rust-analyzer
            just
            nodejs_20
            podman
            podman-compose
          ];

          shellHook = ''
            echo "🔧 Development Tools Shell"
            echo ""
            echo "Additional tools available:"
            echo "  cargo watch   - Auto-rebuild on changes"
            echo "  cargo audit   - Security vulnerability scanning"
            echo "  cargo outdated - Check outdated dependencies"
            echo "  cargo edit    - Manage dependencies"
            echo ""
          '';
        };

        # CI/CD shell
        devShells.ci = pkgs.mkShell {
          buildInputs = nativeBuildInputs ++ buildInputs ++ (with pkgs; [
            cargo-tarpaulin  # Code coverage
            clippy           # Linter
          ]);

          shellHook = ''
            echo "🤖 CI/CD Shell"
            echo "Running checks..."
            just ci
          '';
        };

        # Docker image build
        packages.dockerImage = pkgs.dockerTools.buildLayeredImage {
          name = "universal-connector";
          tag = "latest";

          contents = [ self.packages.${system}.default ];

          config = {
            Cmd = [ "/bin/universal-connector-server" ];
            ExposedPorts = {
              "8080/tcp" = {};
              "8081/tcp" = {};
            };
            Env = [
              "HTTP_ADDR=0.0.0.0:8080"
              "WS_ADDR=0.0.0.0:8081"
              "RUST_LOG=info"
            ];
            User = "1000:1000";
          };
        };

        # Formatter
        formatter = pkgs.nixpkgs-fmt;

        # Checks
        checks = {
          # Build check
          build = self.packages.${system}.default;

          # Test check
          test = pkgs.runCommand "test-universal-connector" {
            buildInputs = nativeBuildInputs ++ buildInputs;
          } ''
            cd ${./server}
            cargo test
            touch $out
          '';

          # Lint check
          lint = pkgs.runCommand "lint-universal-connector" {
            buildInputs = nativeBuildInputs ++ buildInputs;
          } ''
            cd ${./server}
            cargo clippy -- -D warnings
            touch $out
          '';

          # Format check
          format = pkgs.runCommand "format-universal-connector" {
            buildInputs = nativeBuildInputs ++ buildInputs;
          } ''
            cd ${./server}
            cargo fmt --check
            touch $out
          '';
        };
      }
    );
}
