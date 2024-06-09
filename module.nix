{ pkgs, config, lib, ... }:
let
  cfg = config.dd-ix.website-content-api;
in
{
  options.dd-ix.website-content-api = {
    enable = lib.mkEnableOption "website-content-api";

    package = lib.mkPackageOption pkgs "website-content-api" { };
    content = lib.mkPackageOption pkgs "website-content" { };

    domain = lib.mkOption {
      type = lib.types.str;
      description = "The domain the frontend should be served.";
    };

    http = {
      host = lib.mkOption {
        type = lib.types.str;
        default = "http://127.0.0.1";
        description = ''
          To which IP website-content-api should bind.
        '';
      };
      port = lib.mkOption {
        type = lib.types.port;
        default = 8080;
        description = ''
          To which port should website-content-api bind.
        '';
      };
    };
    listmonk = {
      host = lib.mkOption {
        type = lib.types.str;
        default = "http://127.0.0.1";
        description = ''
          At which host is listmonk listening
        '';
      };
      port = lib.mkOption {
        type = lib.types.port;
        default = 8081;
        description = ''
          At which port is listmonk listening
        '';
      };
      user = lib.mkOption {
        type = lib.types.str;
        default = "admin";
        description = ''
          username of listmonk user
        '';
      };
      passwordFile = lib.mkOption {
        type = lib.types.path;
        description = ''
          path from where the user password can be read
        '';
      };
      allowed_lists = lib.mkOption {
        type = lib.types.listOf lib.types.int;
        default = [ ];
        description = ''
          list of allowed mailing list ids
        '';
      };
    };
    logLevel = lib.mkOption {
      type = lib.types.str;
      default = "info";
      description = ''log level of the application'';
    };
    url = lib.mkOption {
      type = lib.types.str;
      description = ''under which domain website-content-api serves its content'';
    };
    prometheusUrl = lib.mkOption {
      type = lib.types.str;
      description = ''base url of prometheus'';
    };
    ixpManagerUrl = lib.mkOption {
      type = lib.types.str;
      description = ''base url of ixp manager'';
    };
  };

  config = lib.mkIf cfg.enable {
    systemd.services.website-content-api = {
      enable = true;
      wantedBy = [ "multi-user.target" ];

      environment = {
        RUST_LOG = cfg.logLevel;
        RUST_BACKTRACE = if (cfg.logLevel == "info") then "0" else "1";
        WEBSITE_CONTENT_API_LISTEN_ADDR = "${cfg.http.host}:${toString cfg.http.port}";
        WEBSITE_CONTENT_API_CONTENT_DIRECTORY = "${pkgs.website-content}/content/";
        WEBSITE_CONTENT_API_BASE_URL = cfg.url;
        WEBSITE_CONTENT_API_LISTMONK_URL = "${cfg.listmonk.host}:${toString cfg.listmonk.port}";
        WEBSITE_CONTENT_API_LISTMONK_USER = cfg.listmonk.user;
        WEBSITE_CONTENT_API_LISTMONK_PASSWORD_FILE = "%d/listmonk_pw";
        WEBSITE_CONTENT_API_LISTMONK_LISTS = "${builtins.toJSON cfg.listmonk.allowed_lists}";
        WEBSITE_CONTENT_API_PROMETHEUS_URL = cfg.prometheusUrl;
        WEBSITE_CONTENT_API_IXP_MANAGER_URL = cfg.ixpManagerUrl;
      };

      serviceConfig = {
        ExecStart = "${pkgs.website-content-api}/bin/foundation";
        DynamicUser = true;
        Restart = "always";
        LoadCredential = "listmonk_pw:${cfg.listmonk.passwordFile}";
      };
    };

    services.nginx = {
      enable = true;

      virtualHosts."${cfg.domain}".locations = {
        "/text-blocks/assets/" = {
          alias = "${cfg.content}/content/text_blocks/assets/";
          tryFiles = "$uri $uri/ =404";
          extraConfig = ''
            expires max;
            access_log off;
          '';
        };
        "/blog/assets/" = {
          alias = "${cfg.content}/content/blog/assets/";
          tryFiles = "$uri $uri/ =404";
          extraConfig = ''
            expires max;
            access_log off;
          '';
        };
        "/team/assets/" = {
          alias = "${cfg.content}/content/team/assets/";
          tryFiles = "$uri $uri/ =404";
          extraConfig = ''
            expires max;
            access_log off;
          '';
        };
        "/" = {
          recommendedProxySettings = true;
          proxyPass = "http://${cfg.http.host}:${toString cfg.http.port}/";
        };
      };
    };
  };
}
