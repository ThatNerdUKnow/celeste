import {Viewer} from 'resium';
import useSats from './hooks/useSats';
function App() {
  useSats();
  return (
    <Viewer/>
  )
}

export default App
