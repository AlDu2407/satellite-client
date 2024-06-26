// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.
import type { ConnectReq } from "./connect-req";
import type { DeleteReq } from "./delete-req";
import type { GetReq } from "./get-req";
import type { HeadReq } from "./head-req";
import type { OptionsReq } from "./options-req";
import type { PatchReq } from "./patch-req";
import type { PostReq } from "./post-req";
import type { PutReq } from "./put-req";
import type { TraceReq } from "./trace-req";

export type Request =
  | ({ method: "GET" } & GetReq)
  | ({ method: "POST" } & PostReq)
  | ({ method: "PUT" } & PutReq)
  | ({ method: "DELETE" } & DeleteReq)
  | ({ method: "PATCH" } & PatchReq)
  | ({ method: "HEAD" } & HeadReq)
  | ({ method: "CONNECT" } & ConnectReq)
  | ({ method: "OPTIONS" } & OptionsReq)
  | ({ method: "TRACE" } & TraceReq);
