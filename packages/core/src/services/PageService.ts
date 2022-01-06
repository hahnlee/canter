import { v4 } from 'uuid'
import { WIService } from './WIService'

export class PageService {
  private service: WIService
  private bundleIdentifierKey: string
  private pageIdentifierKey: number
  private connectionId = v4()
  private initialized = false
  private id: number = 1
  private targetId: string = ''

  constructor(
    service: WIService,
    bundleIdentifierKey: string,
    pageIdentifierKey: number
  ) {
    this.service = service
    this.bundleIdentifierKey = bundleIdentifierKey
    this.pageIdentifierKey = pageIdentifierKey
    this.goto = this.goto.bind(this)
  }

  private initialize() {
    this.service.forwardIndicateWebView(
      this.bundleIdentifierKey,
      this.pageIdentifierKey,
      true
    )
    this.targetId = this.service.forwardSocketSetup(
      this.bundleIdentifierKey,
      this.pageIdentifierKey,
      this.connectionId
    ).params.targetInfo.targetId

    this.sendMessage('Inspector.enable')
    this.sendMessage('Page.enable')
    this.initialized = true
  }

  private sendMessage(method: string, params?: any) {
    this.service.forwardSocketData(
      this.bundleIdentifierKey,
      this.pageIdentifierKey,
      this.connectionId,
      this.targetId,
      this.id,
      {
        id: this.id,
        method,
        params,
      }
    )

    this.id = this.id + 1
  }

  goto(url: string) {
    if (!this.initialized) {
      this.initialize()
    }

    this.sendMessage('Page.navigate', {
      url,
    })
  }
}
