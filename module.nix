{ pkgs, config, lib, ... }:

let
  cfg = config.dd-ix.website-content-api;
in
{
  options.dd-ix.website-content-api = {
    enable = lib.mkEnableOption "DD-IX Website Content API";

    package = lib.mkPackageOption pkgs "ddix-website-content-api" { };
    content = lib.mkPackageOption pkgs "ddix-website-content" { };

    domain = lib.mkOption {
      type = lib.types.str;
      description = "The domain the frontend should be served.";
    };

    http = {
      host = lib.mkOption {
        type = lib.types.str;
        default = "127.0.0.1";
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
    url = lib.mkOption {
      type = lib.types.str;
      description = ''under which domain website-content-api serves its content'';
    };
    lookingGlassUrl = lib.mkOption {
      type = lib.types.str;
      description = "base url of looking glass";
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
    systemd.services.ddix-website-content-api = {
      description = "DD-IX Website Content API";

      wantedBy = [ "multi-user.target" ];

      environment = {
        WEBSITE_CONTENT_API_LISTEN_ADDR = "${cfg.http.host}:${toString cfg.http.port}";
        WEBSITE_CONTENT_API_CONTENT_DIRECTORY = "${cfg.content}/content/";
        WEBSITE_CONTENT_API_BASE_URL = cfg.url;
        WEBSITE_CONTENT_API_LOOKING_GLASS_URL = cfg.lookingGlassUrl;
        WEBSITE_CONTENT_API_PROMETHEUS_URL = cfg.prometheusUrl;
        WEBSITE_CONTENT_API_IXP_MANAGER_URL = cfg.ixpManagerUrl;
      };

      serviceConfig = {
        ExecStart = lib.getExe cfg.package;
        DynamicUser = true;
        Restart = "always";
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
        "/event/assets/" = {
          alias = "${cfg.content}/content/event/assets/";
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
        "/news/assets/" = {
          alias = "${cfg.content}/content/news/assets/";
          tryFiles = "$uri $uri/ =404";
          extraConfig = ''
            expires max;
            access_log off;
          '';
        };
        "/documents/download/" = {
          alias = "${cfg.content}/content/documents/download/";
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
