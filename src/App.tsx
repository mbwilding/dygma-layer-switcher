import "./App.css";
import * as focus from "./tauri/focus";
import { useEffect } from "react";

function App() {
  useEffect(() => {
    focus.connect_first_available();
  }, []);

  function renderLayerButtons() {
    const buttons = [];
    for (let i = 0; i < 10; i++) {
      buttons.push(
        <button key={i} onClick={() => focus.layer_activate(i)}>
          {i + 1}
        </button>,
      );
    }
    return buttons;
  }

  return (
    <main className="container">
      <h1>Welcome to Dygma Layer Switcher</h1>

      <div className="row">{renderLayerButtons()}</div>
    </main>
  );
}

export default App;
