import { PageService } from 'canter-core'

export class Page {
  private service: PageService

  constructor(service: PageService) {
    this.service = service
  }

  goto(url: string) {
    return this.service.goto(url)
  }
}
