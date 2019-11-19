# playground-neon-winepath

trying out neon

## Sample

See [./lib/index.js](./lib/index.js).

```js
var { WineConfig } = require('./native')
var config = new WineConfig()
console.log('WINEPREFIX =', config.prefix())
console.log('Native for C:\\ =', config.toNativePath('c:\\'))
console.log('Wine path to /tmp =', config.toWinePath('/tmp'))
```

## License
[winepath](https://github.com/goto-bus-stop/winepath) is released under the MPL-2.0 license. What little code exists in this repository is also made available under the MPL-2.0 license.
