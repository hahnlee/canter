import { PageService } from './PageService'
import { WIService } from './WIService'

export class BundleService {
  private service: WIService
  private identifierKey: string

  constructor(service: WIService, identifierKey: string) {
    this.service = service
    this.identifierKey = identifierKey

    this.pages = this.pages.bind(this)
  }

  pages() {
    const response = this.service.forwardGetListing(this.identifierKey)
    const listing = Object.values(response.WIRListingKey)
    return listing.map(
      (list) =>
        new PageService(
          this.service,
          this.identifierKey,
          list.WIRPageIdentifierKey
        )
    )
  }
}
