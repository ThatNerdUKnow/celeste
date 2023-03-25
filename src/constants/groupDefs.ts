import { SatelliteCategories } from "../types/categories";
//import { parse_sat } from "satellite-rs";

export const SAT_CATEGORIES: SatelliteCategories = {
  specialInterest: [
    {
      name: "Last 30 Days' Launches",
      group: "last-30-days",
    },
    { name: "Space Stations", group: "stations" },
    { name: "100 (or so) Brightest", group: "visual" },
    { name: "Active Satellites", group: "active" },
    { name: "Analyst Satellites", group: "analyst" },
    { name: "Russian ASAT Test Debris (COSMOS 1408)", group: "1982-092" },
    { name: "Chinese ASAT Test Debris (FENGYUN 1C", group: "1999-025" },
    { name: "IRIDIUM 33 Debris", group: "iridium-33-debris" },
    { name: "COSMOS 2251 Debris", group: "cosmos-2251-debris" },
  ],
  weatherAndEarthResources: [
    { name: "Weather", group: "weather" },
    { name: "NOAA", group: "noaa" },
    { name: "GOES", group: "goes" },
    { name: "Earth Resources", group: "resource" },
    { name: "Search & Rescue (SARSAT)", group: "sarsat" },
    { name: "Disaster Monitoring", group: "dmc" },
    {
      name: "Tracking and Data Relay Satellite System (TDRSS)",
      group: "tdrss",
    },
    { name: "ARGOS Data Collection System", group: "argos" },
    { name: "Planet", group: "planet" },
    { name: "Spire", group: "spire" },
  ],
  communications: [
    { name: "Active Geosynchronous", group: "geo" },
    //{ name: "GEO Protected Zone", group: "gpz" },
    //{ name: "GEO Protected Zone Plus", group: "gpz-plus" },
    { name: "Intelsat", group: "intelsat" },
    { name: "SES", group: "ses" },
    { name: "Iridium", group: "iridium" },
    { name: "Iridium NEXT", group: "iridium-NEXT" },
    { name: "Starlink", group: "starlink" },
    { name: "OneWeb", group: "oneweb" },
    { name: "Orbcomm", group: "orbcomm" },
    { name: "Globalstar", group: "globalstar" },
    { name: "Swarm", group: "swarm" },
    { name: "Amateur Radio", group: "amateur" },
    { name: "Experimental Comm", group: "x-comm" },
    { name: "Other Comm", group: "other-comm" },
    { name: "SatNOGS", group: "satnogs" },
    { name: "Gorizont", group: "gorizont" },
    { name: "Raduga", group: "raduga" },
    { name: "Molniya", group: "molniya" },
  ],
  navigation: [
    { name: "GNSS", group: "gnss" },
    { name: "GPS Operational", group: "gps-ops" },
    { name: "GLONASS Operational", group: "glo-ops" },
    { name: "Galileo", group: "galileo" },
    { name: "Beidou", group: "beidou" },
    {
      name: "Satellite-Based Augmentation System (WAAS/EGNOS/MSAS}",
      group: "sbas",
    },
    { name: "Navy Navigation Satellite System (NNSS)", group: "nnss" },
    { name: "Russion LEO Navigation", group: "musson" },
  ],
  scientific: [
    { name: "Space & Earth Science", group: "science" },
    { name: "Geodetic", group: "geodetic" },
    { name: "Engineering", group: "engineering" },
    { name: "Education", group: "education" },
  ],
  miscellaneous: [
    { name: "Miscellaneous Military", group: "military" },
    { name: "Radar Calibration", group: "radar" },
    { name: "CubeSats", group: "cubesat" },
    { name: "Other Satellites", group: "other" },
  ],
};

/// SPECIAL INTEREST
//last-30-days
//stations
//visual
//active
//analyst
//1982-092
//1999-025
//iridium-33-debris
//cosmos-2251-debris

/**/

/// Weather & Earth Resources
//weather
//noaa
//goes
//resource
//sarsat
//dmc
//tdrss
//argos
//planet
//spire

/***/

/// Communications
//geo
//gpz
//gpz-plus
//intelsat
//ses
//iridium
//iridium-NEXT
//starlink
//oneweb
//orbcomm
//globalstar
//swarm
//amateur
//x-comm
//other-comm
//satnogs
//gorizont
//raduga
//molniya

/** */

/// Navigation
//gnss
//gps-ops
//glo-ops
//galileo
//beidou
//sbas
//nnss
//musson

/** */

/// Scientific Satellites
//science
//geodetic
//engineering
//education

/// Miscellaneous
//military
//radar
//cubesat
//other
