import { ShadowMode } from "cesium";
import { CustomDataSource, Moon, Sun, Viewer } from "resium";
import SatelliteData from "./components/satellites";
function App() {
  return (
    <Viewer
      terrainShadows={ShadowMode.ENABLED}
      showRenderLoopErrors={true}
      shadows={true}
    >
      <SatelliteData />
      <Moon />
      <Sun />
    </Viewer>
  );
}

export default App;
