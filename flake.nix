{
  inputs = {
    nixpkgs.url = "nixpkgs";
  };

  outputs = { self, nixpkgs }: {
    packages =
      let
        system = "x86_64-linux";
        pkgs = import nixpkgs { inherit system; };
        lib = pkgs.lib;
      in {
        "${system}" = rec {
          toast-lang = pkgs.stdenv.mkDerivation rec {
            pname = "toast-lang";
            version = "0";
            nativeBuildInputs = with pkgs; [
              rustc
              cargo
            ];
          };
          default = toast-lang;
        };
      };
  };
}
