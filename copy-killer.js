if (process.platform !== "win32") {
  console.log(`skipping copy for platform [${process.platform}]`);
  process.exit(0);
}

const { copyFile } = require('fs').promises;
const { join } = require('path');

const [,,profile] = process.argv;
const source = join(__dirname, 'native', 'target', profile, 'killer.exe');
const destination = join(__dirname, 'native', 'killer.exe');

console.log(`killer.exe: ${destination}`);

copyFile(source, destination)
  .then(() => process.exit(0))
  .catch((err) => {
    console.log(err);
    process.exit(1);
  });
