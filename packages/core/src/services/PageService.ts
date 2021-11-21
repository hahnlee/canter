// TODO: (@hahnlee)
export class PageService {
  private uri: string

  constructor(uri: string) {
    this.uri = uri

    this.goto = this.goto.bind(this)
    this.url = this.url.bind(this)
  }

  goto(url: string) {}

  url() {
    return this.uri;
  }
}
