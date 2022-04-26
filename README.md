# Canter (WIP)
(WIP) iOS Safari and webview testing library with [playwright](https://playwright.dev/docs/api/class-android) compatible API.

Works only on macOS.

# Usage
```ts
import ios from 'canter-playwright'

;(async () => {
  const [device] = await ios.devices()
  const [webview] = await device.webViews()
  const page = await webview.page()
  await page.goto('https://example.com')
  await webview.close()
})()
```

# Goal
Provide [playwright android](https://playwright.dev/docs/api/class-android) compatible API.

# Current status
- Connecting to a iOS device.
- Connecting to webview or iOS Safari.
- Navigate webpage

# Packages
- `canter-core`
- `canter-connector`
- `canter-playwright`

# License
```
Copyright 2021-2022 Han Lee

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
```
