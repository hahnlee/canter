# @canter/core
Core API of canter

## Usage
```js
import { DeviceListener } from '@canter/core'

const listener = new DeviceListener()
const devices = listener.getDevices()
const device = devices[0]
device.connect()

await device.navigate('http://example.com')

device.disconnect()
```
