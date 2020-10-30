const { ctrlc } = require('../native');
const { join } = require('path');
module.exports = {
  ctrlc(pid) {
    try {
      // don't even attempt if the
      // process is not running
      if (process.kill(pid, 0)) {
        ctrlc(pid, join(__dirname, "..", "native/killer.exe"));
      }
    } catch (error) {
      if (error.code !== 'ESRCH') {
        throw error;
      }
    }
  }
};
