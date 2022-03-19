import { AMService } from '@canter/connector'
import { v4 } from 'uuid'
import {
  ConnectedApplicationsResponse,
  ForwardGetListingResponse,
  RpcResponse,
  RpcResponseCallback,
  RpcResponseMatcher,
  ReportIdentifierResponse,
} from '../types/message'
import { bufferToObject } from '../utils/bytes'

export class WIService {
  private service: AMService

  private connectionId: string

  private callbacks: Set<RpcResponseCallback<any>> = new Set()

  constructor(service: AMService) {
    this.service = service
    this.connectionId = v4()
    service.registerReceiveListener(this.receiveMessage)
  }

  private sendMessage = (name: string, params: unknown) => {
    this.service.send({
      __selector: name,
      __argument: params,
    })
  }

  private receiveMessage = (message: RpcResponse<any>) => {
    this.callbacks.forEach((callback) => {
      callback(message)
    })
  }

  private waitUntilResponse = <P>(matcher: RpcResponseMatcher) => {
    return new Promise<RpcResponse<P>>((resolve) => {
      const handler = (response: RpcResponse<any>) => {
        if (matcher(response)) {
          this.callbacks.delete(handler)
          resolve(response)
        }
      }

      this.callbacks.add(handler)
    })
  }

  reportIdentifier = async () => {
    this.sendMessage('_rpc_reportIdentifier:', {
      WIRConnectionIdentifierKey: this.connectionId,
    })

    const response = await this.waitUntilResponse<ReportIdentifierResponse>(
      (message) => message.__selector === '_rpc_reportCurrentState:'
    )

    return response
  }

  getConnectedApplications = async () => {
    this.sendMessage('_rpc_getConnectedApplications:', {
      WIRConnectionIdentifierKey: this.connectionId,
    })

    const response =
      await this.waitUntilResponse<ConnectedApplicationsResponse>(
        (message) =>
          message.__selector === '_rpc_reportConnectedApplicationList:'
      )

    return Object.values(response.__argument.WIRApplicationDictionaryKey).flat()
  }

  forwardGetListing = async (bundle: string) => {
    this.sendMessage('_rpc_forwardGetListing:', {
      WIRConnectionIdentifierKey: this.connectionId,
      WIRApplicationIdentifierKey: bundle,
    })

    const response = await this.waitUntilResponse<ForwardGetListingResponse>(
      (message) => message.__selector === '_rpc_applicationSentListing:'
    )

    return response.__argument
  }

  forwardIndicateWebView = (
    appId: string,
    pageId: number,
    enabled: boolean
  ) => {
    this.sendMessage('_rpc_forwardIndicateWebView:', {
      WIRConnectionIdentifierKey: this.connectionId,
      WIRApplicationIdentifierKey: appId,
      WIRPageIdentifierKey: pageId,
      WIRIndicateEnabledKey: enabled,
    })
  }

  forwardSocketSetup = async (
    appId: string,
    pageId: number,
    senderId: string
  ) => {
    this.sendMessage('_rpc_forwardSocketSetup:', {
      WIRConnectionIdentifierKey: this.connectionId,
      WIRApplicationIdentifierKey: appId,
      WIRPageIdentifierKey: pageId,
      WIRSenderKey: senderId,
    })

    const response = await this.waitUntilResponse<{
      WIRDestinationKey: string
      WIRMessageDataKey: ArrayBuffer
    }>((message) => {
      if (message.__selector !== '_rpc_applicationSentData:') {
        return false
      }

      return message.__argument.WIRDestinationKey === senderId
    })

    const message = bufferToObject<{
      method: 'Target.targetCreated'
      params: { targetInfo: { targetId: string; type: 'page' } }
    }>(response.__argument.WIRMessageDataKey)

    return message
  }

  forwardSocketData = (
    appId: string,
    pageId: number,
    senderId: string,
    targetId: string,
    id: number,
    message: any
  ) => {
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

    // this.receiveMessage()
  }
}
