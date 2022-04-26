import { WIService, BundleService } from 'canter-core'
import { AMDevice } from 'canter-connector'

import { IOSWebView } from './webview'

export class IOSDevice {
  private device: AMDevice

  constructor(device: AMDevice) {
    this.device = device
  }

  webViews = async () => {
    this.device.connect()

    const service = this.device.startService('com.apple.webinspector')
    const wiService = new WIService(service)

    await wiService.reportIdentifier()

    const applications = await wiService.getConnectedApplications()

    const bundles = applications
      .filter(
        (app) =>
          app.WIRApplicationBundleIdentifierKey !== 'com.apple.mobile.lockdownd'
      )
      .flatMap((app) => new BundleService(
        wiService,
        app.WIRApplicationIdentifierKey
      ))

      const pages = await Promise.all(bundles.map(async bundle => {
        const pages = await bundle.pages()
        return pages.map((page) => new IOSWebView(page))
      }))

      return pages.flat()
  }

  close = async () => {
    this.device.disconnect()
  }
}
