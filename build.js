if (process.platform !== 'win32') {
  console.log('skipping build for non-windows platform');
  process.exit(0);
}

require('make-promises-safe');
const CLI = require('neon-cli/lib/cli').default;
const { exec } = require('child_process');
const { copyFile, mkdir } = require('fs').promises;
const { join } = require('path');

const [,,profile, napiVersion = '4'] = process.argv;

async function main(argv) {
  let isRelease = argv.find(arg => arg === 'release');
  let args = isRelease ? ['--release'] : [];

  let cli = new CLI([,,'build', ...args], process.cwd());

  await cli.exec();

  // build the killer binary
  let profile = isRelease ? 'release' : 'debug';

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

  await mkdir(`dist/napi-${napiVersion}`, { recursive: true });

  await copyFile(
    join(__dirname, 'native', 'target', profile, 'killer.exe'),
    join(__dirname, `dist/napi-${napiVersion}`, 'killer.exe')
  );
  await copyFile(
    join(__dirname, 'native', 'index.node'),
    join(__dirname, `dist/napi-${napiVersion}`, 'index.node')
  );
}

main(process.argv)
  .catch(e => {
    console.error(e);
    process.exit(1);
  })
  .then(() => {
    process.exit(0);
  });
