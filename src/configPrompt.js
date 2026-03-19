import inquirer from 'inquirer';
import { PROVIDERS } from './providers/index.js';
import { CONFIG_FILE, saveConfig } from './config.js';

export async function promptForConfig(themeManager) {
  console.log(themeManager.color('warning')('\n🔧 Configuration Setup'));
  console.log(themeManager.color('info')("Let's configure your AI provider and API settings.\n"));

  try {
    const { provider } = await inquirer.prompt([
      {
        type: 'list',
        name: 'provider',
        message: 'Select your AI provider:',
        choices: [
          { name: `${PROVIDERS.openai.name} - Direct OpenAI API`, value: 'openai' },
          { name: `${PROVIDERS.anthropic.name} - Direct Anthropic API`, value: 'anthropic' },
          { name: `${PROVIDERS.google.name} - Direct Google AI API`, value: 'google' },
          { name: `${PROVIDERS.openrouter.name} - Access multiple providers`, value: 'openrouter' },
          { name: `${PROVIDERS.ollama.name} - Local Ollama`, value: 'ollama' }
        ],
        default: 'openrouter'
      }
    ]);

    const providerConfig = PROVIDERS[provider];

    let apiKey = '';
    if (provider === 'ollama') {
      const { baseURL } = await inquirer.prompt([
        {
          type: 'input',
          name: 'baseURL',
          message: 'Enter your Ollama base URL (press Enter for default):',
          default: providerConfig.baseURL
        }
      ]);
      apiKey = baseURL.trim();
    } else {
      const resp = await inquirer.prompt([
        {
          type: 'password',
          name: 'apiKey',
          message: `Enter your ${providerConfig.keyLabel}:`,
          mask: '*',
          validate: async (input) => {
            const trimmed = input.trim();
            if (!trimmed) {
              return 'API key is required';
            }
            if (trimmed.length < 10) {
              return 'API key seems too short. Please verify.';
            }
            return true;
          }
        }
      ]);
      apiKey = resp.apiKey.trim();
    }

    const { model } = await inquirer.prompt([
      {
        type: 'list',
        name: 'model',
        message: 'Select a model:',
        choices: [
          ...providerConfig.models.map(model => ({
            name: model === providerConfig.defaultModel ? `${model} (recommended)` : model,
            value: model
          })),
          { name: '📝 Enter Custom Model', value: 'custom' }
        ],
        default: providerConfig.defaultModel
      }
    ]);

    let finalModel = model;
    if (model === 'custom') {
      const { customModel } = await inquirer.prompt([
        {
          type: 'input',
          name: 'customModel',
          message: 'Enter custom model name:',
          validate: async (input) => {
            const trimmed = input.trim();
            if (!trimmed) {
              return 'Model name is required';
            }
            if (trimmed.length < 2) {
              return 'Model name must be at least 2 characters';
            }
            return true;
          }
        }
      ]);
      finalModel = customModel.trim();
    }

    const config = { provider, apiKey, model: finalModel };

    if (saveConfig(config)) {
      console.log(themeManager.color('success')(`\n✅ Configuration saved!`));
      console.log(themeManager.color('info')(`🌐 Provider: ${providerConfig.name}`));
      console.log(themeManager.color('info')(`📝 Model: ${config.model}`));
      console.log(themeManager.color('info')(`🔑 API Key: ${config.apiKey.substring(0, 8)}...`));
      console.log(themeManager.color('info')(`📁 Config saved to: ${CONFIG_FILE}\n`));
      return config;
    } else {
      console.log(themeManager.color('error')('❌ Failed to save configuration.'));
      return null;
    }
  } catch (error) {
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
}
