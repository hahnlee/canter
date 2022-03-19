import ios from '../src'

;(async () => {
  const [device] = await ios.devices()
  const [webview] = await device.webViews()
  const page = await webview.page()
  await page.goto('https://example.com')
  await webview.close()
})()
