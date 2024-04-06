import { invoke } from "@tauri-apps/api";
import React, { useState } from "react";
import ResponseDisplay from "./ResponseDisplay";
import UrlInput from "./UrlInput";

const RequestView = () => {
  const [url, setUrl] = useState<string>("");
  const [resp, setResp] = useState<string>("");

  const handleUrlChanged = (event: React.ChangeEvent<HTMLInputElement>) => {
    const value = event.target.value;
    setUrl(value);
  };

  const handleExecuteClick = (_event: React.MouseEvent<HTMLButtonElement>) => {
    invoke("execute_request", { url: url }).then((response: any) =>
      setResp(JSON.stringify(JSON.parse(response), undefined, 2))
    );
  };

  return (
    <>
      <UrlInput
        url={url}
        onChange={handleUrlChanged}
        onClick={handleExecuteClick}
      />
      <ResponseDisplay response={resp} />
    </>
  );
};

export default RequestView;
