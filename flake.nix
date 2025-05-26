rec {
  description = "Niri window helper daemon and launcher";

  inputs.nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";

  outputs =
    { self, nixpkgs }:
    let
      name = "niri-helper";
      forAllSystems = nixpkgs.lib.genAttrs nixpkgs.lib.platforms.all;
    in
    {
      packages = forAllSystems (
        system:
        let
          pkgs = import nixpkgs { inherit system; };
        in
        {
          default = (pkgs.callPackage ./${name}.nix { }).package;
        }
      );

      homeManagerModules.default =
        {
          config,
          lib,
          pkgs,
          ...
        }:
        {
          options.services.${name} = {
            enable = lib.mkEnableOption "Enable ${name}";
          };

          config = lib.mkIf config.services.${name}.enable {
            assertions = [
              (lib.hm.assertions.assertPlatform "services.${name}" pkgs lib.platforms.linux)
            ];

            home.packages = [ self.packages.${pkgs.system}.default ];

            systemd.user.services.${name} = {
              Unit = {
                Description = description;
                ConditionEnvironment = "NIRI_SOCKET";
                After = [ "graphical-session.target" ];
                Requires = [ "graphical-session.target" ];
              };

              Service = {
                Type = "simple";
                Restart = "always";
                ExecStart = "${lib.getExe self.packages.${pkgs.system}.default} --daemon";
              };

              Install = {
                WantedBy = [ "graphical-session.target" ];
              };
            };
          };
        };
    };
}
