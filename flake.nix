{
 description = "Actuator Tree";

 inputs = {
   nixpkgs.url = "flake:nixpkgs";
   fenix = {
     url = "github:nix-community/fenix";
     inputs.nixpkgs.follows = "nixpkgs";
   };
 };

 outputs = { self, ... }@inputs:
 let
   system = "x86_64-linux";

   pkgs = inputs.nixpkgs.legacyPackages.${system};

   toolchain = inputs.fenix.packages.${system}.fromToolchainFile {
     file = ./rust-toolchain.toml;
     sha256 = "sha256-Yt/+VH2ih9dDq83dNv0C3MtxJpxxVzWIW+FF0e0aAAw=";
   };
 in
 {
   devShells.${system}.default = pkgs.mkShell {
     buildInputs = [
       toolchain
       pkgs.probe-rs
     ];
   };
 };
}
