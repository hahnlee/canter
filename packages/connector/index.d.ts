export declare class AMService {
  sendMessage(message: unknown): void;
  receiveMessage<T>(): T;
}

export declare class AMDevice {
  udid: string;
  connect(): void;
  disconnect(): void;
  startService(name: string): AMService;
}

export function getDevices(): AMDevice[]
