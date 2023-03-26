import { CustomDataSource, Viewer } from "resium";
import SatelliteData from "./components/satellites";
function App() {
  return (
    <Viewer>
      <SatelliteData />
    </Viewer>
  );
}

export default App;
