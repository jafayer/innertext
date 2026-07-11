'use strict';

const { existsSync } = require('node:fs');
const { join } = require('node:path');

function isMusl() {
  // Node >= 12 exposes glibc runtime info through process.report.
  if (process.platform !== 'linux') return false;
  if (process.report && typeof process.report.getReport === 'function') {
    const report = process.report.getReport();
    return !(report && report.header && report.header.glibcVersionRuntime);
  }
  return false;
}

function localCandidates() {
  const platform = process.platform;
  const arch = process.arch;

  if (platform === 'linux' && arch === 'x64') {
    return isMusl()
      ? ['innertext.linux-x64-musl.node', 'innertext.x86_64-unknown-linux-musl.node']
      : ['innertext.linux-x64-gnu.node', 'innertext.x86_64-unknown-linux-gnu.node'];
  }
  if (platform === 'darwin' && arch === 'x64') {
    return ['innertext.darwin-x64.node', 'innertext.x86_64-apple-darwin.node'];
  }
  if (platform === 'darwin' && arch === 'arm64') {
    return ['innertext.darwin-arm64.node', 'innertext.aarch64-apple-darwin.node'];
  }
  if (platform === 'win32' && arch === 'x64') {
    return ['innertext.win32-x64-msvc.node', 'innertext.x86_64-pc-windows-msvc.node'];
  }
  return [];
}

function loadNativeBinding() {
  for (const file of localCandidates()) {
    const full = join(__dirname, file);
    if (existsSync(full)) {
      return require(full);
    }
  }

  // Fallback for local dev builds without --platform.
  const fallback = join(__dirname, 'innertext.node');
  if (existsSync(fallback)) {
    return require(fallback);
  }

  const tried = localCandidates().concat(['innertext.node']).join(', ');
  throw new Error(
    `Failed to load innertext native binding for ${process.platform}/${process.arch}. Tried: ${tried}. ` +
      'Reinstall the package or build from source with `npm run build:release`.'
  );
}

const { HtmlDocument, innerText, outerText, textContent } = loadNativeBinding();

module.exports = {
  HtmlDocument,
  innerText,
  outerText,
  textContent,
};
