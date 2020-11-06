if (process.platform !== 'win32') {
  console.log('skipping build for non-windows platform');
  process.exit(0);
}

require('make-promises-safe');
const CLI = require('neon-cli/lib/cli').default;
const { exec } = require('child_process');
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
  let code = await new Promise((resolve, reject) => {
    let child = exec(cargo, {
      cwd: join(__dirname, "native"),
      stdio: 'inherit'
    });
    child.on('exit', (code) => resolve(code));
    child.on('error', reject);
  });

  if (code !== 0) {
    process.exit(code || 1);
  }

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
