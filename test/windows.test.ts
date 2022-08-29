import { ChildProcess, spawn } from 'child_process';
import expect from 'expect';
import { ctrlc } from '../lib/index';

if (process.platform === "win32") {
  describe('ctrlc', () => {
    describe('on a process that does not exist', () => {
      it('is just a no-op', async() => {
        ctrlc(1);
      });
    });

    describe('on a running process', () => {
      let child: ChildProcess;
      let promise: Promise<unknown>;
      beforeEach(async () => {
        promise = new Promise((resolve, reject)=> {
          child = spawn("node", ["./test/fixtures/daemon.js"], {
            stdio: 'pipe',
            shell: false,
            windowsHide: true
          });
          child.on('error', reject);
          child.on('exit', () => resolve('process complete'));
        });
        await new Promise(resolve => setTimeout(resolve, 100));
        ctrlc(child.pid);
        child.stdin?.end();
      });
      it('causes that process to exit', async () => {
        await expect(promise).resolves.toEqual('process complete');
      });
    });
  });
}
