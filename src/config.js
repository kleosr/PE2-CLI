import fs from 'fs';
import os from 'os';
import path from 'path';
import { DEFAULT_CONFIG, FILE_PERMISSIONS } from './constants.js';
import { writeJsonFileAtomically } from './utils/writeJsonFileAtomically.js';

export const CONFIG_DIR = path.join(os.homedir(), '.kleosr-pe2');
export const CONFIG_FILE = path.join(CONFIG_DIR, 'config.json');

export function ensureConfigDir() {
  if (!fs.existsSync(CONFIG_DIR)) {
    fs.mkdirSync(CONFIG_DIR, { recursive: true });
  }
}

export function loadConfig() {
  ensureConfigDir();
  if (fs.existsSync(CONFIG_FILE)) {
    try {
      const storedConfig = fs.readFileSync(CONFIG_FILE, 'utf-8');
      return JSON.parse(storedConfig);
    } catch (error) {
      return {};
    }
  }
  return {};
}

export function saveConfig(config) {
  ensureConfigDir();
  try {
    writeJsonFileAtomically(CONFIG_FILE, config, { mode: FILE_PERMISSIONS.configFile });
    fs.chmodSync(CONFIG_FILE, FILE_PERMISSIONS.configFile);
    return true;
  } catch (error) {
    return false;
  }
}

export function getDefaultConfig() {
  return { ...DEFAULT_CONFIG };
}

export const PROVIDER_ENV_VARS = {
  openai: 'OPENAI_API_KEY',
  anthropic: 'ANTHROPIC_API_KEY',
  google: 'GOOGLE_API_KEY',
  openrouter: 'OPENROUTER_API_KEY',
  ollama: 'OLLAMA_BASE_URL'
};

export function resolveApiKey(provider, configApiKey) {
  if (configApiKey?.trim()) return configApiKey;
  if (provider === 'ollama') {
    return process.env[PROVIDER_ENV_VARS.ollama] ?? null;
  }
  return process.env[PROVIDER_ENV_VARS[provider] ?? 'OPENROUTER_API_KEY'] ?? null;
}
