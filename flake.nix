{
  description = "A partial clone of the Prometheus node exporter written in Rust";

  inputs.nixpkgs.url = "nixpkgs/nixos-unstable";
  inputs.flake-utils.url = "github:numtide/flake-utils";

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
      in
      {
        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            prometheus-node-exporter
            rustup

            # Keep this line if you use bash.
            bashInteractive
          ];

          shellHook = ''
            export CARGO_HOME="$(pwd)/.cargo"
            export RUSTUP_HOME="$(pwd)/.rustup"
            export PATH="$CARGO_HOME/bin:$PATH"

            rustup toolchain install --component rust-analyzer,rust-src --no-self-update stable
            rustup default stable
            cargo install cross --git https://github.com/cross-rs/cross
          '';
        };
      });
}
