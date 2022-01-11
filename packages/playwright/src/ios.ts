import { getDevices } from '@canter/connector'
import { IOSDevice } from './device'

class IOS {
  async devices() {
    const amDevices = getDevices()
    return amDevices.map(device => new IOSDevice(device))
  }
}

export const ios = new IOS()
