import fs from 'fs';
import os from 'os';
import path from 'path';
import chalk from 'chalk';

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
      const configData = fs.readFileSync(CONFIG_FILE, 'utf-8');
      return JSON.parse(configData);
    } catch (error) {
      console.log(chalk.yellow('Warning: Could not load config file, using defaults.'));
      return {};
    }
  }
  return {};
}

export function saveConfig(config) {
  ensureConfigDir();
  try {
    fs.writeFileSync(CONFIG_FILE, JSON.stringify(config, null, 2), { encoding: 'utf-8', mode: 0o600 });
    fs.chmodSync(CONFIG_FILE, 0o600);
    return true;
  } catch (error) {
    console.log(chalk.red(`❌ Error saving config: ${error.message}`));
    return false;
  }
}

export function getDefaultConfig() {
  return {
    apiKey: null,
    model: 'openai/gpt-4o-mini',
    provider: 'openrouter'
  };
}

export const PROVIDER_ENV_VARS = {
  openai: 'OPENAI_API_KEY',
  anthropic: 'ANTHROPIC_API_KEY',
  google: 'GOOGLE_API_KEY',
  openrouter: 'OPENROUTER_API_KEY',
  ollama: 'OLLAMA_BASE_URL'
};

export function resolveApiKey(provider, configApiKey) {
  if (configApiKey && configApiKey.trim()) return configApiKey;
  if (provider === 'ollama') {
    return process.env[PROVIDER_ENV_VARS.ollama] || null;
  }
  const envVar = PROVIDER_ENV_VARS[provider] || 'OPENROUTER_API_KEY';
  return process.env[envVar] || null;
}


