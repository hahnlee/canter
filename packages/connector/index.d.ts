export declare class AMService {
  send(message: unknown): void
  registerReceiveListener(callback: (response: any) => void): void
}

export declare class AMDevice {
  udid: string
  connect(): void
  disconnect(): void
  startService(name: string): AMService
}

export function getDevices(): AMDevice[]
