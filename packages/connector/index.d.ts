export declare class AMService {
  send(message: unknown): void;
  receive<T>(): T;
}

export declare class AMDevice {
  udid: string;
  connect(): void;
  disconnect(): void;
  startService(name: string): AMService;
}

export function getDevices(): AMDevice[]
