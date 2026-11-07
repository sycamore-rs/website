{
  description = "Dev shell for Sycamore Website";
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    {
      self,
      nixpkgs,
      flake-utils,
      ...
    }@inputs:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
      in
      {
        devShells.default = pkgs.mkShell {
          packages = [ pkgs.tailwindcss_4 ];
          buildInputs = [
            pkgs.pkg-config
            pkgs.openssl
          ];

          DOCS_DIR = "checkouts/main/docs";
        };
      }
    );
}
