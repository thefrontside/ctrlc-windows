const { platform } = require('os');
const { join } = require('path');

const native = platform() === 'win32' ? require('../native') : require('./posix');

module.exports = {
  ctrlc(pid) {
    try {
      // don't even attempt if the
      // process is not running
      if (process.kill(pid, 0)) {
        native.ctrlc(pid, join(__dirname, "..", "native/killer.exe"));
      }
    } catch (error) {
      if (error.code !== 'ESRCH') {
        throw error;
      }
    }
  }
};
