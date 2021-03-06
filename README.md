# ctrlc-windows

Send CTRL-C to a Windows console application

Long running stateful processes like servers often hold a lot of
system resources that they need to operate. These are things like
communication ports, file descriptors, database connections, etc... At some point,
when it is time for that process to shut it needs to release all those
resources back to the operating system and exit. This process of
freeing up all the things that were being used is called "graceful
shutdown" and is really important to get right in systems that start
and stop processes repeatedly. On Unix systems, the method for
achieving a graceful shutdown of a process is to send it a signal
(usually either `SIGINT` or `SIGTERM`).

The process can then register a handler for this signal which releases
any resources that it is holding and exits.

The alternative to a graceful shutdown is a "forced termination". This
is what happens when a process is unable to respond to a interrupt or
termination signal. In this case, the process is ended immediately
without the opprotunity to release any of the resources it holds.

On Unix systems, when you invoke `process.kill()` on a `ChildProcess`
object, it sends the `SIGTERM` signal. That way, if the process has a
`SIGTERM` handler it has an opportunity to perform a graceful
shutdown.

Windows on the other hand, does not have a method to send signals to a
process, so when you invoke the same `process.kill()` function on
_that_ platform, what you end up with is a forceful termination
without any cleanup. However, Windows _does_ have a method for
applications to shutdown gracefully, namely hitting `CTRL-C` on the
console to which the process is attached. And that's what this is all
about. In fact, whenever you hit `CTRL-C` on a windows console that is
running a `node` application, `node` will translate that `CTRL-C` into
a `SIGINT` and dispatch any `SIGINT` handlers that it has
registered. Since most well-behaved long-running Nodejs processes
respond to `SIGINT`, this allows them to shutdown too.

This module exports a single `ctrlc` function which accepts a process
id, and sends a `CTRL-C` event to that console.

> Note: This is a no-op on non-windows platforms

## Usage

``` javascript
import { ctrlc } from 'ctrlc-windows';

export function gracefulShutdown(process) {
  if (process.platform === 'win32') {
    ctrlc(process.pid);
  } else {
    process.kill();
  }
}
```
