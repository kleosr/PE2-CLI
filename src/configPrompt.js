import inquirer from 'inquirer';
import { PROVIDERS } from './providers/index.js';
import { CONFIG_FILE, saveConfig } from './config.js';

const PROVIDER_CHOICES = [
  { name: `${PROVIDERS.openai.name} - Direct OpenAI API`, value: 'openai' },
  { name: `${PROVIDERS.anthropic.name} - Direct Anthropic API`, value: 'anthropic' },
  { name: `${PROVIDERS.google.name} - Direct Google AI API`, value: 'google' },
  { name: `${PROVIDERS.openrouter.name} - Access multiple providers`, value: 'openrouter' },
  { name: `${PROVIDERS.ollama.name} - Local Ollama`, value: 'ollama' }
];

function validateApiKey(input) {
  const trimmed = input.trim();
  if (!trimmed) return 'API key is required';
  if (trimmed.length < 10) return 'API key seems too short. Please verify.';
  return true;
}

function validateModelName(input) {
  const trimmed = input.trim();
  if (!trimmed) return 'Model name is required';
  if (trimmed.length < 2) return 'Model name must be at least 2 characters';
  return true;
}

async function promptForProvider() {
  const { provider } = await inquirer.prompt([{
    type: 'list',
    name: 'provider',
    message: 'Select your AI provider:',
    choices: PROVIDER_CHOICES,
    default: 'openrouter'
  }]);
  return provider;
}

async function promptForOllamaUrl(defaultUrl) {
  const { baseURL } = await inquirer.prompt([{
    type: 'input',
    name: 'baseURL',
    message: 'Enter your Ollama base URL (press Enter for default):',
    default: defaultUrl
  }]);
  return baseURL.trim();
}

async function promptForApiKey(keyLabel) {
  const resp = await inquirer.prompt([{
    type: 'password',
    name: 'apiKey',
    message: `Enter your ${keyLabel}:`,
    mask: '*',
    validate: validateApiKey
  }]);
  return resp.apiKey.trim();
}

async function promptForModel(providerConfig) {
  const { model } = await inquirer.prompt([{
    type: 'list',
    name: 'model',
    message: 'Select a model:',
    choices: [
      ...providerConfig.models.map(m => ({
        name: m === providerConfig.defaultModel ? `${m} (recommended)` : m,
        value: m
      })),
      { name: '📝 Enter Custom Model', value: 'custom' }
    ],
    default: providerConfig.defaultModel
  }]);
  return model;
}

async function promptForCustomModel() {
  const { customModel } = await inquirer.prompt([{
    type: 'input',
    name: 'customModel',
    message: 'Enter custom model name:',
    validate: validateModelName
  }]);
  return customModel.trim();
}

function displayConfigSaved(themeManager, providerConfig, config) {
  console.log(themeManager.color('success')(`\n✅ Configuration saved!`));
  console.log(themeManager.color('info')(`🌐 Provider: ${providerConfig.name}`));
  console.log(themeManager.color('info')(`📝 Model: ${config.model}`));
  console.log(themeManager.color('info')(`🔑 API Key: ${config.apiKey.substring(0, 8)}...`));
  console.log(themeManager.color('info')(`📁 Config saved to: ${CONFIG_FILE}\n`));
}

function handleConfigError(themeManager, error) {
  if (error instanceof Error && error.name === 'ExitPromptError') {
    console.log('\n👋 Configuration cancelled.');
    return null;
  }
  if (error.isTtyError) {
    console.log(themeManager.color('error')('❌ Interactive prompts are not supported in this environment.'));
    console.log(themeManager.color('warning')('Please run this in a proper terminal.'));
  } else {
    console.log(themeManager.color('error')(`❌ Configuration error: ${error.message}`));
  }
  return null;
}

async function saveAndReturnConfig(themeManager, providerConfig, config) {
  if (saveConfig(config)) {
    displayConfigSaved(themeManager, providerConfig, config);
    return config;
  }
  console.log(themeManager.color('error')('Failed to save configuration.'));
  return null;
}

async function collectProviderCredentials(provider, providerConfig) {
  return provider === 'ollama'
    ? await promptForOllamaUrl(providerConfig.baseURL)
    : await promptForApiKey(providerConfig.keyLabel);
}

async function resolveModelSelection(providerConfig) {
  const model = await promptForModel(providerConfig);
  return model === 'custom' ? await promptForCustomModel() : model;
}

export async function promptForConfig(themeManager) {
  console.log(themeManager.color('warning')('\nConfiguration Setup'));
  console.log(themeManager.color('info')("Configure your AI provider and API settings.\n"));

  try {
    const provider = await promptForProvider();
    const providerConfig = PROVIDERS[provider];
    const apiKey = await collectProviderCredentials(provider, providerConfig);
    const finalModel = await resolveModelSelection(providerConfig);
    const config = { provider, apiKey, model: finalModel };
    return saveAndReturnConfig(themeManager, providerConfig, config);
  } catch (error) {
    return handleConfigError(themeManager, error);
  }
}
