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
        default = "http://127.0.0.1";
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
    listmonk = {
      host = mkOption {
        type = types.str;
        default = "http://127.0.0.1";
        description = ''
          At which host is listmonk listening
        '';
      };
      port = mkOption {
        type = types.port;
        default = 8081;
        description = ''
          At which port is listmonk listening
        '';
      };
      user = mkOption {
        type = types.str;
        default = "admin";
        description = ''
          username of listmonk user
        '';
      };
      passwordFile = mkOption {
        type = types.path;
        description = ''
          path from where the user password can be read
        '';
      };
      allowed_lists = mkOption {
        type = types.listOf types.int;
        default = [ ];
        description = ''
          list of allowed mailing list ids
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
    prometheusUrl = mkOption {
      type = types.str;
      description = ''base url of prometheus'';
    };
    ixpManagerUrl = mkOption {
                                      type = types.str;
                                      description = ''base url of ixp manager'';
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
            "FOUNDATION_LISTMONK_URL" = "${cfg.listmonk.host}:${toString cfg.listmonk.port}";
            "FOUNDATION_LISTMONK_USER" = "${cfg.listmonk.user}";
            "FOUNDATION_LISTMONK_PASSWORD_FILE" = "${cfg.listmonk.passwordFile}";
            "FOUNDATION_LISTMONK_LISTS" = "${builtins.toJSON cfg.listmonk.allowed_lists}";
            "FOUNDATION_PROMETHEUS_URL" = "${cfg.prometheusUrl}";
            "FOUNDATION_IXP_MANAGER_URL" = "${cfg.ixpManagerUrl}";
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
    users.groups."foundation" = { };
  };
}
