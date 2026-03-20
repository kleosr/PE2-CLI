import { createRequire } from 'module';
import { dirname, join } from 'path';
import { fileURLToPath } from 'url';

const require = createRequire(import.meta.url);
const packageJsonPath = join(dirname(fileURLToPath(import.meta.url)), '..', 'package.json');
const { version: CLI_SEMVER } = require(packageJsonPath);

/** npm semver; single source: package.json */
export { CLI_SEMVER };

/** Product generation label shown in UI (PE2 pipeline / release line). */
export const PE2_CODE_GENERATION = 'V4';

export function cliVersionWithPrefix() {
  return `v${CLI_SEMVER}`;
}

export function cliBannerSubtitle() {
  return `${cliVersionWithPrefix()} • Code ${PE2_CODE_GENERATION} • Prompt Engineering 2.0`;
}
