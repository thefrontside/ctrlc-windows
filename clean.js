const CLI = require('neon-cli/lib/cli').default;
const { join } = require('path');
const { rmdir } = require('fs').promises;

async function main() {
  if (process.platform === 'win32') {
    let cli = new CLI([,,'clean'], process.cwd());

    await cli.exec();

    await rmdir(join(process.cwd(), 'dist'), {
      recursive: true
    });
  }
}

main()
  .catch(e => {
    console.error(e);
    process.exit(1);
  })
  .then(() => {
    process.exit(0);
  });
