if (process.platform !== 'win32') {
  console.log('skipping build for non-windows platform');
}
require('make-promises-safe');
const CLI = require('neon-cli/lib/cli').default;
const { exec } = require('child-process-promise');

const { copyFile } = require('fs').promises;
const { join } = require('path');

const [,,profile] = process.argv;

async function main(argv) {
  let isRelease = argv.find(arg => arg === 'release');
  let args = isRelease ? ['--release'] : [];

  let cli = new CLI([,,'build', ...args], process.cwd());

  await cli.exec();

  // build the killer binary
  let profile = isRelease ? 'release' : 'debug';
  const source = join(__dirname, 'native', 'target', profile, 'killer.exe');
  const destination = join(__dirname, 'native', 'killer.exe');
  let cargo = ['cargo build', ...args, '--bin killer'].join(' ');
  await exec(cargo, {
    cwd: join(__dirname, "native"),
    stdio: 'inherit'
  });

  await copyFile(source, destination);
}

main(process.argv)
  .catch(e => {
    console.error(e);
    process.exit(1);
  })
  .then(() => {
    process.exit(0);
  });
