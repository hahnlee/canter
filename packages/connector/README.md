# @canter/connector
`MobileDevice.framework` connector for node.js

## Usage
```js
import { DeviceConnector } from '@canter/connector'
import { v1 } from 'uuid'

const connector = new DeviceConnector()

const handleConnected = async (device) => {
  device.connect()

  // NOTE: only "com.apple.inspector" tested now
  const session = device.session('com.apple.inspector')
  const received = await session.sendMessage({
    '_rpc_reportIdentifier': {
      'WIRConnectionIdentifierKey': v1(),
    },
  })

  device.disconnect()
}

connector.addEventListener('connected', handleConnected)
connector.addEventListener('connected', handleConnected)

connector.subscribe()

connector.unsubscribe();
```
