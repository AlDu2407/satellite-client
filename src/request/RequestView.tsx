import { invoke } from "@tauri-apps/api";
import React, { useState } from "react";
import UrlInput from "./UrlInput";

const RequestView = () => {
  const [url, setUrl] = useState<string>("");

  const handleUrlChanged = (event: React.ChangeEvent<HTMLInputElement>) => {
    const value = event.target.value;
    setUrl(value);
  };

  const handleExecuteClick = (_event: React.MouseEvent<HTMLButtonElement>) => {
    invoke("execute_request", { url: url }).then((response) =>
      console.log(response)
    );
  };

  return (
    <>
      <UrlInput
        url={url}
        onChange={handleUrlChanged}
        onClick={handleExecuteClick}
      />
    </>
  );
};

export default RequestView;
