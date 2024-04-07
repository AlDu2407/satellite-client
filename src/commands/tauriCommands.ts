import { invoke } from "@tauri-apps/api";
import { SCRequest, SCRequestType } from "../types/request";
import { SCResponse } from "../types/response";

export const executeGet = async (
  url: string,
  secure: boolean
): Promise<SCResponse> => {
  let request: SCRequest = {
    type: SCRequestType.GET,
    url,
    secure,
  };

  return await invoke("execute_request", { request: request });
};
