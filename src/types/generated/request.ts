// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.
import type { DeleteReq } from "./delete-req";
import type { GetReq } from "./get-req";
import type { PostReq } from "./post-req";
import type { PutReq } from "./put-req";

export type Request =
  | ({ method: "GET" } & GetReq)
  | ({ method: "POST" } & PostReq)
  | ({ method: "PUT" } & PutReq)
  | ({ method: "DELETE" } & DeleteReq);
