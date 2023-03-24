import { SatelliteGroup } from "./groups";

export type SatelliteCategories = {
  specialInterest: Array<SatelliteGroup>;
  weatherAndEarthResources: Array<SatelliteGroup>;
  communications: Array<SatelliteGroup>;
  navigation: Array<SatelliteGroup>;
  scientific: Array<SatelliteGroup>;
  miscellaneous: Array<SatelliteGroup>;
};
