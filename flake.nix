{
  description = "Bharat Basha Prachar Sabha — Mother Tongue Learning Platform";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    rust-overlay.inputs.nixpkgs.follows = "nixpkgs";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { inherit system overlays; };

        rustToolchain = pkgs.rust-bin.stable.latest.default.override {
          extensions = [ "rust-src" "rust-analyzer" "clippy" "rustfmt" ];
        };
      in
      {
        devShells.default = pkgs.mkShell {
          name = "bbps-dev";

          buildInputs = with pkgs; [
            # ── Rust ──────────────────────────────────────
            rustToolchain
            cargo-watch
            cargo-nextest
            diesel-cli

            # ── Node.js (web frontend) ───────────────────
            nodejs_20
            nodePackages.npm

            # ── Python (AI/ML service) ───────────────────
            python311
            python311Packages.pip
            python311Packages.virtualenv

            # ── Databases ────────────────────────────────
            postgresql_16
            redis

            # ── Build dependencies ───────────────────────
            pkg-config
            openssl
            openssl.dev

            # ── Tools ────────────────────────────────────
            just
            docker-compose
            jq
            curl
            watchexec
          ];

          nativeBuildInputs = with pkgs; [
            pkg-config
          ];

          # Ensure Diesel can find libpq
          LIBPQ_DIR = "${pkgs.postgresql_16.lib}";
          PQ_LIB_DIR = "${pkgs.postgresql_16.lib}/lib";

          # Rust build flags
          RUST_BACKTRACE = "1";
          RUST_LOG = "info,tower_http=debug";

          shellHook = ''
            echo ""
            echo "  ╔══════════════════════════════════════════════════╗"
            echo "  ║  Bharat Basha Prachar Sabha — Dev Environment   ║"
            echo "  ╠══════════════════════════════════════════════════╣"
            echo "  ║  Rust:    $(rustc --version | cut -d' ' -f2)                            ║"
            echo "  ║  Node:    $(node --version)                            ║"
            echo "  ║  Python:  $(python3 --version | cut -d' ' -f2)                            ║"
            echo "  ║  Just:    $(just --version | cut -d' ' -f2)                           ║"
            echo "  ╠══════════════════════════════════════════════════╣"
            echo "  ║  Run 'just' to see available commands            ║"
            echo "  ║  Run 'just infra-up' to start PostgreSQL + Redis ║"
            echo "  ╚══════════════════════════════════════════════════╝"
            echo ""

            # Set default env vars if .env doesn't exist
            if [ ! -f .env ]; then
              cp .env.example .env 2>/dev/null || true
            fi

            # Source .env if present
            if [ -f .env ]; then
              set -a
              source .env
              set +a
            fi
          '';
        };
      }
    );
}
