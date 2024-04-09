import { SatelliteError } from "@/types/generated/error";
import { Request } from "@/types/generated/request";
import { JsonValue } from "@/types/generated/serde_json/JsonValue";
import { invoke } from "@tauri-apps/api";
import { InvokeArgs } from "@tauri-apps/api/tauri";

export const isSatelliteErr = (err: unknown): err is SatelliteError => {
  return (
    typeof err === "object" &&
    err !== null &&
    "error" in err &&
    "message" in err
  );
};

const createCmd =
  <TArgs extends InvokeArgs, TResp>(cmd: string, key: string) =>
  async (args: TArgs): Promise<TResp> => {
    return await invoke<TResp>(cmd, { [key]: args });
  };

const request = createCmd<Request, JsonValue>("execute_request", "request");

export const Commands = {
  request,
};
