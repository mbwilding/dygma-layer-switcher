import "./App.css";
import * as focus from "./tauri/focus";
import { useEffect } from "react";

function App() {
  useEffect(() => {
    focus.connect_first_available();
  }, []);

  return (
    <main className="container">
      <h1>Welcome to Dygma Layer Switcher</h1>

      <div className="row">
        {Array.from({ length: 10 }, (_, i) => (
          <button key={i} onClick={() => focus.layer_activate(i)}>
            {i}
          </button>
        ))}
      </div>
    </main>
  );
}

export default App;
