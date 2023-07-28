{ pkgs, config, lib, ... }:
let
  cfg = config.dd-ix.foundation;
in
{
  options.dd-ix.foundation = with lib; {
    enable = mkOption {
      type = types.bool;
      default = false;
      description = ''Wether to enable foundation service'';
    };
    http = {
      host = mkOption {
        type = types.str;
        default = "0.0.0.0";
        description = ''
          To which IP foundation should bind.
        '';
      };
      port = mkOption {
        type = types.port;
        default = 8080;
        description = ''
          To which port should foundation bind.
        '';
      };
    };
    user = mkOption {
      type = types.str;
      default = "foundation";
      description = ''systemd user'';
    };
    group = mkOption {
      type = types.str;
      default = "foundation";
      description = ''group of systemd user'';
    };
    logLevel = mkOption {
      type = types.str;
      default = "info";
      description = ''log level of the application'';
    };
    url = mkOption {
      type = types.str;
      description = ''under which domain foundation serves its content'';
    };
  };

  config = lib.mkIf cfg.enable {
    systemd = {
      services = {
        "foundation" = {
          enable = true;
          wantedBy = [ "multi-user.target" ];

          script = ''
            exec ${pkgs.foundation}/bin/foundation&
          '';

          environment = {
            "RUST_LOG" = "${cfg.logLevel}";
            "RUST_BACKTRACE" = if (cfg.logLevel == "info") then "0" else "1";
            "FOUNDATION_LISTEN_ADDR" = "${cfg.http.host}:${toString cfg.http.port}";
            "FOUNDATION_CONTENT_DIRECTORY" = "${pkgs.website-content}/content/";
            "FOUNDATION_BASE_URL" = "${cfg.url}";
          };

          serviceConfig = {
            Type = "forking";
            User = cfg.user;
            Restart = "always";
          };
        };
      };
    };

    # user accounts for systemd units
    users.users."${cfg.user}" = {
      name = "${cfg.user}";
      description = "This guy runs foundation";
      isNormalUser = false;
      isSystemUser = true;
      group = cfg.group;
      uid = 1503;
    };
    users.groups."foundation" = {};
  };
}
