# @canter/core

Core API of canter

## Usage

```js
const canter = require('@canter/core')

(async () => {
  const target = await canter.launch({
    udid: '<DEVICE_UDID>',
    bundleId: 'com.apple.mobilesafari',
  })
  const page = await browser.newPage()

  await target.navigate('http://example.com')
  await target.close()
})()
```
