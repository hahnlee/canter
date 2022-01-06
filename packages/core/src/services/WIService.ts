import { AMService } from '@canter/connector'
import { v4 } from 'uuid'
import {
  ConnectedApplicationsResponse,
  ForwardGetListingResponse,
  RpcResponse,
  ReportIdentifierResponse,
} from '../types/message'
import { bufferToObject } from '../utils/bytes'

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
    const response = this.service.receive<RpcResponse<P, T>>()
    return response.__argument
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

    // TODO: (@hahnlee) do not block main thread
    let response = this.service.receive<RpcResponse<any>>()
    while (response.__selector !== '_rpc_applicationSentListing:') {
      response = this.service.receive<RpcResponse<any>>()
    }

    return response.__argument as ForwardGetListingResponse
  }

  forwardIndicateWebView(appId: string, pageId: number, enabled: boolean) {
    this.sendMessage('_rpc_forwardIndicateWebView:', {
      WIRConnectionIdentifierKey: this.connectionId,
      WIRApplicationIdentifierKey: appId,
      WIRPageIdentifierKey: pageId,
      WIRIndicateEnabledKey: enabled,
    })
  }

  forwardSocketSetup(appId: string, pageId: number, senderId: string) {
    this.sendMessage('_rpc_forwardSocketSetup:', {
      WIRConnectionIdentifierKey: this.connectionId,
      WIRApplicationIdentifierKey: appId,
      WIRPageIdentifierKey: pageId,
      WIRSenderKey: senderId,
    })

    const response = this.receiveMessage<{
      WIRDestinationKey: string
      WIRMessageDataKey: ArrayBuffer
    }>()

    const message = bufferToObject<{
      method: 'Target.targetCreated'
      params: { targetInfo: { targetId: string; type: 'page' } }
    }>(response.WIRMessageDataKey)
    return message
  }

  forwardSocketData(
    appId: string,
    pageId: number,
    senderId: string,
    targetId: string,
    id: number,
    message: any
  ) {
    this.sendMessage('_rpc_forwardSocketData:', {
      WIRConnectionIdentifierKey: this.connectionId,
      WIRApplicationIdentifierKey: appId,
      WIRPageIdentifierKey: pageId,
      WIRSenderKey: senderId,
      WIRSocketDataKey: new TextEncoder().encode(
        JSON.stringify({
          method: 'Target.sendMessageToTarget',
          id,
          params: {
            targetId,
            message: JSON.stringify(message),
          },
        })
      ),
    })

    this.receiveMessage()
  }
}
