import { createCesiumComponent, useCesium } from "resium";
import useSats from "../hooks/useSats";

/*const SatelliteData = createCesiumComponent({
  name: "Satellite Data",
  create(context) {
    if (!context) {
      console.error("Resium context not available!");
    } else {
      //let dataSource = useSats();
    }
  },
});*/

export default function SatelliteData() {
  let data = useSats();
  console.log("From Component", data);
  return <></>;
}

//export default SatelliteData;
