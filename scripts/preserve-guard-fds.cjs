// Node preload for Wrangler custom builds. Keep the original validated Guard
// capabilities at their original numbers across Node's default close-fds spawn.
const cp = require('node:child_process');
const tokenNames = ['DISK_GUARD_RESERVATION', 'DISK_GUARD_CUSTODY'];
const candidates = tokenNames.flatMap((name) => {
  if (!process.env[name]) return [];
  const value = JSON.parse(process.env[name]);
  if (!Number.isSafeInteger(value.fd) || value.fd < 3) throw new Error('Invalid Guard descriptor candidate');
  return [value.fd];
});
if (candidates.length) {
  function withDescriptors(options = {}, descriptors = candidates) {
    const mode = options.stdio ?? 'pipe';
    const stdio = Array.isArray(mode) ? [...mode] : [mode, mode, mode];
    for (const fd of descriptors) {
      while (stdio.length <= fd) stdio.push('ignore');
      if (stdio[fd] !== 'ignore' && stdio[fd] !== fd) throw new Error('Guard descriptor conflicts with child stdio');
      stdio[fd] = fd;
    }
    return { ...options, stdio };
  }
  const status = cp.spawnSync('disk-guard', ['inheritance-status'], withDescriptors({ encoding: 'utf8' }));
  if (status.status !== 0) throw new Error(`Guard inheritance validation failed: ${status.stderr}`);
  const receipt = JSON.parse(status.stdout);
  if (receipt.status !== 'VALID' || !Array.isArray(receipt.pass_fds) || receipt.pass_fds.some((fd) => !candidates.includes(fd))) throw new Error('Unexpected Guard inheritance receipt');
  for (const name of ['spawn', 'spawnSync']) {
    const original = cp[name];
    cp[name] = function(command, args, options) {
      if (!Array.isArray(args)) return original.call(this, command, withDescriptors(args, receipt.pass_fds));
      return original.call(this, command, args, withDescriptors(options, receipt.pass_fds));
    };
  }
  require('node:module').syncBuiltinESMExports();
}

module.exports = { spawn: cp.spawn, spawnSync: cp.spawnSync };
