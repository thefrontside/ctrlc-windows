const { execFileSync } = require("child_process");
const { platform, arch } = require("os");
const { join } = require("path");

const isArm64 = arch() === "arm64"; // ? "arm64" : "x64";
const archDistDirName = isArm64 ? "arm64" : "x64";
const isWindows = platform() === "win32";

// this would be more friendly to bundlers
const native = isWindows
  ? isArm64
    ? require("../dist/arm64/ctrlc-windows.node")
    : require("../dist/x64/ctrlc-windows.node")
  : null;

/**
 * workaround for node-loader users; because node-loader only copies .node files,
 * devs using them will need to manually copy process-killer.exe into their app bundle
 * and provide the path to it when calling ctrlc()
 */
const defaultKillerPath = join(__dirname, "..", `dist/${archDistDirName}/process-killer.exe`);

module.exports = {
  ctrlc(pid, processKillerPath = defaultKillerPath) {
    if (!isWindows) {
      let error = new Error(
        "tried to invoke windows-specific ctrlc on a non-windows platform",
      );
      error.name = "PlatformError";
      throw error;
    }
    try {
      // don't even attempt if the
      // process is not running
      if (process.kill(pid, 0)) {
        native.ignoreSignal(true);
        execFileSync(processKillerPath, [pid.toString()]);
        native.ignoreSignal(false);
      }
    } catch (error) {
      if (error.code !== "ESRCH") {
        throw error;
      }
    }
  },
};

