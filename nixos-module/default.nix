{ pkgs, config, lib, ... }:
let
  cfg = config.dd-ix.website-content-api;
in
{
  options.dd-ix.website-content-api = with lib; {
    enable = mkOption {
      type = types.bool;
      default = false;
      description = ''Wether to enable website-content-api service'';
    };
    http = {
      host = mkOption {
        type = types.str;
        default = "http://127.0.0.1";
        description = ''
          To which IP website-content-api should bind.
        '';
      };
      port = mkOption {
        type = types.port;
        default = 8080;
        description = ''
          To which port should website-content-api bind.
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
      default = "website-content-api";
      description = ''systemd user'';
    };
    group = mkOption {
      type = types.str;
      default = "website-content-api";
      description = ''group of systemd user'';
    };
    logLevel = mkOption {
      type = types.str;
      default = "info";
      description = ''log level of the application'';
    };
    url = mkOption {
      type = types.str;
      description = ''under which domain website-content-api serves its content'';
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
        "website-content-api" = {
          enable = true;
          wantedBy = [ "multi-user.target" ];

          script = ''
            exec ${pkgs.website-content-api}/bin/website-content-api&
          '';

          environment = {
            "RUST_LOG" = "${cfg.logLevel}";
            "RUST_BACKTRACE" = if (cfg.logLevel == "info") then "0" else "1";
            "website-content-api_LISTEN_ADDR" = "${cfg.http.host}:${toString cfg.http.port}";
            "website-content-api_CONTENT_DIRECTORY" = "${pkgs.website-content}/content/";
            "website-content-api_BASE_URL" = "${cfg.url}";
            "website-content-api_LISTMONK_URL" = "${cfg.listmonk.host}:${toString cfg.listmonk.port}";
            "website-content-api_LISTMONK_USER" = "${cfg.listmonk.user}";
            "website-content-api_LISTMONK_PASSWORD_FILE" = "${cfg.listmonk.passwordFile}";
            "website-content-api_LISTMONK_LISTS" = "${builtins.toJSON cfg.listmonk.allowed_lists}";
            "website-content-api_PROMETHEUS_URL" = "${cfg.prometheusUrl}";
            "website-content-api_IXP_MANAGER_URL" = "${cfg.ixpManagerUrl}";
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
      description = "This guy runs website-content-api";
      isNormalUser = false;
      isSystemUser = true;
      group = cfg.group;
      uid = 1503;
    };
    users.groups."website-content-api" = { };
  };
}
