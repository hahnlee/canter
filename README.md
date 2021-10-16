# Canter
Canter ios safari remote debugging helper.
Developed for safari/webview e2e testing on iPhone.
Works only on macOS.

# Usage
```ts
import * as canter from '@canter/core'

const service = canter.launch({
  udid: '<device udid>', // optional
  bundle: '<app bundle id>', // optional (default com.apple.mobilesafari)
})

const page = await service.pages()[0];
page.goto('https://example.com');
```

# Packages
- `@canter/core`
- `@canter/connector`

## WIP
- `@canter/webdriver`
- `@canter/selenium`
- `@canter/vscode`

# Note
0.x version will not follow semver.
- minor version change has breaking changes
- patch version change has bug fixes and new features 

# License
```
Copyright 2021 Han Lee

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
