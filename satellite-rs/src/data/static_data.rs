use std::collections::{BTreeSet, HashSet};

use lazy_static::lazy_static;

use crate::data::group::Group;

use super::category::Category;

lazy_static! {
    pub static ref CATEGORIES: HashSet<Category> = HashSet::from([
        Category {
            name: "Special Interest",
            groups: BTreeSet::from([
                Group {
                    name: "Last 30 Days' Launches",
                    id: "last-30-days"
                },
                Group {
                    name: "Space Stations",
                    id: "stations"
                },
                Group {
                    name: "100 (or so) Brightest",
                    id: "visual"
                },
                Group {
                    name: "Active Satellites",
                    id: "active"
                },
                Group {
                    name: "Analyst Satellites",
                    id: "analyst"
                },
                Group {
                    name: "Russian ASAT Test Debris (COSMOS 1408)",
                    id: "1982-092"
                },
                Group {
                    name: "Chinese ASAT Test Debris (FENGYUN 1C)",
                    id: "1999-025"
                },
                Group {
                    name: "IRIDIUM 33 Debris",
                    id: "iridium-33-debris"
                },
                Group {
                    name: "COSMOS 2251 Debris",
                    id: "cosmos-2251-debris"
                }
            ]),
        },
        Category {
            name: "Weather & Earth Satellites",
            groups: BTreeSet::from([
                Group {
                    name: "Weather",
                    id: "weather"
                },
                Group {
                    name: "NOAA",
                    id: "noaa"
                },
                Group {
                    name: "GOES",
                    id: "goes"
                },
                Group {
                    name: "Earth Resources",
                    id: "resource"
                },
                Group {
                    name: "Search & Rescue (SARSAT)",
                    id: "sarsat"
                },
                Group {
                    name: "Disaster Monitoring",
                    id: "dmc"
                },
                Group {
                    name: "Tracking and Data Relay Satellite System (TDRSS)",
                    id: "tdrss"
                },
                Group {
                    name: "ARGOS Data Collection System",
                    id: "argos",
                },
                Group {
                    name: "Planet",
                    id: "spire"
                }
            ])
        },
        Category {
            name: "Communications",
            groups: BTreeSet::from([
                Group {
                    name: "Active Geosynchronous",
                    id: "geo"
                },
                Group {
                    name: "Intelsat",
                    id: "intelsat"
                },
                Group {
                    name: "SES",
                    id: "ses"
                },
                Group {
                    name: "Iridium",
                    id: "iridium"
                },
                Group {
                    name: "Iridium NEXT",
                    id: "iridium-NEXT"
                },
                Group {
                    name: "Starlink",
                    id: "starlink"
                },
                Group {
                    name: "OneWeb",
                    id: "oneweb"
                },
                Group {
                    name: "Orbcomm",
                    id: "orbcomm"
                },
                Group {
                    name: "Globalstar",
                    id: "globalstar"
                },
                Group {
                    name: "Swarm",
                    id: "swarm"
                },
                Group {
                    name: "Amateur Radio",
                    id: "amateur"
                },
                Group {
                    name: "Experimental Comm",
                    id: "x-comm"
                },
                Group {
                    name: "Other Comm",
                    id: "other-comm"
                },
                Group {
                    name: "SatNOGS",
                    id: "satnogs"
                },
                Group {
                    name: "Gorizont",
                    id: "gorizont"
                },
                Group {
                    name: "Raduga",
                    id: "raduga"
                },
                Group {
                    name: "Molniya",
                    id: "molniya"
                },
            ])
        },
        Category {
            name: "Navigation",
            groups: BTreeSet::from([
                Group {
                    name: "GNSS",
                    id: "gnss"
                },
                Group {
                    name: "GPS Operational",
                    id: "gps-ops"
                },
                Group {
                    name: "GLONASS Operational",
                    id: "glo-ops"
                },
                Group {
                    name: "Galileo",
                    id: "galileo"
                },
                Group {
                    name: "Beidou",
                    id: "beidou"
                },
                Group {
                    name: "Satellite-Based Augmentation System (WAAS/EGNOS/MSAS}",
                    id: "sbas",
                },
                Group {
                    name: "Navy Navigation Satellite System (NNSS)",
                    id: "nnss"
                },
                Group {
                    name: "Russion LEO Navigation",
                    id: "musson"
                },
            ])
        },
        Category {
            name: "Scientific",
            groups: BTreeSet::from([
                Group {
                    name: "Space & Earth Science",
                    id: "science"
                },
                Group {
                    name: "Geodetic",
                    id: "geodetic"
                },
                Group {
                    name: "Engineering",
                    id: "engineering"
                },
                Group {
                    name: "Education",
                    id: "education"
                },
            ])
        },
        Category {
            name: "Miscellaneous",
            groups: BTreeSet::from([
                Group {
                    name: "Miscellaneous Military",
                    id: "military"
                },
                Group {
                    name: "Radar Calibration",
                    id: "radar"
                },
                Group {
                    name: "CubeSats",
                    id: "cubesat"
                },
                Group {
                    name: "Other Satellites",
                    id: "other"
                },
            ])
        }
    ]);
}
