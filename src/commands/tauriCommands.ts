import { SatelliteError } from "@/types/generated/error";
import { Request } from "@/types/generated/request";
import { SatelliteResponse } from "@/types/generated/response";
import { invoke } from "@tauri-apps/api";
import { InvokeArgs } from "@tauri-apps/api/tauri";

export const isSatelliteErr = (err: unknown): err is SatelliteError => {
  return (
    typeof err === "object" && err !== null && "type" in err && "error" in err
  );
};

const createCmd =
  <TArgs extends InvokeArgs, TResp>(cmd: string, key: string) =>
  async (args: TArgs): Promise<TResp> => {
    return await invoke<TResp>(cmd, { [key]: args });
  };

const request = createCmd<Request, SatelliteResponse>(
  "execute_request",
  "request"
);

export const Commands = {
  request,
};
