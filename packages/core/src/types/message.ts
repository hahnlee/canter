export interface RpcResponse<P, T extends string = string> {
  __selector: T
  __argument: P
}

export type WIRAutomationAvailability =
  | 'WIRAutomationAvailabilityNotAvailable'
  | 'WIRAutomationAvailabilityUnknown'
  | 'WIRAutomationAvailabilityAvailable'

export interface ReportIdentifierResponse {
  WIRAutomationAvailabilityKey: WIRAutomationAvailability
}

export interface WIRApplicationDictionaryValue {
  WIRAutomationAvailabilityKey: WIRAutomationAvailability
  WIRIsApplicationActiveKey: number
  WIRIsApplicationReadyKey: boolean
  WIRApplicationIdentifierKey: string
  WIRIsApplicationProxyKey: boolean
  WIRApplicationNameKey: string
  WIRApplicationBundleIdentifierKey: string
  WIRHostApplicationIdentifierKey: string
}

export interface ConnectedApplicationsResponse {
  WIRApplicationDictionaryKey: { [key: string]: WIRApplicationDictionaryValue }
}
