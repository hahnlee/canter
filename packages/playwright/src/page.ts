import { PageService } from '@canter/core'

export class Page {
  private service: PageService

  constructor(service: PageService) {
    this.service = service
  }

  async goto(url: string) {
    this.service.goto(url)
  }
}
