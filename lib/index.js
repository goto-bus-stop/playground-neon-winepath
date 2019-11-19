var addon = require('../native');

var config = new addon.WineConfig()
console.log('WINEPREFIX =', config.prefix())
console.log('Native for C:\\ =', config.toNativePath('c:\\'))
console.log('Wine path to /tmp =', config.toWinePath('/tmp'))
console.log()

if (process.argv[2] === '-u') {
  console.log(config.toWinePath(process.argv[3]))
} else if (process.argv[2] === '-w') {
  console.log(config.toNativePath(process.argv[3]))
}
