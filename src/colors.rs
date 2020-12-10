

use serde_json::{json, Value};

lazy_static! {
    pub static ref BG: &'static str = "#000";
    pub static ref TXT_BG: &'static str = "#000";
    pub static ref TXT_FG: &'static str = "#fff";
    pub static ref COLOR_KEYS: Vec<String> = ["day", "dusk", "dark"].iter().map(|&ea| ea.into()).collect();
    pub static ref COLORS: Value = json!({
      "DAY_BRIGHT": {
        "NODTA": "#a3b4b7",
        "CURSR": "#eb7d36",
        "CHBLK": "#070707",
        "CHGRD": "#7d898c",
        "CHGRF": "#a3b4b7",
        "CHRED": "#f15469",
        "CHGRN": "#68e456",
        "CHYLW": "#f4da48",
        "CHMGD": "#c545c3",
        "CHMGF": "#d3a6e9",
        "CHBRN": "#b19139",
        "CHWHT": "#d4eaee",
        "SCLBR": "#eb7d36",
        "CHCOR": "#eb7d36",
        "LITRD": "#f15469",
        "LITGN": "#68e456",
        "LITYW": "#f4da48",
        "ISDNG": "#c545c3",
        "DNGHL": "#f15469",
        "TRFCD": "#c545c3",
        "TRFCF": "#d3a6e9",
        "LANDA": "#c9b97a",
        "LANDF": "#8b661f",
        "CSTLN": "#525a5c",
        "SNDG1": "#7d898c",
        "SNDG2": "#070707",
        "DEPSC": "#525a5c",
        "DEPCN": "#7d898c",
        "DEPDW": "#d4eaee",
        "DEPMD": "#bad5e1",
        "DEPMS": "#98c5f2",
        "DEPVS": "#73b6ef",
        "DEPIT": "#83b295",
        "RADHI": "#68e456",
        "RADLO": "#3f8a34",
        "ARPAT": "#3fa56f",
        "NINFO": "#eb7d36",
        "RESBL": "#3a78f0",
        "ADINF": "#b29f34",
        "RESGR": "#7d898c",
        "SHIPS": "#070707",
        "PSTRK": "#070707",
        "SYTRK": "#7d898c",
        "PLRTE": "#dc4025",
        "APLRT": "#eb7d36",
        "UINFD": "#070707",
        "UINFF": "#7d898c",
        "UIBCK": "#d4eaee",
        "UIAFD": "#73b6ef",
        "UINFR": "#f15469",
        "UINFG": "#68e456",
        "UINFO": "#eb7d36",
        "UINFB": "#3a78f0",
        "UINFM": "#c545c3",
        "UIBDR": "#7d898c",
        "UIAFF": "#c9b97a",
        "OUTLW": "#070707",
        "OUTLL": "#c9b97a",
        "RES01": "#a3b4b7",
        "RES02": "#a3b4b7",
        "RES03": "#a3b4b7",
        "BKAJ1": "#070707",
        "BKAJ2": "#232728"
      },
      "DAY_BLACKBACK": {
        "NODTA": "#7d898c",
        "CURSR": "#dd7633",
        "CHBLK": "#a3b4b7",
        "CHGRD": "#a3b4b7",
        "CHGRF": "#7d898c",
        "CHRED": "#f15469",
        "CHGRN": "#68e456",
        "CHYLW": "#f4da48",
        "CHMGD": "#ddadf4",
        "CHMGF": "#ad3dab",
        "CHBRN": "#a38534",
        "CHWHT": "#d4eaee",
        "SCLBR": "#dd7633",
        "CHCOR": "#dd7633",
        "LITRD": "#f15469",
        "LITGN": "#68e456",
        "LITYW": "#f4da48",
        "ISDNG": "#ddadf4",
        "DNGHL": "#f15469",
        "TRFCD": "#ddadf4",
        "TRFCF": "#ad3dab",
        "LANDA": "#867b51",
        "LANDF": "#e5a833",
        "CSTLN": "#a3b4b7",
        "SNDG1": "#7d898c",
        "SNDG2": "#d4eaee",
        "DEPSC": "#a3b4b7",
        "DEPCN": "#7d898c",
        "DEPDW": "#070707",
        "DEPMD": "#262b2e",
        "DEPMS": "#3f5265",
        "DEPVS": "#436b8c",
        "DEPIT": "#41715f",
        "RADHI": "#68e456",
        "RADLO": "#3f8a34",
        "ARPAT": "#50d08c",
        "NINFO": "#dd7633",
        "RESBL": "#3a78f0",
        "ADINF": "#b29f34",
        "RESGR": "#7d898c",
        "SHIPS": "#d4eaee",
        "PSTRK": "#d4eaee",
        "SYTRK": "#7d898c",
        "PLRTE": "#dc4025",
        "APLRT": "#dd7633",
        "UINFD": "#d4eaee",
        "UINFF": "#7d898c",
        "UIBCK": "#070707",
        "UIAFD": "#436b8c",
        "UINFR": "#f15469",
        "UINFG": "#68e456",
        "UINFO": "#dd7633",
        "UINFB": "#3a78f0",
        "UINFM": "#ad3dab",
        "UIBDR": "#a3b4b7",
        "UIAFF": "#867b51",
        "OUTLW": "#070707",
        "OUTLL": "#867b51",
        "RES01": "#7d898c",
        "RES02": "#7d898c",
        "RES03": "#7d898c",
        "BKAJ1": "#070707",
        "BKAJ2": "#232728"
      },
      "DAY_WHITEBACK": {
        "NODTA": "#8b999b",
        "CURSR": "#c76b2e",
        "CHBLK": "#070707",
        "CHGRD": "#6a7577",
        "CHGRF": "#8b999b",
        "CHRED": "#cd4759",
        "CHGRN": "#59c249",
        "CHYLW": "#d0ba3d",
        "CHMGD": "#a83ba6",
        "CHMGF": "#b48dc6",
        "CHBRN": "#967b30",
        "CHWHT": "#b4c7ca",
        "SCLBR": "#c76b2e",
        "CHCOR": "#c76b2e",
        "LITRD": "#cd4759",
        "LITGN": "#59c249",
        "LITYW": "#d0ba3d",
        "ISDNG": "#a83ba6",
        "DNGHL": "#cd4759",
        "TRFCD": "#a83ba6",
        "TRFCF": "#b48dc6",
        "LANDA": "#ab9d68",
        "LANDF": "#76571a",
        "CSTLN": "#464d4e",
        "SNDG1": "#6a7577",
        "SNDG2": "#070707",
        "DEPSC": "#464d4e",
        "DEPCN": "#6a7577",
        "DEPDW": "#b4c7ca",
        "DEPMD": "#9eb5c0",
        "DEPMS": "#81a8ce",
        "DEPVS": "#629acb",
        "DEPIT": "#6f977f",
        "RADHI": "#59c249",
        "RADLO": "#36752c",
        "ARPAT": "#368c5e",
        "NINFO": "#c76b2e",
        "RESBL": "#3166cc",
        "ADINF": "#97872c",
        "RESGR": "#6a7577",
        "SHIPS": "#070707",
        "PSTRK": "#070707",
        "SYTRK": "#6a7577",
        "PLRTE": "#dc4025",
        "APLRT": "#c76b2e",
        "UINFD": "#070707",
        "UINFF": "#6a7577",
        "UIBCK": "#b4c7ca",
        "UIAFD": "#629acb",
        "UINFR": "#cd4759",
        "UINFG": "#59c249",
        "UINFO": "#c76b2e",
        "UINFB": "#3166cc",
        "UINFM": "#a83ba6",
        "UIBDR": "#6a7577",
        "UIAFF": "#ab9d68",
        "OUTLW": "#070707",
        "OUTLL": "#ab9d68",
        "RES01": "#8b999b",
        "RES02": "#8b999b",
        "RES03": "#8b999b",
        "BKAJ1": "#070707",
        "BKAJ2": "#1e2122"
      },
      "DUSK": {
        "NODTA": "#292e2e",
        "CURSR": "#4b2613",
        "CHBLK": "#363c3d",
        "CHGRD": "#363c3d",
        "CHGRF": "#292e2e",
        "CHRED": "#501c23",
        "CHGRN": "#234c1d",
        "CHYLW": "#514918",
        "CHMGD": "#4a3a51",
        "CHMGF": "#3a1439",
        "CHBRN": "#362c11",
        "CHWHT": "#474e4f",
        "SCLBR": "#4b2613",
        "CHCOR": "#4b2613",
        "LITRD": "#501c23",
        "LITGN": "#234c1d",
        "LITYW": "#514918",
        "ISDNG": "#4a3a51",
        "DNGHL": "#501c23",
        "TRFCD": "#4a3a51",
        "TRFCF": "#3a1439",
        "LANDA": "#2c291b",
        "LANDF": "#4c3811",
        "CSTLN": "#363c3d",
        "SNDG1": "#292e2e",
        "SNDG2": "#474e4f",
        "DEPSC": "#363c3d",
        "DEPCN": "#292e2e",
        "DEPDW": "#070707",
        "DEPMD": "#0c0e0f",
        "DEPMS": "#151b21",
        "DEPVS": "#16232f",
        "DEPIT": "#15251f",
        "RADHI": "#234c1d",
        "RADLO": "#152e11",
        "ARPAT": "#1a452f",
        "NINFO": "#4b2613",
        "RESBL": "#132850",
        "ADINF": "#3b3511",
        "RESGR": "#292e2e",
        "SHIPS": "#474e4f",
        "PSTRK": "#474e4f",
        "SYTRK": "#292e2e",
        "PLRTE": "#49150c",
        "APLRT": "#4b2613",
        "UINFD": "#474e4f",
        "UINFF": "#292e2e",
        "UIBCK": "#070707",
        "UIAFD": "#16232f",
        "UINFR": "#501c23",
        "UINFG": "#234c1d",
        "UINFO": "#4b2613",
        "UINFB": "#132850",
        "UINFM": "#3a1439",
        "UIBDR": "#363c3d",
        "UIAFF": "#2c291b",
        "OUTLW": "#070707",
        "OUTLL": "#2c291b",
        "RES01": "#292e2e",
        "RES02": "#292e2e",
        "RES03": "#292e2e",
        "BKAJ1": "#070707",
        "BKAJ2": "#0b0d0d"
      },
      "NIGHT": {
        "NODTA": "#070707",
        "CURSR": "#341c0c",
        "CHBLK": "#1f2223",
        "CHGRD": "#1f2223",
        "CHGRF": "#101212",
        "CHRED": "#3b110a",
        "CHGRN": "#162207",
        "CHYLW": "#29210a",
        "CHMGD": "#341234",
        "CHMGF": "#341234",
        "CHBRN": "#0f0d05",
        "CHWHT": "#252929",
        "SCLBR": "#341c0c",
        "CHCOR": "#341c0c",
        "LITRD": "#3b110a",
        "LITGN": "#162207",
        "LITYW": "#29210a",
        "ISDNG": "#341234",
        "DNGHL": "#3b110a",
        "TRFCD": "#3a143a",
        "TRFCF": "#341234",
        "LANDA": "#0d0a08",
        "LANDF": "#171105",
        "CSTLN": "#252929",
        "SNDG1": "#1f2223",
        "SNDG2": "#2b3030",
        "DEPSC": "#252929",
        "DEPCN": "#1f2223",
        "DEPDW": "#070707",
        "DEPMD": "#070707",
        "DEPMS": "#030413",
        "DEPVS": "#030413",
        "DEPIT": "#080b09",
        "RADHI": "#162207",
        "RADLO": "#0a1003",
        "ARPAT": "#0c1f15",
        "NINFO": "#341c0c",
        "RESBL": "#151d45",
        "ADINF": "#29210a",
        "RESGR": "#101212",
        "SHIPS": "#252929",
        "PSTRK": "#252929",
        "SYTRK": "#1f2223",
        "PLRTE": "#42130b",
        "APLRT": "#341c0c",
        "UINFD": "#2b3030",
        "UINFF": "#1f2223",
        "UIBCK": "#070707",
        "UIAFD": "#030413",
        "UINFR": "#3b110a",
        "UINFG": "#162207",
        "UINFO": "#341c0c",
        "UINFB": "#151d45",
        "UINFM": "#341234",
        "UIBDR": "#1f2223",
        "UIAFF": "#0d0a08",
        "OUTLW": "#070707",
        "OUTLL": "#0d0a08",
        "RES01": "#070707",
        "RES02": "#070707",
        "RES03": "#070707",
        "BKAJ1": "#070707",
        "BKAJ2": "#070808"
      }
    });
}
