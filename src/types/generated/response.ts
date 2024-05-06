// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.
import type { JsonValue } from "./serde_json/JsonValue";
import type { ResponseStatus } from "./response_status";

export type SatelliteResponse = {
  code: number;
  status: ResponseStatus;
  headers: { [key: string]: string };
  body: JsonValue;
};
