{
  lib,
  rustPlatform,
}:
rustPlatform.buildRustPackage rec {
  pname = "rp-fancontrol";
  version = "v0.1.0";

  src = ./.;

  cargoHash = "sha256-Y9JwURaVsFw6Dmle7A1PWwoxJrjqiGTgTh0xzSz9d9M=";

  meta = with lib; {
    description = "A controller for raspberry fans connected to the PWM GPIO pins";
    homepage = "https://github.com/jonboh/raspi-fancontrol";
    license = licenses.mit;
    maintainers = ["jonboh"];
  };
}
