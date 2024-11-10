{ config, pkgs, lib, ... }:

with lib;

let
  name = "niri-helper";
  description = "Niri window helper";
  compilation = with pkgs; [
    gcc
    cargo
    rustc
  ];
  cfg = config.services.${name};
in
{
  package = pkgs.rustPlatform.buildRustPackage {
    pname = name;
    version = "0.1.0";

    src = ./.;
    cargoHash = "sha256-w6xB4ulrionZvVoynLu+TK0otLPlF4dBeq9SCbJ5lDU=";

    options.services.${name} = {
      enable = mkEnableOption description;
    };

    config = mkIf cfg.enable {
      assertions = [
        (hm.assertions.assertPlatform "services.${name}" pkgs platforms.linux)
      ];

      systemd.user.services.${name} = {
        Unit = {
          Description = description;
          ConditionalEnvironment = "NIRI_SOCKET";
          After = [ "graphical-session.target" ];
          Requires = [ "niri.service" ];
        };

        Service = {
          Type = "simple";
          Restart = "always";
          ExecStart = pkgs.${name};
        };

        Install.WantedBy = [ cfg.systemTArget ];
      };
    };
  };

  env = pkgs.mkShell {
    name = "env";
    buildInputs = compilation;
  };

}
