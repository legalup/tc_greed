{ lib
, llvmPackages_11
, cmake
, spdlog
, abseil-cpp }:

llvmPackages_11.stdenv.mkDerivation rec {
  pname = "codeforces";
  version = "0.1.0";
  
  src = ./.;

  nativeBuildInputs = [ cmake ];
  buildInputs = [ spdlog abseil-cpp ];

  cmakeFlags = [
    "-DENABLE_TESTING=OFF"
  ];

  meta = with lib; {
    description = ''
      codeforces setup.";
    '';
    license = licenses.mit;
    platforms = with platforms; linux ++ darwin;
    maintainers = [ maintainers.breakds ];    
  };
}
