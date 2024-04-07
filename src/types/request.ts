export enum SCRequestType {
  GET = "Get",
  POST = "Post",
}

export interface SCRequest {
  type: SCRequestType;
  url: string;
  body?: any;
  secure?: boolean;
}
