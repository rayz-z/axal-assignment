export interface TVLResponse {
  tvl: number;
  warning: boolean;
}

export interface BackendResponse {
  tvlArray: TVLResponse[],
}