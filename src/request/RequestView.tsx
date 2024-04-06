import { invoke } from "@tauri-apps/api";
import React, { useState } from "react";
import ResponseDisplay from "./ResponseDisplay";
import UrlInput from "./UrlInput";

const RequestView = () => {
  const [url, setUrl] = useState<string>("");
  const [response, setResponse] = useState<string>("");

  const handleUrlChanged = (event: React.ChangeEvent<HTMLInputElement>) => {
    const value = event.target.value;
    setUrl(value);
  };

  const handleExecuteClick = (_event: React.MouseEvent<HTMLButtonElement>) => {
    invoke("execute_request", { url: url }).then((httpResponse: any) =>
      // TODO: (aldu 2024/04/05) This is a dirty hack and has to be fixed. Prefereably the serialized data is provided in the correct form.
      setResponse(JSON.stringify(JSON.parse(httpResponse), undefined, 2))
    );
  };

  return (
    <>
      <UrlInput
        url={url}
        onChange={handleUrlChanged}
        onClick={handleExecuteClick}
      />
      <ResponseDisplay response={response} />
    </>
  );
};

export default RequestView;
