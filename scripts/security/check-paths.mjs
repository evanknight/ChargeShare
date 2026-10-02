#!/usr/bin/env node
import { execFileSync } from 'node:child_process';

// Inspect the index, not the working tree: force-adding an ignored file is blocked.
const paths = execFileSync('git', ['ls-files', '-z'], { encoding: 'utf8' })
  .split('\0').filter(Boolean);
const forbidden = [
  /(^|\/)\.env(?:\..*)?$/i,
  /(^|\/)[^/]*\.env(?:\..*)?$/i,
  /(^|\/)(?:\.envrc|\.npmrc|\.netrc|\.git-credentials)$/i,
  /(^|\/)(?:\.auth|\.aws|\.ssh|\.config|\.codex|\.direnv|node_modules|\.tools)(\/|$)/i,
  /(^|\/)(?:credentials[^/]*|secrets[^/]*|id_rsa[^/]*|id_ed25519[^/]*)$/i,
  /(^|\/)(?:tokens?\.json|oauth\.json|service[-_]account[^/]*\.json|storage-state[^/]*\.json|cookies[^/]*\.txt)$/i,
  /\.(?:pem|key|p12|pfx|jks|keystore|crt|cer|secret|tokens?|db|sqlite3?|dump|bak|sql|log|har|pcap|pcapng)$/i,
  /\.(?:db|sqlite3?)-[^/]*$/i,
  /(^|\/)(?:backups|exports|private|private-data|local|raw-data|vehicle-data|telemetry|location-history|charging-history|security-reports)(\/|$)/i,
  /(^|\/)data\/(?:raw|private)(\/|$)/i,
  /(^|\/)(?:\.replit|replit\.nix|gitleaks-report\.[^/]+)$/i,
];
const blocked = paths.filter(path => path !== '.env.example' && forbidden.some(rule => rule.test(path)));
if (blocked.length) {
  console.error('Blocked: sensitive or local-only paths are staged/tracked. Remove them from the index.');
  for (const path of blocked) console.error(`  ${JSON.stringify(path)}`);
  process.exit(1);
}
console.log('Sensitive-path check passed');
