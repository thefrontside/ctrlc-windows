module.exports = {
  ctrlc() {
    let error = new Error('tried to invoke windows-specific ctrlc on a non-windows platform');
    error.name = 'PlatformError';
    throw error;
  }
}
