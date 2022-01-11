import { WIService, BundleService } from '@canter/core'
import { AMDevice } from '@canter/connector'

import { IOSWebView } from './webview'

export class IOSDevice {
  private device: AMDevice

  constructor(device: AMDevice) {
    this.device = device
  }

  webViews = async () => {
    // FIXME: (@hahnlee) use connected state
    this.device.connect()

    const service = this.device.startService('com.apple.webinspector')
    const wiService = new WIService(service)

    wiService.reportIdentifier()

    const applications = wiService
      .getConnectedApplications()
      .filter(
        (app) =>
          app.WIRApplicationBundleIdentifierKey !== 'com.apple.mobile.lockdownd'
      )

    return applications.flatMap((app) => {
      const bundleService = new BundleService(
        wiService,
        app.WIRApplicationIdentifierKey
      )

      const pages = bundleService.pages()
      return pages.map((page) => new IOSWebView(page))
    })
  }

  close = async () => {
    this.device.disconnect()
  }
}
