import "./App.css";
import { Button } from "@chakra-ui/react";
import { invoke } from "@tauri-apps/api/core";

function App() {
  return (
    <div className="App">
      <Button> Hello </Button>
    </div>
  );
}

export default App;
