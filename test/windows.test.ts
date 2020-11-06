import { ChildProcess, spawn } from 'child_process';
import expect from 'expect';

import { ctrlc } from '../lib/index';

if (process.platform === "win32") {
  describe('ctrlc', () => {
    describe('on a process that does not exist', () => {
      it('is is just a no-op', async() => {
        ctrlc(1);
      });
    });

    describe('on a running process', () => {
      let child: ChildProcess;
      let promise: Promise<unknown>;
      let stdout: string;
      beforeEach(async () => {
        stdout = '';
        promise = new Promise((resolve, reject)=> {
          child = spawn("node", ["./test/fixtures/daemon.js"], {
            stdio: 'pipe',
            windowsHide: true
          });
          child.on('error', reject);
          child.on('exit', resolve);
          child.stdout?.on('data', chunk => stdout += chunk);
        });
        await new Promise(resolve => setTimeout(resolve, 100));

        ctrlc(child.pid);
      });
      it('causes that process to exit', async () => {
        await promise;
        expect(stdout).toContain("exiting...")
      });
    });
  });
}
