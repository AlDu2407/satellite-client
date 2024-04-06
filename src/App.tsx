import { useEffect, useState } from "react";
import ModeSwitcher from "./common/ModeSwitcher";
import RequestView from "./request/RequestView";
import "./styles/global.css";

const App = () => {
  const defaultStyles = "h-full w-full";
  const [styles, setStyles] = useState<string>("");
  const [mode, setMode] = useState<string>("light");

  const onModeChange = () => {
    setMode((previousMode) => (previousMode == "light" ? "dark" : "light"));
  };

  useEffect(() => {
    setStyles(`${mode} ${defaultStyles}`);
  }, [mode]);

  return (
    <div className={styles}>
      <div className="inline-flex w-screen bg-white text-lg text-gray-500 dark:bg-gray-900 dark:text-gray-400">
        <ModeSwitcher mode={mode} onChange={onModeChange} />
      </div>
      <RequestView />
    </div>
  );
};

export default App;
