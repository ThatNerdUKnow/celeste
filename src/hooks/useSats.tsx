import { Dispatch, SetStateAction, useEffect, useMemo, useState } from "react";
import { Axios } from "axios";
import { SAT_CATEGORIES } from "../constants/groupDefs";
import { SatelliteDataSource } from "satellite-rs";
import { chunk } from "lodash";
import { useCesium } from "resium";

export default function useSats() {
  let { viewer } = useCesium();

  let satellites = useMemo(() => {
    let sats = new SatelliteDataSource();
    console.log("Within Hook", sats);
    sats
      .load_data()
      .then(() => {
        console.log("Loaded Data");
        viewer?.dataSources.add(sats);
      })
      .catch(e=>{
        console.error("JS: Got error when loading data:",e)
      });

    return sats;
  }, []);

  return satellites;
}

/*
async function getAllSats(setter: Dispatch<SetStateAction<{}>>) {
  const FORMAT = "tle";

  const client = new Axios({
    baseURL: "https://celestrak.org/NORAD/elements/gp.php",
    params: {
      FORMAT,
    },
  });

 // let categories: Record<string, Elements[]> = {};

  // Split satellites by group
  let handle = Object.entries(SAT_CATEGORIES).map(
    async ([name, satelliteGroup]) => {
      let handle = satelliteGroup.map(async (group) => {
        try {
          // get raw tle data
          let response: { data: string } = await client.get("", {
            params: { GROUP: group.group },
          });

          // split data by new lines
          let lines = response.data.split("\n");

          console.time(group.name);

          // chunk the lines in groups of three and then join them again using newlines
          let elements = chunk(lines, 3)
            .map((lines) => lines.join("\n"))
            .filter((tle) => tle != "")
            .map((tle) => {
              try {
                //return parse_sat(tle);
              } catch (e) {
                console.error(`Couldn't parse the following TLE:
              ${tle}`);
              }
            });

          console.timeEnd(group.name);

          // If this category has already been accounted for, append to it. otherwise, set categories[name] to current elements
          if (categories[name]) {
            categories[name].concat(elements);
          } else {
            categories[name] = elements;
          }

          // group.name will always be unique
          categories[group.name] = elements;
        } catch (e) {
          console.error(`Couldn't fetch satellite group ${group.name}`, e);
        }
      });

      await Promise.all(handle);
    }
  );

  await Promise.all(handle);

  console.log(categories);

  setter(categories);
}
*/
