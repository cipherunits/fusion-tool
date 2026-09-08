use crate::setting::config::Language;
use anyhow::{Context, Result};
use console::style;
use std::fs;
use std::path::Path;

const PROJECT_NAME_PLACEHOLDER: &str = "__PROJECT_NAME__";

const PYTHON_MAIN: &str = r#"

"""Entry point: register routes, middleware, and start the server."""

from pathlib import Path

import src.modules.products.products  # registers @route classes

from fusion_framework.app import FusionApp
from fusion_framework.config import get_settings, load_settings_module
from fusion_framework.middleware import (
    cache_headers,
    cors,
    framework_headers,
    request_id,
    security_headers,
    static_files,
)


def main() -> None:
    load_settings_module("settings")
    from core.settings import TEMPLATES_DIR

    app = FusionApp(get_settings())
    app.use(static_files(root=Path(TEMPLATES_DIR) / "home", prefix="/"))  # Delete this if you are using API
    app.use(request_id())
    app.use(cors())
    app.use(cache_headers())
    app.use(security_headers())
    app.use(framework_headers())  # Delete this middleware if you are in production
    app.listen()


if __name__ == "__main__":
    main()

"#;

const PYTHON_PRODUCTS: &str = r#"
# Fusion Framework — application route module (FMA)
# Docs:     https://fusion.cipherunit.xyz/
# Desktop:  https://fusion.cipherunit.xyz/en/gui
# CLI tool: https://github.com/cipherunits/fusion-tool

from fusion_framework import status
from fusion_framework.api import FusionBaseApi
from fusion_framework.http_route import http_get
from fusion_framework.route import route
from fusion_framework.template import FusionBaseTemplate


@route("/")
class HomePage(FusionBaseTemplate):
    """Root page rendered (templates/home/index.html)."""

    template = "home/index.html"

    def context(self):
        return {
            "title": "__PROJECT_NAME__",
            "message": (
                "Your Fusion Framework application is ready. "
                "Start building by creating routes, templates, and business logic."
            ),
            "project": "__PROJECT_NAME__",
            "table_headers": ["Route", "Method"],
            "table_rows": [
                ["/v1/api/product", "GET"],
                ["/swagger", "GET"],
            ],
        }


@route(
    "api/[module]/",
    tags=["products"],
    desc="Product resource",
    version="v1",
    deprecated=False,
)
class ProductModule(FusionBaseApi):
    """Product management route module."""

    def get(self):
        # GET /v1/api/product/
        return self.response({"products_id": 12}, status=status.HTTP_SUCCESS)

    def post(self):
        return self.response({"products_id": 12}, status=status.HTTP_201_CREATED)

    def delete(self):
        return self.response({"products_id": 12}, status=status.HTTP_204_NO_CONTENT)

    def patch(self):
        return self.response({"products_id": 12}, status=status.HTTP_SUCCESS)

    @http_get("catalog/[action]", title="Product catalog", tags=["products"])
    def CatalogAction(self):
        # GET /v1/api/product/catalog/catalog
        return self.response({"items": []}, status=status.HTTP_SUCCESS)

"#;

const PYTHON_SETTINGS: &str = r#"
# Fusion Framework settings overlay (UPPERCASE names are merged into settings).
# Runtime values primarily come from fusion.<env>.json (FUSION_ENV, default: dev).
# Docs: https://fusion.cipherunit.xyz/

from fusion_framework import settings

# Never commit real secrets — prefer ALL_CAPS env placeholders in fusion.<env>.json
SECRET_KEY = settings.get("secret_key")

# Prefer reading debug from JSON; keep False as a safe default here
DEBUG = settings.get("debug", default=False)

# Auto-restart on source changes (True/False — language-native, not JSON)
RELOAD = settings.get("reload", default=True)

# templates root (relative to project cwd)
TEMPLATES_DIR = settings.get("templates.dir", default="templates")

"#;

const TYPESCRIPT_MAIN: &str = r#"
/**
 * Entry point: register routes, middleware, and start the server.
 */
import path from "node:path";

import "./src/modules/products/products";

import {
  FusionApp,
  cacheHeaders,
  cors,
  frameworkHeaders,
  getSettings,
  requestId,
  securityHeaders,
  staticFiles,
} from "fusion-framework";

async function main() {
  const { TEMPLATES_DIR } = await import("./core/settings");

  const app = new FusionApp(getSettings());
  app.use(
    staticFiles({ root: path.join(String(TEMPLATES_DIR), "home"), prefix: "/" }),
  ); // Delete this if you are using API
  app.use(requestId());
  app.use(cors());
  app.use(cacheHeaders());
  app.use(securityHeaders());
  app.use(frameworkHeaders()); // Delete this middleware if you are in production
  await app.listen();
}

main().catch((err) => {
  console.error(err);
  process.exit(1);
});
"#;

const TYPESCRIPT_PRODUCTS: &str = r#"
// Fusion Framework — application route module (FMA)
// Docs:     https://fusion.cipherunit.xyz/
// Desktop:  https://fusion.cipherunit.xyz/en/gui
// CLI tool: https://github.com/cipherunits/fusion-tool

import {
  FusionBaseApi,
  FusionBaseTemplate,
  httpGet,
  route,
  status,
} from "fusion-framework";

class HomePage extends FusionBaseTemplate {
  static template = "home/index.html";

  context() {
    return {
      title: "__PROJECT_NAME__",
      message:
        "Your Fusion Framework application is ready. Start building by creating routes, templates, and business logic.",
      project: "__PROJECT_NAME__",
      table_headers: ["Route", "Method"],
      table_rows: [
        ["/v1/api/product", "GET"],
        ["/swagger", "GET"],
      ],
    };
  }
}

route("/")(HomePage);

class ProductModule extends FusionBaseApi {
  get() {
    // GET /v1/api/product/
    return this.response({ products_id: 12 }, status.HTTP_SUCCESS);
  }

  post() {
    return this.response({ products_id: 12 }, status.HTTP_201_CREATED);
  }

  delete() {
    return this.response({ products_id: 12 }, status.HTTP_204_NO_CONTENT);
  }

  patch() {
    return this.response({ products_id: 12 }, status.HTTP_SUCCESS);
  }

  CatalogAction() {
    // GET /v1/api/product/catalog/catalog
    return this.response({ items: [] }, status.HTTP_SUCCESS);
  }
}

httpGet("catalog/[action]", {
  title: "Product catalog",
  tags: ["products"],
})(ProductModule.prototype.CatalogAction);

route("api/[module]/", {
  tags: ["products"],
  desc: "Product resource",
  version: "v1",
  deprecated: false,
})(ProductModule);

export { HomePage, ProductModule };
"#;

const TYPESCRIPT_SETTINGS: &str = r#"
// Fusion Framework settings overlay.
// Values come from fusion.<env>.json (FUSION_ENV, default: dev).

import { settings } from "fusion-framework";

settings.ensureLoaded([process.cwd()]);

export const SECRET_KEY = settings.get("secret_key");
export const DEBUG = settings.get("debug", false);
// Auto-restart on source changes (boolean — language-native)
export const RELOAD = settings.get("reload", true) ?? true;
// templates root
export const TEMPLATES_DIR = settings.get("templates.dir", "templates") ?? "templates";

settings.merge({
  reload: RELOAD,
  templates: { dir: TEMPLATES_DIR },
});
"#;

const CSHARP_MAIN: &str = r#"
// Fusion Framework entry point
using System.IO;
using FusionFramework;

static class Program
{
    static void Main()
    {
        Route.RegisterAll(typeof(Program).Assembly);

        SettingsStore.Current.EnsureLoaded(
            Directory.GetCurrentDirectory()
        );
        _ = CoreSettings.Reload;
        var templatesDir = CoreSettings.TemplatesDir?.ToString() ?? "templates";

        var app = new FusionApp(SettingsStore.GetSettings());
        app.Use(BuiltinMiddleware.StaticFiles(
            root: Path.Combine(templatesDir, "home"),
            prefix: "/")); // Delete this if you are using API
        app.Use(BuiltinMiddleware.RequestId());
        app.Use(BuiltinMiddleware.Cors());
        app.Use(BuiltinMiddleware.CacheHeaders());
        app.Use(BuiltinMiddleware.SecurityHeaders());
        app.Use(BuiltinMiddleware.FrameworkHeaders()); // Delete this middleware if you are in production
        app.Listen();
    }
}
"#;

const CSHARP_PRODUCTS: &str = r#"
// Fusion Framework — application route module (FMA)
// Docs:     https://fusion.cipherunit.xyz/
// Desktop:  https://fusion.cipherunit.xyz/en/gui
// CLI tool: https://github.com/cipherunits/fusion-tool

using System.Text.Json.Nodes;
using FusionFramework;

namespace Products;

[Route("/")]
public class HomePage : FusionBaseTemplate
{
    static HomePage()
    {
        Template = "home/index.html";
    }

    public override Dictionary<string, JsonNode?> Context() => new()
    {
        ["title"] = JsonValue.Create("__PROJECT_NAME__"),
        ["message"] = JsonValue.Create(
            "Your Fusion Framework application is ready. Start building by creating routes, templates, and business logic."),
        ["project"] = JsonValue.Create("__PROJECT_NAME__"),
        ["table_headers"] = new JsonArray("Route", "Method"),
        ["table_rows"] = new JsonArray(
            new JsonArray("/v1/api/product", "GET"),
            new JsonArray("/swagger", "GET")),
    };
}

[Route("api/[module]", Tags = new[] { "products" }, Desc = "Product resource", Version = "v1")]
public class ProductModule : FusionBaseApi
{
    // GET /v1/api/product
    public object Get() =>
        Response(new { products_id = 12 }, Status.HTTP_SUCCESS);

    public object Post() =>
        Response(new { products_id = 12 }, Status.HTTP_201_CREATED);

    public object Delete() =>
        Response(new { products_id = 12 }, Status.HTTP_204_NO_CONTENT);

    public object Patch() =>
        Response(new { products_id = 12 }, Status.HTTP_SUCCESS);

    // GET /v1/api/product/catalog/catalog
    [HttpGet("catalog/[action]", Title = "Product catalog")]
    public object CatalogAction() =>
        Response(new { items = Array.Empty<object>() }, Status.HTTP_SUCCESS);
}
"#;

const CSHARP_SETTINGS: &str = r#"
// Fusion Framework settings overlay.
// Values come from fusion.<env>.json (FUSION_ENV, default: dev).

using FusionFramework;

public static class CoreSettings
{
    static CoreSettings()
    {
        SettingsStore.Current.EnsureLoaded();
    }

    public static object? SecretKey => SettingsStore.Current.Get("secret_key");
    public static object? Debug => SettingsStore.Current.Get("debug", false);
    // Auto-restart on source changes
    public static object? Reload => SettingsStore.Current.Get("reload", true);
    // templates root
    public static object? TemplatesDir => SettingsStore.Current.Get("templates.dir", "templates");
}
"#;

const HOME_INDEX_HTML: &str = r#"<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="utf-8" />
  <meta name="viewport" content="width=device-width, initial-scale=1" />
  <title>{{ title }}</title>
  <link rel="icon" href="/Fusion-Framework-Transparent.png" type="image/png" />
  <link rel="apple-touch-icon" href="/Fusion-Framework-Transparent.png" />
  <script>
    (function () {
      try {
        var stored = localStorage.getItem("fusion-theme");
        var dark =
          stored === "dark" || stored === "light"
            ? stored === "dark"
            : window.matchMedia("(prefers-color-scheme: dark)").matches;
        document.documentElement.classList.toggle("dark", dark);
        document.documentElement.setAttribute("data-theme", dark ? "dark" : "light");
      } catch (_) {}
    })();
  </script>
  <link rel="preconnect" href="https://fonts.googleapis.com" />
  <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin />
  <link href="https://fonts.googleapis.com/css2?family=DM+Sans:wght@400;500;600;700&family=JetBrains+Mono:wght@400;500&display=swap" rel="stylesheet" />
  <style>
    {% include "fusion/components.css" %}
    {% include "home/style.css" %}
  </style>
</head>
<body>
  <div class="atmosphere" aria-hidden="true"></div>

  <button
    type="button"
    class="theme-toggle"
    id="theme-toggle"
    aria-label="Toggle color theme"
    title="Toggle theme"
  >
    <span class="theme-toggle__viewport" aria-hidden="true">
      <svg class="theme-toggle__icon theme-toggle__icon--sun is-active" viewBox="0 0 24 24" fill="none">
        <circle cx="12" cy="12" r="4" stroke="currentColor" stroke-width="1.8"/>
        <path d="M12 2v2M12 20v2M4.93 4.93l1.41 1.41M17.66 17.66l1.41 1.41M2 12h2M20 12h2M4.93 19.07l1.41-1.41M17.66 6.34l1.41-1.41" stroke="currentColor" stroke-width="1.8" stroke-linecap="round"/>
      </svg>
      <svg class="theme-toggle__icon theme-toggle__icon--moon" viewBox="0 0 24 24" fill="none">
        <path d="M21 14.3A8.2 8.2 0 0 1 9.7 3 7.4 7.4 0 1 0 21 14.3z" stroke="currentColor" stroke-width="1.8" stroke-linejoin="round"/>
      </svg>
    </span>
  </button>

  <main class="container">
    <div class="logo">
      <img src="/Fusion-Framework-Transparent.png" alt="Fusion Framework" width="88" />
    </div>

    {{<fusion.badge label="Installation successful" variant="success" dot={true} />}}

    <h1 class="brand">Welcome to <span>Fusion</span></h1>

    <p class="description">
      {{ message }}
    </p>

    {% <fusion.card title="Get started"> %}
      <div class="code" data-command="fusion command run">
        <code class="code__text"><span class="command">$</span> fusion command run</code>
        <button type="button" class="copy-btn" id="copy-command" aria-label="Copy command" title="Copy">
          <svg class="copy-btn__icon copy-btn__icon--copy" viewBox="0 0 24 24" fill="none" aria-hidden="true">
            <rect x="9" y="9" width="11" height="11" rx="2" stroke="currentColor" stroke-width="1.8"/>
            <path d="M5 15V7a2 2 0 0 1 2-2h8" stroke="currentColor" stroke-width="1.8" stroke-linecap="round"/>
          </svg>
          <svg class="copy-btn__icon copy-btn__icon--check" viewBox="0 0 24 24" fill="none" aria-hidden="true">
            <path d="M5 12.5l4.2 4.2L19 7" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round"/>
          </svg>
        </button>
      </div>
    {% </fusion.card> %}

    <div class="links">
      {{<fusion.button label="Documentation" href="https://fusion.cipherunit.xyz/docs" variant="primary" />}}
      {{<fusion.button label="GitHub" href="https://github.com/cipherunits" variant="secondary" />}}
    </div>

    {{<fusion.table headers={table_headers} rows={table_rows} caption="Sample API routes" />}}

    <div class="footer">
      Project <strong>{{ project }}</strong> · powered by
      <a class="footer-link" href="https://fusion.cipherunit.xyz/"><strong>Fusion Framework</strong></a>
    </div>
  </main>

  <script>
    (function () {
      var root = document.documentElement;
      var button = document.getElementById("theme-toggle");
      var media = window.matchMedia("(prefers-color-scheme: dark)");

      function readStored() {
        try {
          var stored = localStorage.getItem("fusion-theme");
          if (stored === "dark" || stored === "light") return stored;
        } catch (_) {}
        return null;
      }

      function systemDark() {
        return media.matches;
      }

      function resolveDark() {
        var stored = readStored();
        if (stored) return stored === "dark";
        return systemDark();
      }

      if (button) {
        var sun = button.querySelector(".theme-toggle__icon--sun");
        var moon = button.querySelector(".theme-toggle__icon--moon");
        var swapping = false;

        function applyTheme(dark, animate) {
          var currentDark = root.classList.contains("dark");
          if (animate && currentDark !== dark) {
            swapIcons(dark);
          }
          root.classList.toggle("dark", dark);
          root.setAttribute("data-theme", dark ? "dark" : "light");
          button.setAttribute("aria-pressed", dark ? "true" : "false");
          if (sun && moon && !animate) {
            sun.classList.toggle("is-active", !dark);
            moon.classList.toggle("is-active", dark);
            sun.classList.remove("is-leaving", "is-armed", "is-parked");
            moon.classList.remove("is-leaving", "is-armed", "is-parked");
          }
        }

        function persistTheme(dark) {
          try {
            localStorage.setItem("fusion-theme", dark ? "dark" : "light");
          } catch (_) {}
        }

        function swapIcons(toDark) {
          if (!sun || !moon || swapping) return;
          var outgoing = toDark ? sun : moon;
          var incoming = toDark ? moon : sun;
          swapping = true;

          outgoing.classList.remove("is-active");
          outgoing.classList.add("is-leaving");

          incoming.classList.add("is-armed");
          incoming.offsetHeight;
          incoming.classList.remove("is-armed");
          incoming.classList.add("is-active");

          window.setTimeout(function () {
            outgoing.classList.remove("is-leaving");
            outgoing.classList.add("is-parked");
            outgoing.offsetHeight;
            outgoing.classList.remove("is-parked");
            swapping = false;
          }, 720);
        }

        button.addEventListener("click", function () {
          var toDark = !root.classList.contains("dark");
          applyTheme(toDark, true);
          persistTheme(toDark);
        });

        applyTheme(resolveDark(), false);

        function onSystemChange() {
          if (readStored()) return;
          applyTheme(systemDark(), true);
        }

        if (typeof media.addEventListener === "function") {
          media.addEventListener("change", onSystemChange);
        } else if (typeof media.addListener === "function") {
          media.addListener(onSystemChange);
        }
      }

      var copyBtn = document.getElementById("copy-command");
      var code = document.querySelector(".code[data-command]");
      if (copyBtn && code) {
        var resetTimer;
        copyBtn.addEventListener("click", function () {
          var text = code.getAttribute("data-command") || "";
          function done() {
            copyBtn.classList.add("is-copied");
            copyBtn.setAttribute("aria-label", "Copied");
            copyBtn.setAttribute("title", "Copied");
            clearTimeout(resetTimer);
            resetTimer = setTimeout(function () {
              copyBtn.classList.remove("is-copied");
              copyBtn.setAttribute("aria-label", "Copy command");
              copyBtn.setAttribute("title", "Copy");
            }, 1600);
          }
          if (navigator.clipboard && navigator.clipboard.writeText) {
            navigator.clipboard.writeText(text).then(done).catch(function () {
              fallbackCopy(text, done);
            });
          } else {
            fallbackCopy(text, done);
          }
        });
      }

      function fallbackCopy(text, done) {
        var area = document.createElement("textarea");
        area.value = text;
        area.setAttribute("readonly", "");
        area.style.position = "fixed";
        area.style.left = "-9999px";
        document.body.appendChild(area);
        area.select();
        try {
          document.execCommand("copy");
        } catch (_) {}
        document.body.removeChild(area);
        done();
      }
    })();
  </script>
</body>
</html>
"#;

const HOME_STYLE_CSS: &str = r#"
:root {
  --radius: 0.625rem;
  --background: oklch(0.955 0 0);
  --foreground: oklch(0.18 0 0);
  --card: oklch(1 0 0);
  --card-foreground: oklch(0.18 0 0);
  --popover: oklch(1 0 0);
  --popover-foreground: oklch(0.18 0 0);
  --primary: oklch(0.2 0 0);
  --primary-foreground: oklch(0.99 0 0);
  --secondary: oklch(0.93 0 0);
  --secondary-foreground: oklch(0.22 0 0);
  --muted: oklch(0.93 0 0);
  --muted-foreground: oklch(0.4 0 0);
  --accent: oklch(0.925 0 0);
  --accent-foreground: oklch(0.18 0 0);
  --destructive: oklch(0.5 0.2 25);
  --destructive-foreground: oklch(0.99 0 0);
  --border: oklch(0.82 0 0);
  --input: oklch(0.82 0 0);
  --ring: oklch(0.45 0 0);
  --chart-1: oklch(0.4 0 0);
  --chart-2: oklch(0.55 0 0);
  --chart-3: oklch(0.7 0 0);
  --chart-4: oklch(0.3 0 0);
  --chart-5: oklch(0.6 0 0);
  --sidebar: oklch(1 0 0);
  --sidebar-foreground: oklch(0.18 0 0);
  --sidebar-primary: oklch(0.2 0 0);
  --sidebar-primary-foreground: oklch(0.99 0 0);
  --sidebar-accent: oklch(0.925 0 0);
  --sidebar-accent-foreground: oklch(0.18 0 0);
  --sidebar-border: oklch(0.82 0 0);
  --sidebar-ring: oklch(0.45 0 0);
}

html.dark {
  --background: oklch(0.145 0 0);
  --foreground: oklch(0.985 0 0);
  --card: oklch(0.205 0 0);
  --card-foreground: oklch(0.985 0 0);
  --popover: oklch(0.205 0 0);
  --popover-foreground: oklch(0.985 0 0);
  --primary: oklch(0.922 0 0);
  --primary-foreground: oklch(0.205 0 0);
  --secondary: oklch(0.269 0 0);
  --secondary-foreground: oklch(0.985 0 0);
  --muted: oklch(0.269 0 0);
  --muted-foreground: oklch(0.708 0 0);
  --accent: oklch(0.269 0 0);
  --accent-foreground: oklch(0.985 0 0);
  --destructive: oklch(0.704 0.191 22.216);
  --destructive-foreground: oklch(0.985 0 0);
  --border: oklch(1 0 0 / 10%);
  --input: oklch(1 0 0 / 15%);
  --ring: oklch(0.556 0 0);
  --chart-1: oklch(0.488 0.243 264.376);
  --chart-2: oklch(0.696 0.17 162.48);
  --chart-3: oklch(0.769 0.188 70.08);
  --chart-4: oklch(0.627 0.265 303.9);
  --chart-5: oklch(0.645 0.246 16.439);
  --sidebar: oklch(0.205 0 0);
  --sidebar-foreground: oklch(0.985 0 0);
  --sidebar-primary: oklch(0.488 0.243 264.376);
  --sidebar-primary-foreground: oklch(0.985 0 0);
  --sidebar-accent: oklch(0.269 0 0);
  --sidebar-accent-foreground: oklch(0.985 0 0);
  --sidebar-border: oklch(1 0 0 / 10%);
  --sidebar-ring: oklch(0.556 0 0);
}

* {
  box-sizing: border-box;
  margin: 0;
  padding: 0;
  scrollbar-width: none;
  -ms-overflow-style: none;
}

*::-webkit-scrollbar {
  display: none;
  width: 0;
  height: 0;
}

::selection {
  background: var(--foreground);
  color: var(--background);
}

html {
  color-scheme: light;
  overflow-y: auto;
}

html.dark {
  color-scheme: dark;
}

body {
  min-height: 100vh;
  display: flex;
  align-items: center;
  justify-content: center;
  font-family: "DM Sans", "Segoe UI", sans-serif;
  background-color: var(--background);
  color: var(--foreground);
  position: relative;
  overflow-x: hidden;
  overflow-y: auto;
}

.atmosphere {
  position: fixed;
  inset: 0;
  z-index: 0;
  pointer-events: none;
  background: transparent;
  overflow: hidden;
}

.atmosphere::before {
  content: "";
  position: absolute;
  inset: -120px;
  opacity: 1;
  background-image:
    radial-gradient(circle at 8% 12%, currentColor 1.35px, transparent 1.55px),
    radial-gradient(circle at 71% 6%, currentColor 0.95px, transparent 1.15px),
    radial-gradient(circle at 39% 28%, currentColor 1.7px, transparent 1.95px),
    radial-gradient(circle at 92% 22%, currentColor 1.1px, transparent 1.3px),
    radial-gradient(circle at 18% 47%, currentColor 1.5px, transparent 1.7px),
    radial-gradient(circle at 55% 41%, currentColor 0.85px, transparent 1.05px),
    radial-gradient(circle at 83% 58%, currentColor 1.45px, transparent 1.65px),
    radial-gradient(circle at 4% 73%, currentColor 1.2px, transparent 1.4px),
    radial-gradient(circle at 47% 69%, currentColor 1.6px, transparent 1.85px),
    radial-gradient(circle at 66% 88%, currentColor 1px, transparent 1.2px),
    radial-gradient(circle at 29% 91%, currentColor 1.4px, transparent 1.6px),
    radial-gradient(circle at 96% 79%, currentColor 0.9px, transparent 1.1px),
    radial-gradient(circle at 14% 33%, currentColor 0.8px, transparent 1px),
    radial-gradient(circle at 61% 15%, currentColor 1.25px, transparent 1.45px),
    radial-gradient(circle at 77% 36%, currentColor 1.55px, transparent 1.75px),
    radial-gradient(circle at 33% 54%, currentColor 1.05px, transparent 1.25px),
    radial-gradient(circle at 51% 3%, currentColor 0.75px, transparent 0.95px),
    radial-gradient(circle at 87% 94%, currentColor 1.3px, transparent 1.5px),
    radial-gradient(circle at 22% 64%, currentColor 1.65px, transparent 1.9px),
    radial-gradient(circle at 43% 81%, currentColor 0.95px, transparent 1.15px);
  background-size:
    187px 163px, 211px 179px, 173px 201px, 229px 157px, 193px 221px,
    167px 183px, 241px 169px, 179px 197px, 203px 151px, 217px 189px,
    161px 213px, 233px 171px, 181px 205px, 199px 159px, 223px 191px,
    171px 177px, 247px 165px, 185px 219px, 209px 153px, 195px 207px;
  background-position:
    0 0, 37px 21px, -19px 48px, 61px -11px, 14px 73px,
    -41px 29px, 88px 55px, 7px -33px, 52px 91px, -27px 64px,
    99px 12px, 23px -47px, -8px 105px, 71px 38px, -53px 17px,
    44px -22px, 16px 82px, -35px 59px, 78px -5px, 31px 96px;
  color: oklch(0.12 0 0 / 0.72);
  filter: blur(1.6px);
  -webkit-filter: blur(1.6px);
  animation: star-drift 36s linear infinite;
  will-change: background-position;
}

.atmosphere::after {
  content: "";
  position: absolute;
  inset: 0;
  background: var(--background);
  opacity: 0.42;
  pointer-events: none;
}

html.dark .atmosphere::before {
  color: oklch(1 0 0 / 0.75);
}

html.dark .atmosphere::after {
  opacity: 0.5;
}

@keyframes star-drift {
  0% {
    background-position:
      0 0, 37px 21px, -19px 48px, 61px -11px, 14px 73px,
      -41px 29px, 88px 55px, 7px -33px, 52px 91px, -27px 64px,
      99px 12px, 23px -47px, -8px 105px, 71px 38px, -53px 17px,
      44px -22px, 16px 82px, -35px 59px, 78px -5px, 31px 96px;
  }
  100% {
    background-position:
      187px -163px, -174px 200px, 154px -153px, -168px 146px, 179px -148px,
      126px -154px, -153px 114px, 172px 164px, -151px -60px, 190px 125px,
      -62px 225px, 256px 124px, 173px -100px, -128px 197px, 170px -174px,
      -127px 155px, 263px -83px, 150px 160px, -131px 148px, 226px -111px;
  }
}

.theme-toggle {
  position: fixed;
  top: 18px;
  right: 18px;
  z-index: 2;
  width: 42px;
  height: 42px;
  border-radius: calc(var(--radius) + 2px);
  border: 1px solid var(--border);
  background: var(--card);
  color: var(--foreground);
  display: inline-flex;
  align-items: center;
  justify-content: center;
  cursor: pointer;
  overflow: hidden;
  box-shadow: 0 8px 24px oklch(0 0 0 / 0.08);
  transition: background-color 0.15s ease, border-color 0.15s ease;
}

.theme-toggle:hover {
  background: var(--accent);
}

.theme-toggle:focus-visible {
  outline: 2px solid var(--ring);
  outline-offset: 2px;
}

.theme-toggle__viewport {
  position: relative;
  width: 18px;
  height: 18px;
  overflow: hidden;
  display: block;
}

.theme-toggle__icon {
  position: absolute;
  inset: 0;
  width: 18px;
  height: 18px;
  display: block;
  opacity: 0;
  transform: translateY(-108%);
  transition:
    transform 0.7s cubic-bezier(0.33, 1, 0.68, 1),
    opacity 0.55s cubic-bezier(0.33, 1, 0.68, 1);
  pointer-events: none;
  will-change: transform, opacity;
}

.theme-toggle__icon.is-active {
  opacity: 1;
  transform: translateY(0);
}

.theme-toggle__icon.is-leaving {
  opacity: 0;
  transform: translateY(108%);
}

.theme-toggle__icon.is-armed {
  transition: none;
  opacity: 0;
  transform: translateY(-108%);
}

.theme-toggle__icon.is-parked {
  transition: none;
  opacity: 0;
  transform: translateY(-108%);
}

.container {
  position: relative;
  z-index: 1;
  width: 100%;
  max-width: 760px;
  padding: 48px 24px;
  text-align: center;
}

.logo,
.fusion-badge,
.brand,
.description,
.fusion-card,
.links,
.fusion-table-wrap,
.footer {
  animation: rise 0.7s cubic-bezier(0.22, 1, 0.36, 1) both;
}

.logo { animation-delay: 0.02s; }
.fusion-badge { animation-delay: 0.08s; }
.brand { animation-delay: 0.14s; }
.description { animation-delay: 0.2s; }
.fusion-card { animation-delay: 0.26s; }
.links { animation-delay: 0.32s; }
.fusion-table-wrap { animation-delay: 0.38s; }
.footer { animation-delay: 0.44s; }

@keyframes rise {
  from {
    opacity: 0;
    transform: translateY(14px);
  }
  to {
    opacity: 1;
    transform: translateY(0);
  }
}

.logo {
  width: 92px;
  height: 92px;
  margin: 0 auto 22px;
  display: flex;
  align-items: center;
  justify-content: center;
  border-radius: calc(var(--radius) + 0.7rem);
  background: oklch(0.145 0 0);
  border: 1px solid oklch(1 0 0 / 8%);
  box-shadow: 0 18px 40px oklch(0 0 0 / 0.18);
  animation-name: rise, float;
  animation-duration: 0.7s, 4.5s;
  animation-timing-function: cubic-bezier(0.22, 1, 0.36, 1), ease-in-out;
  animation-fill-mode: both, none;
  animation-iteration-count: 1, infinite;
  animation-delay: 0.02s, 0.8s;
}

@keyframes float {
  0%, 100% { transform: translateY(0); }
  50% { transform: translateY(-6px); }
}

.logo img {
  display: block;
  max-width: 58px;
  height: auto;
}

.fusion-badge {
  margin-bottom: 18px;
  background: oklch(0.94 0.045 150) !important;
  color: oklch(0.36 0.08 150) !important;
  border-color: oklch(0.84 0.06 150) !important;
}

html.dark .fusion-badge {
  background: oklch(0.3 0.045 150) !important;
  color: oklch(0.88 0.06 150) !important;
  border-color: oklch(0.42 0.05 150) !important;
}

.brand {
  font-family: "DM Sans", "Segoe UI", sans-serif;
  font-size: clamp(36px, 7vw, 56px);
  font-weight: 700;
  line-height: 1.15;
  letter-spacing: -0.02em;
  color: var(--foreground);
  margin-bottom: 14px;
  font-style: normal;
  text-transform: none;
}

.brand span {
  font-weight: 700;
}

.description {
  max-width: 540px;
  margin: 0 auto;
  color: var(--muted-foreground);
  font-family: "DM Sans", "Segoe UI", sans-serif;
  font-size: 16px;
  font-weight: 400;
  line-height: 1.7;
  letter-spacing: 0;
  font-style: normal;
}

.fusion-card {
  margin-top: 36px;
  background: var(--card);
  color: var(--card-foreground);
  border-color: var(--border);
  border-radius: calc(var(--radius) + 4px);
}

.code {
  position: relative;
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 15px 48px 15px 17px;
  background: oklch(0.145 0 0);
  color: oklch(0.95 0 0);
  border-radius: var(--radius);
  font-family: "JetBrains Mono", Consolas, monospace;
  font-size: 13.5px;
  line-height: 1.65;
  overflow-x: auto;
  text-align: left;
  border: 1px solid oklch(1 0 0 / 8%);
}

.dark .code {
  background: oklch(0.12 0 0);
  border-color: var(--border);
}

.code__text {
  flex: 1;
  min-width: 0;
  font: inherit;
  color: inherit;
  white-space: nowrap;
}

.code .command {
  color: oklch(0.78 0 0);
}

.copy-btn {
  position: absolute;
  top: 50%;
  right: 10px;
  transform: translateY(-50%);
  width: 32px;
  height: 32px;
  border: 1px solid oklch(1 0 0 / 12%);
  border-radius: calc(var(--radius) - 2px);
  background: oklch(1 0 0 / 8%);
  color: oklch(0.92 0 0);
  display: inline-flex;
  align-items: center;
  justify-content: center;
  cursor: pointer;
  transition: background-color 0.15s ease, border-color 0.15s ease, color 0.15s ease;
}

.copy-btn:hover {
  background: oklch(1 0 0 / 14%);
  border-color: oklch(1 0 0 / 22%);
}

.copy-btn:focus-visible {
  outline: 2px solid var(--ring);
  outline-offset: 2px;
}

.copy-btn__icon {
  width: 15px;
  height: 15px;
  display: none;
}

.copy-btn__icon--copy { display: block; }
.copy-btn.is-copied .copy-btn__icon--copy { display: none; }
.copy-btn.is-copied .copy-btn__icon--check { display: block; }
.copy-btn.is-copied {
  color: oklch(0.86 0.08 150);
  border-color: oklch(0.55 0.08 150 / 45%);
}

.links {
  margin-top: 24px;
  display: flex;
  justify-content: center;
  gap: 12px;
  flex-wrap: wrap;
}

.fusion-btn {
  border-radius: var(--radius) !important;
}

.fusion-btn--primary {
  background: var(--primary) !important;
  border-color: var(--primary) !important;
  color: var(--primary-foreground) !important;
}

.fusion-btn--primary:hover {
  filter: brightness(1.08);
}

.fusion-btn--secondary {
  background: var(--secondary) !important;
  border-color: var(--border) !important;
  color: var(--secondary-foreground) !important;
}

.fusion-btn--secondary:hover {
  background: var(--accent) !important;
}

.fusion-table-wrap {
  margin-top: 28px;
  text-align: left;
  background: var(--card);
  border-color: var(--border);
  border-radius: calc(var(--radius) + 4px);
  color: var(--card-foreground);
  overflow: hidden;
}

.fusion-table th {
  background: var(--muted) !important;
  color: var(--muted-foreground) !important;
}

.fusion-table td,
.fusion-table caption {
  color: var(--card-foreground);
  border-color: var(--border) !important;
}

.fusion-table tbody tr {
  transition: background-color 0.15s ease;
  cursor: pointer;
}

.fusion-table tbody tr:hover {
  background: var(--accent);
}

.fusion-table tbody tr:hover td {
  color: var(--accent-foreground);
}

.footer {
  margin-top: 36px;
  color: var(--muted-foreground);
  font-size: 13px;
}

.footer-link {
  color: var(--foreground);
  text-decoration: underline;
  text-underline-offset: 2px;
}

.footer-link:hover {
  opacity: 0.8;
}

@media (prefers-reduced-motion: reduce) {
  .logo,
  .fusion-badge,
  .brand,
  .description,
  .fusion-card,
  .links,
  .fusion-table-wrap,
  .footer {
    animation: none;
  }

  .theme-toggle__icon {
    transition: none;
  }

  .atmosphere::before {
    animation: none;
  }
}

@media (max-width: 600px) {
  .container {
    padding: 32px 18px;
  }
  .logo {
    width: 80px;
    height: 80px;
  }
  .fusion-card {
    margin-top: 28px;
  }
  .description {
    font-size: 15px;
  }
  .links {
    flex-direction: column;
  }
  .fusion-btn {
    width: 100%;
  }
  .theme-toggle {
    top: 12px;
    right: 12px;
  }
}
"#;

/// Directories every new project starts with. `src/modules` also creates `src`.
const DIRECTORIES: [&str; 4] = [
    "core",
    "src/modules",
    "src/modules/products",
    "templates/home",
];

/// Default home page logo copied into `templates/home/` by `fusion init`.
const HOME_LOGO_PNG: &[u8] =
    include_bytes!("../../assets/Fusion-Framework-Transparent.png");

/// Create the starting layout of a new project:
///
/// ```text
/// ├── core
/// │   └── settings.py
/// ├── main.py
/// ├── templates
/// │   └── home
/// │       ├── index.html
/// │       ├── style.css
/// │       └── Fusion-Framework-Transparent.png
/// └── src
///     └── modules
///         └── products
///             └── products.py   # API + HomePage (FusionBaseTemplate)
/// ```
pub fn create(target_dir: &Path, language: &Language, project_name: &str) -> Result<()> {
    for directory in DIRECTORIES {
        let path = target_dir.join(directory);

        fs::create_dir_all(&path)
            .with_context(|| format!("Could not create {}", path.display()))?;

        report(&path);
    }

    let (main_template, settings_template, _) = templates(language);

    let extension = language.extension();

    write(
        &target_dir.join(format!("main{}", extension)),
        &render(main_template, project_name),
    )?;

    write(
        &target_dir
            .join("core")
            .join(format!("settings{}", extension)),
        &render(settings_template, project_name),
    )?;

    let products_template = products_template(language);

    write(
        &target_dir
            .join("src/modules/products")
            .join(format!("products{}", extension)),
        &render(products_template, project_name),
    )?;

    write(
        &target_dir.join("templates/home/index.html"),
        &render(HOME_INDEX_HTML, project_name),
    )?;
    write(
        &target_dir.join("templates/home/style.css"),
        HOME_STYLE_CSS,
    )?;

    let logo_path = target_dir.join("templates/home/Fusion-Framework-Transparent.png");
    fs::write(&logo_path, HOME_LOGO_PNG)
        .with_context(|| format!("Could not create {}", logo_path.display()))?;
    report(&logo_path);

    write_language_project_files(target_dir, language, project_name)?;

    Ok(())
}

fn write_language_project_files(
    target_dir: &Path,
    language: &Language,
    project_name: &str,
) -> Result<()> {
    match language {
        Language::Python => {
            let pyproject = format!(
                r#"[project]
name = "{name}"
version = "0.1.0"
description = "Fusion Framework application"
requires-python = ">=3.9"
"#,
                name = project_name.replace('_', "-"),
            );
            write(target_dir.join("pyproject.toml").as_path(), &pyproject)?;
            Ok(())
        }
        Language::TypeScript => {
            let package_json = format!(
                r#"{{
  "name": "{name}",
  "version": "0.1.0",
  "private": true,
  "type": "module",
  "scripts": {{
    "start": "node main.ts"
  }}
}}
"#,
                name = project_name,
            );
            write(&target_dir.join("package.json"), &package_json)?;
            write(
                &target_dir.join("tsconfig.json"),
                r#"{
  "compilerOptions": {
    "target": "ES2022",
    "module": "NodeNext",
    "moduleResolution": "NodeNext",
    "strict": true,
    "esModuleInterop": true,
    "skipLibCheck": true,
    "outDir": "dist",
    "types": ["node"]
  },
  "include": ["**/*.ts"]
}
"#,
            )?;
            Ok(())
        }
        Language::AspNetCore => {
            let csproj = format!(
                r#"<Project Sdk="Microsoft.NET.Sdk">
  <PropertyGroup>
    <OutputType>Exe</OutputType>
    <TargetFramework>net10.0</TargetFramework>
    <ImplicitUsings>enable</ImplicitUsings>
    <Nullable>enable</Nullable>
    <RootNamespace>{ns}</RootNamespace>
    <AutomaticallyUseReferenceAssemblyPackages>false</AutomaticallyUseReferenceAssemblyPackages>
  </PropertyGroup>
</Project>
"#,
                ns = project_name.replace('-', "_"),
            );
            write(
                &target_dir.join(format!("{project_name}.csproj")),
                &csproj,
            )?;
            Ok(())
        }
    }
}

/// Entry point, settings, and products templates for a language
fn templates(language: &Language) -> (&'static str, &'static str, &'static str) {
    match language {
        Language::Python => (PYTHON_MAIN, PYTHON_SETTINGS, PYTHON_PRODUCTS),

        Language::TypeScript => (TYPESCRIPT_MAIN, TYPESCRIPT_SETTINGS, TYPESCRIPT_PRODUCTS),

        Language::AspNetCore => (CSHARP_MAIN, CSHARP_SETTINGS, CSHARP_PRODUCTS),
    }
}

fn products_template(language: &Language) -> &'static str {
    match language {
        Language::Python => PYTHON_PRODUCTS,
        Language::TypeScript => TYPESCRIPT_PRODUCTS,
        Language::AspNetCore => CSHARP_PRODUCTS,
    }
}

fn render(template: &str, project_name: &str) -> String {
    template.replace(PROJECT_NAME_PLACEHOLDER, project_name)
}

fn write(path: &Path, content: &str) -> Result<()> {
    fs::write(path, content).with_context(|| format!("Could not create {}", path.display()))?;

    report(path);

    Ok(())
}

fn report(path: &Path) {
    println!(
        "{}",
        style(format!("✔ {} created successfully!", path.display()))
            .green()
            .bold()
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    fn temp_dir(label: &str) -> std::path::PathBuf {
        let dir = std::env::temp_dir().join(format!(
            "fusion-structure-test-{}-{}",
            label,
            std::process::id()
        ));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();
        dir
    }

    #[test]
    fn test_python_layout_is_created() {
        let target_dir = temp_dir("python");

        create(&target_dir, &Language::Python, "my-app").unwrap();

        assert!(target_dir.join("main.py").is_file());
        assert!(target_dir.join("core/settings.py").is_file());
        assert!(target_dir.join("src/modules/products/products.py").is_file());
        assert!(target_dir.join("templates/home/index.html").is_file());
        assert!(target_dir.join("templates/home/style.css").is_file());
        assert!(target_dir
            .join("templates/home/Fusion-Framework-Transparent.png")
            .is_file());
        assert!(target_dir.join("pyproject.toml").is_file());

        let main = fs::read_to_string(target_dir.join("main.py")).unwrap();
        assert!(main.contains("registers @route classes"));
        assert!(main.contains("static_files"));
        assert!(main.contains("TEMPLATES_DIR"));
        assert!(main.contains("Delete this if you are using API"));
        assert!(main.contains("framework_headers"));
        assert!(main.contains("security_headers"));
        assert!(main.contains("cache_headers"));
        assert!(main.contains("cors"));
        assert!(main.contains("request_id"));
        assert!(!main.contains("MIDDLEWARE"));
        assert!(!main.contains("@router"));

        let products = fs::read_to_string(target_dir.join("src/modules/products/products.py")).unwrap();
        assert!(products.contains("http_get"));
        assert!(products.contains("CatalogAction"));
        assert!(products.contains("version=\"v1\""));
        assert!(products.contains("FusionBaseTemplate"));
        assert!(products.contains("HomePage"));
        assert!(products.contains("table_headers"));
        assert!(products.contains("@route(\"/\")"));
        assert!(products.contains("home/index.html"));

        let settings = fs::read_to_string(target_dir.join("core/settings.py")).unwrap();
        assert!(settings.contains("RELOAD"));
        assert!(settings.contains("TEMPLATES_DIR"));

        let html = fs::read_to_string(target_dir.join("templates/home/index.html")).unwrap();
        assert!(html.contains("{{ title }}"));
        assert!(html.contains(r#"{% include "fusion/components.css" %}"#));
        assert!(html.contains(r#"{% include "home/style.css" %}"#));
        assert!(html.contains("/Fusion-Framework-Transparent.png"));
        assert!(html.contains(r#"rel="icon""#));
        assert!(html.contains("apple-touch-icon"));
        assert!(html.contains("fusion.badge"));
        assert!(html.contains("fusion.button"));
        assert!(html.contains("fusion.table"));
        assert!(html.contains("fusion.card"));
        assert!(html.contains("{{ project }}"));
        assert!(html.contains("https://fusion.cipherunit.xyz/docs"));
        assert!(html.contains(r#"href="https://github.com/cipherunits""#));
        assert!(!html.contains(r#"href="/swagger""#));
        assert!(!html.contains("github.com/cipherunits/fusion-framework"));
        assert!(html.contains(r#"class="brand""#));
        assert!(html.contains("Welcome to"));
        assert!(html.contains("https://fusion.cipherunit.xyz/"));
        assert!(html.contains("footer-link"));
        assert!(html.contains("atmosphere"));
        assert!(html.contains("theme-toggle"));
        assert!(html.contains("copy-command"));
        assert!(html.contains("data-command"));
        assert!(html.contains("fusion-theme"));
        let style = fs::read_to_string(target_dir.join("templates/home/style.css")).unwrap();
        assert!(style.contains("--background: oklch(0.955 0 0)"));
        assert!(style.contains("html.dark {"));
        assert!(style.contains("--background: oklch(0.145 0 0)"));
        assert!(style.contains("filter: blur(1.6px)"));
        assert!(style.contains("radial-gradient(circle at"));
        assert!(style.contains("star-drift"));
        assert!(style.contains("scrollbar-width: none"));
        assert!(style.contains("::-webkit-scrollbar"));
        assert!(style.contains("background-color: var(--background)"));
        assert!(style.contains(".fusion-table tbody tr:hover"));
        assert!(style.contains("oklch(0.94 0.045 150)"));
        assert!(style.contains(".copy-btn"));
        assert!(style.contains("::selection"));
        assert!(style.contains("cursor: pointer"));
        assert!(style.contains("0.7s cubic-bezier(0.33, 1, 0.68, 1)"));
        assert!(!style.contains("radial-gradient(900px"));
        assert!(style.contains("DM Sans"));
        assert!(products.contains("Your Fusion Framework application is ready."));
        assert!(products.contains(
            "Start building by creating routes, templates, and business logic."
        ));
        assert!(products.contains("my-app"));

        let pyproject = fs::read_to_string(target_dir.join("pyproject.toml")).unwrap();
        assert!(!pyproject.contains("dependencies"));
        assert!(!pyproject.contains("fusion-framework"));

        fs::remove_dir_all(&target_dir).unwrap();
    }

    #[test]
    fn test_typescript_layout_is_created() {
        let target_dir = temp_dir("typescript");

        create(&target_dir, &Language::TypeScript, "my-app").unwrap();

        assert!(target_dir.join("main.ts").is_file());
        assert!(target_dir.join("package.json").is_file());
        assert!(target_dir.join("tsconfig.json").is_file());
        assert!(target_dir.join("src/modules/products/products.ts").is_file());
        assert!(target_dir.join("templates/home/index.html").is_file());

        let package = fs::read_to_string(target_dir.join("package.json")).unwrap();
        assert!(!package.contains("dependencies"));
        assert!(!package.contains("devDependencies"));
        assert!(!package.contains("fusion-framework"));

        let products = fs::read_to_string(target_dir.join("src/modules/products/products.ts")).unwrap();
        assert!(products.contains("httpGet"));
        assert!(products.contains("CatalogAction"));
        assert!(products.contains("FusionBaseTemplate"));
        assert!(products.contains("HomePage"));
        assert!(products.contains("table_headers"));
        assert!(products.contains(r#"route("/")"#));

        let settings = fs::read_to_string(target_dir.join("core/settings.ts")).unwrap();
        assert!(settings.contains("RELOAD"));
        assert!(settings.contains("TEMPLATES_DIR"));

        assert!(target_dir
            .join("templates/home/Fusion-Framework-Transparent.png")
            .is_file());

        let main = fs::read_to_string(target_dir.join("main.ts")).unwrap();
        assert!(main.contains("staticFiles"));
        assert!(main.contains("TEMPLATES_DIR"));
        assert!(main.contains("Delete this if you are using API"));
        assert!(main.contains("requestId()"));
        assert!(main.find("staticFiles").unwrap() < main.find("requestId()").unwrap());
        assert!(main.find("requestId()").unwrap() < main.find("frameworkHeaders()").unwrap());
        assert!(main.contains("Delete this middleware if you are in production"));
        assert!(main.contains("frameworkHeaders"));
        assert!(main.contains("securityHeaders"));
        assert!(main.contains("cors"));
        assert!(main.contains("cacheHeaders"));
        assert!(!main.contains("MIDDLEWARE"));
        assert!(!main.contains("defaultBuiltinMiddleware"));
        assert!(!main.contains("ships with none by default"));

        fs::remove_dir_all(&target_dir).unwrap();
    }

    #[test]
    fn test_csharp_layout_is_created() {
        let target_dir = temp_dir("csharp");

        create(&target_dir, &Language::AspNetCore, "my-app").unwrap();

        assert!(target_dir.join("main.cs").is_file());
        assert!(target_dir.join("my-app.csproj").is_file());
        assert!(target_dir.join("src/modules/products/products.cs").is_file());
        assert!(target_dir.join("templates/home/index.html").is_file());

        let csproj = fs::read_to_string(target_dir.join("my-app.csproj")).unwrap();
        assert!(csproj.contains("net10.0"));
        assert!(!csproj.contains("PackageReference"));
        assert!(!csproj.contains("FusionFramework"));

        let products = fs::read_to_string(target_dir.join("src/modules/products/products.cs")).unwrap();
        assert!(products.contains("[HttpGet"));
        assert!(products.contains("CatalogAction"));
        assert!(products.contains("FusionBaseTemplate"));
        assert!(products.contains("HomePage"));
        assert!(products.contains("table_headers"));
        assert!(products.contains(r#"[Route("/")]"#));

        let settings = fs::read_to_string(target_dir.join("core/settings.cs")).unwrap();
        assert!(settings.contains("Reload"));
        assert!(settings.contains("TemplatesDir"));

        assert!(target_dir
            .join("templates/home/Fusion-Framework-Transparent.png")
            .is_file());

        let main = fs::read_to_string(target_dir.join("main.cs")).unwrap();
        assert!(main.contains("BuiltinMiddleware.StaticFiles"));
        assert!(main.contains("TemplatesDir"));
        assert!(main.contains("Delete this if you are using API"));
        assert!(main.find("StaticFiles").unwrap() < main.find("RequestId()").unwrap());
        assert!(main.find("RequestId()").unwrap() < main.find("FrameworkHeaders()").unwrap());
        assert!(main.contains("Delete this middleware if you are in production"));
        assert!(main.contains("FrameworkHeaders"));
        assert!(main.contains("BuiltinMiddleware.CacheHeaders"));
        assert!(main.contains("BuiltinMiddleware.RequestId"));
        assert!(main.contains("CoreSettings.Reload"));
        assert!(!main.contains("MIDDLEWARE"));

        fs::remove_dir_all(&target_dir).unwrap();
    }
}
