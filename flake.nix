{
  description = "A very basic flake";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    {
      self,
      nixpkgs,
      flake-utils,
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs { inherit system; };
      in
      {
        packages.default = pkgs.rustPlatform.buildRustPackage {
          pname = "crabby-vim";
          version = "0.1.0";
          src = ./.;
	  cargoHash = "sha256-s0hk3c/RzR/HmFhFeGI5rV/Dwc9jHNmaD5IDd9kFHYc=";
	  postInstall = ''
	    mkdir $out -p
	    cp $out/lib/libconfig.so $out/config.so
	    rm -r $out/lib
	  '';
        };
      }
    );
}
