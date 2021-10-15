import { AMService } from '@canter/connector'
import { v4 } from 'uuid'
import {
  ConnectedApplicationsResponse,
  RpcResponse,
  ReportIdentifierResponse,
} from './types/message'

export class WIService {
  private service: AMService

  private connectionId: string

  constructor(service: AMService) {
    this.service = service
    this.connectionId = v4()

    this.sendMessage.bind(this)
    this.receiveMessage.bind(this)
    this.getConnectedApplications.bind(this)
    this.forwardGetListing.bind(this)
  }

  private sendMessage(name: string, params: unknown) {
    this.service.send({
      __selector: name,
      __argument: params,
    })
  }

  private receiveMessage<P, T extends string = string>() {
    return this.service.receive<RpcResponse<P, T>>().__argument
  }

  reportIdentifier() {
    this.sendMessage('_rpc_reportIdentifier:', {
      WIRConnectionIdentifierKey: this.connectionId,
    })

    return this.receiveMessage<ReportIdentifierResponse>()
  }

  getConnectedApplications() {
    this.sendMessage('_rpc_getConnectedApplications:', {
      WIRConnectionIdentifierKey: this.connectionId,
    })

    const response = this.receiveMessage<ConnectedApplicationsResponse>()
    return Object.values(response.WIRApplicationDictionaryKey).flat()
  }

  forwardGetListing(bundle: string) {
    this.sendMessage('_rpc_forwardGetListing:', {
      WIRConnectionIdentifierKey: this.connectionId,
      WIRApplicationIdentifierKey: bundle,
    })

    // TODO: (@hanlee) do not block main thread
    while (
      this.service.receive<RpcResponse<any>>().__argument ===
      '_rpc_applicationConnected:'
    ) {}
  }
}
