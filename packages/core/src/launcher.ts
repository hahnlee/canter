import { getDevices } from '@canter/connector'
import { BundleService, WIService } from './services'

export interface LaunchOptions {
  udid?: string
  bundle?: string
}

function findDevice(udid: string | undefined) {
  const devices = getDevices()

  if (devices.length === 0) {
    throw new Error('No devices available')
  }

  if (udid === undefined) {
    return devices[0]
  }

  const device = devices.find((device) => device.udid === udid)
  if (device === undefined) {
    throw new Error(`Cannot find device with udid: ${udid}`)
  }

  return device
}

export function launch({
  udid,
  bundle = 'com.apple.mobilesafari',
}: LaunchOptions = {}) {
  const device = findDevice(udid)
  device.connect()

  const service = device.startService('com.apple.webinspector')
  const wiService = new WIService(service)
  const response = wiService.reportIdentifier()

  if (
    response.WIRAutomationAvailabilityKey !==
    'WIRAutomationAvailabilityAvailable'
  ) {
    device.disconnect()
    throw new Error('Remote Automation is not activated')
  }

  const applications = wiService.getConnectedApplications()
  const availableApplications = applications;
  const targetApp = availableApplications.find(
    (app) => app.WIRApplicationBundleIdentifierKey === bundle
  )

  if (!targetApp) {
    device.disconnect()
    throw new Error(`Can't connect to (bundle: ${bundle})`)
  }

  const identifierKey = targetApp.WIRApplicationIdentifierKey
  return new BundleService(wiService, identifierKey)
}
