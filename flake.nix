{
  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

  outputs =
    { nixpkgs, ... }:
    let
      forEachSystem =
        function:
        nixpkgs.lib.genAttrs [ "aarch64-darwin" "x86_64-linux" ] (
          system: function nixpkgs.legacyPackages.${system}
        );
    in
    {
      devShells = forEachSystem (pkgs: {
        default = pkgs.mkShell {
          packages = [
            pkgs.cargo
            pkgs.clippy
            pkgs.rust-analyzer
            pkgs.rustc
            pkgs.rustfmt
          ];
          env.RUST_SRC_PATH = "${pkgs.rustPlatform.rustLibSrc}";
          env.RUSTC_WRAPPER = "${pkgs.sccache}/bin/sccache";
          env.SCCACHE_SERVER_PORT = toString (4200 + pkgs.lib.toInt (pkgs.lib.versions.minor pkgs.sccache.version));
          env.SCCACHE_CACHE_SIZE = "50G";
        };
      });
    };
}
