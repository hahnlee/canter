import { AMService, getDevices } from '@canter/connector'
import { v1 } from 'uuid'

export { getDevices }

export class Device {
  private service: AMService

  private connectionId: string

  constructor(service: AMService) {
    this.service = service
    this.connectionId = v1()
  }

  sendMessage = (name: string, params: unknown) => {
    this.service.sendMessage({
      __selector: name,
      __argument: params,
    })
  }

  reportIdentifier = () => {
    this.sendMessage('_rpc_reportIdentifier:', {
      WIRConnectionIdentifierKey: this.connectionId,
    })
  }

  getConnectedApplications = () => {
    this.sendMessage('_rpc_getConnectedApplications:', {
      WIRConnectionIdentifierKey: this.connectionId,
    })
  }
}
