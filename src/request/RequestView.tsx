import { invoke } from "@tauri-apps/api";
import React, { useState } from "react";
import ResponseDisplay from "./ResponseDisplay";
import UrlInput from "./UrlInput";

const RequestView = () => {
    const [url, setUrl] = useState<string>("");
    const [response, setResponse] = useState<any>();

    const handleUrlChanged = (event: React.ChangeEvent<HTMLInputElement>) => {
        const value = event.target.value;
        setUrl(value);
    };

    const handleExecuteClick = async (_event: React.MouseEvent<HTMLButtonElement>) => {
        const response = await invoke("execute_request", { url: url })
        setResponse(response);
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
