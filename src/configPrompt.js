import inquirer from 'inquirer';
import chalk from 'chalk';
import { PROVIDERS } from './providers/index.js';
import { CONFIG_FILE, saveConfig } from './config.js';

export async function promptForConfig(rl) {
  console.log(chalk.hex('#FFD93D')('\n🔧 Configuration Setup'));
  console.log(chalk.hex('#B19CD9')("Let's configure your AI provider and API settings.\n"));

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
          validate: (input) => input.trim() ? true : 'API key is required'
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
          validate: (input) => input.trim() ? true : 'Model name is required'
        }
      ]);
      finalModel = customModel.trim();
    }

    const config = { provider, apiKey, model: finalModel };

    if (saveConfig(config)) {
      console.log(chalk.hex('#50E3C2')(`\n✅ Configuration saved!`));
      console.log(chalk.hex('#B19CD9')(`🌐 Provider: ${providerConfig.name}`));
      console.log(chalk.hex('#B19CD9')(`📝 Model: ${config.model}`));
      console.log(chalk.hex('#B19CD9')(`🔑 API Key: ${config.apiKey.substring(0, 8)}...`));
      console.log(chalk.hex('#B19CD9')(`📁 Config saved to: ${CONFIG_FILE}\n`));
      return config;
    } else {
      console.log(chalk.red('❌ Failed to save configuration.'));
      return null;
    }
  } catch (error) {
    if (error.isTtyError) {
      console.log(chalk.red('❌ Interactive prompts are not supported in this environment.'));
      console.log(chalk.yellow('Please run this in a proper terminal.'));
    } else {
      console.log(chalk.red(`❌ Configuration error: ${error.message}`));
    }
    return null;
  }
}


