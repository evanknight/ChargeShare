#!/usr/bin/env node
import assert from 'node:assert/strict';
import { spawnSync } from 'node:child_process';
import { copyFileSync, mkdirSync, mkdtempSync, rmSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { dirname, join, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';

const root = resolve(dirname(fileURLToPath(import.meta.url)), '../..');
const sandbox = mkdtempSync(join(tmpdir(), 'chargeshare-guards-'));
function run(command, args, expected = 0) {
  const result = spawnSync(command, args, { cwd: sandbox, encoding: 'utf8' });
  // Never print captured scanner output, which can contain sensitive context.
  assert.equal(result.status, expected, `${command} ${args.slice(0, 2).join(' ')} returned ${result.status}; expected ${expected}`);
  return result;
}
function put(path, content) {
  mkdirSync(dirname(join(sandbox, path)), { recursive: true });
  writeFileSync(join(sandbox, path), content);
}
try {
  run('git', ['init', '--quiet']);
  for (const file of ['.gitignore', '.gitleaks.toml', 'scripts/security/check-paths.mjs', 'scripts/security/scan.sh']) {
    mkdirSync(dirname(join(sandbox, file)), { recursive: true });
    copyFileSync(join(root, file), join(sandbox, file));
  }
  mkdirSync(join(sandbox, '.tools/bin'), { recursive: true });
  copyFileSync(join(root, '.tools/bin/gitleaks'), join(sandbox, '.tools/bin/gitleaks'));
  put('.env.example', '# Synthetic empty template\nEXAMPLE_TOKEN=\n');
  put('README.md', 'Synthetic security test repository.\n');
  run('git', ['add', '.']);
  run('bash', ['scripts/security/scan.sh', 'staged']);
  run('git', ['-c', 'user.name=Security Test', '-c', 'user.email=security@example.invalid', 'commit', '--quiet', '-m', 'Synthetic safe fixture']);
  run('bash', ['scripts/security/scan.sh', 'history']);
  let count = 0;
  for (const path of ['.env', '.env.production', 'config/.env.example', 'production.env', '.npmrc', 'credentials.json', 'keys/server.pem', 'database.sqlite-wal', 'telemetry/session.json', 'data/raw/session.json', 'capture.har', 'token.json', 'service-account.json', 'storage-state.json', 'private-data/session.json', '.cargo/credentials.toml', 'target/debug/app']) {
    put(path, 'synthetic data only\n');
    run('git', ['check-ignore', '--quiet', path]);
    run('git', ['add', '--force', path]);
    run('node', ['scripts/security/check-paths.mjs'], 1);
    run('git', ['rm', '--cached', '--quiet', path]);
    count++;
  }
  // Deliberately inert token-shaped test data, built only inside the temporary repo.
  const synthetic = ['ghp', '0123456789abcdef0123456789abcdef012345'].join('_');
  put('synthetic.txt', `fixture = "${synthetic}"\n`);
  run('git', ['add', 'synthetic.txt']);
  run('bash', ['scripts/security/scan.sh', 'staged'], 1);
  run('git', ['rm', '--cached', '--quiet', 'synthetic.txt']);
  run('bash', ['scripts/security/scan.sh', 'staged']);
  console.log(`Security guard tests passed: safe staged/history scans, ${count} force-added sensitive paths blocked, synthetic token blocked`);
} finally {
  rmSync(sandbox, { recursive: true, force: true });
}
