import { useState } from "react";
import ModeSwitcher from "./common/ModeSwitcher";
import RequestView from "./request/RequestView";
import "./styles/global.css";

const App = () => {
  const [mode, setMode] = useState("light");

  const onModeChange = () => {
    setMode((previousMode) => (previousMode == "light" ? "dark" : "light"));
  };

  return (
    <div className={mode}>
      <div className="inline-flex w-screen bg-white text-lg text-gray-500 dark:bg-gray-900 dark:text-gray-400">
        <ModeSwitcher mode={mode} onChange={onModeChange} />
      </div>
      <RequestView />
    </div>
  );
};

export default App;
