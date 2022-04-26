import { PageService } from 'canter-core'
import { Page } from './page'

export class IOSWebView {
  private service: PageService

  constructor(service: PageService) {
    this.service = service
  }
  
  page = async () => {
    return new Page(this.service)
  }

  close = async () => {
    this.service.close()
  }
}
