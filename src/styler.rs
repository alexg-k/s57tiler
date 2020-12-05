use std::path::Path;
use crate::utils;
use serde_json::json;

/// https://tileserver.readthedocs.io/en/latest/config.html
pub fn create_config(
    out_dir: &Path,
    domain_list: Vec<String>
) {
    utils::check_out_dir(out_dir);
    let config_json = json!(
{
  "options": {
    "paths": {
      "root": "",
      "fonts": "fonts",
      "sprites": "sprites",
      "styles": "styles",
      "mbtiles": ""
    },
    "domains": domain_list,
    "formatQuality": {
      "jpeg": 80,
      "webp": 90
    },
    "maxScaleFactor": 3,
    "maxSize": 2048,
    "pbfAlias": "pbf",
    "serveAllFonts": true,
    "serveAllStyles": true,
    "serveStaticMaps": true,
    "tileMargin": 0
  },
  "data": {
    "marine-chart": {
      "mbtiles": "chart.mbtiles"
    }
  },
  "styles": {
    "basic": {
      "day_bright_style": "day_bright_style.json",
      "serve_rendered": true,
      "serve_data": true,
      // "tilejson": {
      //   "format": "vector"
      // }
    }
  }
}
    );
    utils::write_json(out_dir, "config.json", &config_json.to_string());
}

/// https://docs.mapbox.com/mapbox-gl-js/style-spec/
pub fn create_style(
    out_dir: &Path,
    base_url: &String,
) {
    utils::check_out_dir(out_dir);
    let style_json = json!(
{
  "version": 8,
  "name": "Day Bright",
  "sources": {
    "src_senc": {
      "type": "vector",
      "url": format!("{}/data/marine-chart.json", base_url)
    }
  },
  "sprite": "rastersymbols-day",
  "glyphs": format!("{}/fonts/{{fontstack}}/{{range}}.pbf", base_url),
  "layers": [
    {
      "id": "background",
      "type": "background",
      "paint": {
        "background-color": "#000",
        "background-opacity": 1
      }
    },
    {
      "id": "SEAARE_fill",
      "type": "fill",
      "source": "src_senc",
      "source-layer": "SEAARE",
      "filter": [
        "any",
        [
          "==",
          "$type",
          "Polygon"
        ]
      ],
      "paint": {
        "fill-color": "#CEEAEE"
      }
    },
    {
      "id": "SEAARE_line",
      "type": "line",
      "source": "src_senc",
      "source-layer": "SEAARE",
      "filter": [
        "any",
        [
          "==",
          "$type",
          "Polygon"
        ],
        [
          "==",
          "$type",
          "LineString"
        ]
      ],
      "paint": {
        "line-color": "#D631C9",
        "line-dasharray": [
          4,
          2
        ],
        "line-width": 1.5
      }
    },
    {
      "id": "DEPARE_fill_2",
      "type": "fill",
      "source": "src_senc",
      "source-layer": "DEPARE",
      "filter": ["all", ["==", "$type", "Polygon"], ["<=", "DRVAL1", 9.0]],
      "paint": {
        "fill-color": "#B4D6E3"
      }
    },
    {
      "id": "DEPARE_fill_1",
      "type": "fill",
      "source": "src_senc",
      "source-layer": "DEPARE",
      "filter": ["all", ["==", "$type", "Polygon"], ["<=", "DRVAL1", 3.0]],
      "paint": {
        "fill-color": "#5EB7F4"
      }
    },
    {
      "id": "DEPARE_fill_0",
      "type": "fill",
      "source": "src_senc",
      "source-layer": "DEPARE",
      "filter": ["all", ["==", "$type", "Polygon"], ["<", "DRVAL1", 0.0], ["<=", "DRVAL2", 0.0]],
      "paint": {
        "fill-color": "#75B493"
      }
    },
    {
      "id": "DEPARE_line",
      "type": "line",
      "source": "src_senc",
      "source-layer": "DEPARE",
      "filter": ["all", ["==", "$type", "Polygon"], [">", "DRVAL2", 0.0]],
      "paint": {
        "line-color": "#4F595B",
        "line-width": 0.5
      }
    },
    {
      "id": "SLCONS_line",
      "type": "line",
      "source": "src_senc",
      "source-layer": "SLCONS",
      "filter": [
        "all"
      ],
      "paint": {
        "line-color": "#4F595B",
        "line-width": 1
      }
    },
    {
      "id": "PONTON_fill",
      "type": "fill",
      "source": "src_senc",
      "source-layer": "PONTON",
      "filter": [
        "all",
        [
          "==",
          "$type",
          "Polygon"
        ]
      ],
      "paint": {
        "fill-color": "#B7911F"
      }
    },
    {
      "id": "PONTON_line",
      "type": "line",
      "source": "src_senc",
      "source-layer": "PONTON",
      "filter": [
        "any",
        [
          "==",
          "$type",
          "Polygon"
        ],
        [
          "==",
          "$type",
          "LineString"
        ]
      ],
      "paint": {
        "line-color": "#4F595B",
        "line-width": 1
      }
    },
    {
      "id": "HULKES_fill",
      "type": "fill",
      "source": "src_senc",
      "source-layer": "HULKES",
      "filter": [
        "any",
        [
          "==",
          "$type",
          "Polygon"
        ]
      ],
      "paint": {
        "fill-color": "#B7911F"
      }
    },
    {
      "id": "HULKES_line",
      "type": "line",
      "source": "src_senc",
      "source-layer": "HULKES",
      "filter": [
        "any",
        [
          "==",
          "$type",
          "Polygon"
        ],
        [
          "==",
          "$type",
          "LineString"
        ]
      ],
      "paint": {
        "line-color": "#4F595B",
        "line-width": 1.5
      }
    },
    {
      "id": "LNDARE_fill",
      "type": "fill",
      "source": "src_senc",
      "source-layer": "LNDARE",
      "filter": [
        "any",
        [
          "==",
          "$type",
          "Polygon"
        ]
      ],
      "paint": {
        "fill-color": "#C9B97A"
      }
    },
    {
      "id": "LNDARE_line",
      "type": "line",
      "source": "src_senc",
      "source-layer": "LNDARE",
      "filter": [
        "any",
        [
          "==",
          "$type",
          "Polygon"
        ],
        [
          "==",
          "$type",
          "LineString"
        ]
      ],
      "paint": {
        "line-color": "#4F595B",
        "line-width": 2
      }
    },
    {
      "id": "SOUNDG_txt",
      "type": "symbol",
      "source": "src_senc",
      "source-layer": "SOUNDG",
      "filter": [
        "any",
        [
          "==",
          "$type",
          "Point"
        ]
      ],
      "layout": {
        "text-font": [
          "Roboto Bold"
        ],
        "text-anchor": "bottom-right",
        "text-justify": "center",
        "text-field": ["get", "SOUNDG_FT"],
        "text-allow-overlap": true,
        "text-ignore-placement": true,
        "text-max-width": 9,
        "text-size": 10,
        "text-padding": 6,
        "symbol-placement": "point"
      },
      "paint": {
        "text-color": "#fff",
        "text-halo-color": "#000",
        "text-halo-width": 1.5
      }
    },
    {
      "id": "SOUNDGT_txt",
      "type": "symbol",
      "source": "src_senc",
      "source-layer": "SOUNDG",
      "filter": [
        "all",
        [
          "==",
          "$type",
          "Point"
        ],
        [ "!=", "SOUNDG_FTT", 0]
      ],
      "layout": {
        "text-font": [
          "Roboto Bold"
        ],
        "text-anchor": "top-left",
        "text-offset": [0.1, -0.7],
        "text-justify": "center",
        "text-field": ["get", "SOUNDG_FTT"],
        "text-allow-overlap": true,
        "text-ignore-placement": true,
        "text-max-width": 9,
        "text-size": 10,
        "text-padding": 6,
        "symbol-placement": "point"
      },
      "paint": {
        "text-color": "#000"
      }
    },
    {
      "id": "BOYSPP_point",
      "type": "symbol",
      "source": "src_senc",
      "source-layer": "BOYSPP",
      "filter": [
        "any",
        [
          "==",
          "$type",
          "Point"
        ]
      ],
      "layout": {
        "text-font": [
          "Roboto Bold"
        ],
        "text-anchor": "center",
        "text-justify": "center",
        "text-field": [
          "get",
          "OBJNAM"
        ],
        "text-allow-overlap": true,
        "text-ignore-placement": true,
        "text-max-width": 9,
        "text-size": 10,
        "text-padding": 6,
        "symbol-placement": "point"
      },
      "paint": {
        "text-color": "#fff",
        "text-halo-color": "#000",
        "text-halo-width": 1.5
      }
    }
  ]
}
    );
    utils::write_json(out_dir, "day_bright_style.json", &style_json.to_string());
}