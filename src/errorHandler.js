import chalk from 'chalk';

export class CLIError extends Error {
  constructor(message, code = 'CLI_ERROR', exitCode = 1) {
    super(message);
    this.name = 'CLIError';
    this.code = code;
    this.exitCode = exitCode;
    Error.captureStackTrace(this, this.constructor);
  }
}

export class ValidationError extends CLIError {
  constructor(message) {
    super(message, 'VALIDATION_ERROR', 2);
    this.name = 'ValidationError';
  }
}

export class ConfigError extends CLIError {
  constructor(message) {
    super(message, 'CONFIG_ERROR', 3);
    this.name = 'ConfigError';
  }
}

export class ProviderError extends CLIError {
  constructor(message, provider) {
    super(message, 'PROVIDER_ERROR', 4);
    this.name = 'ProviderError';
    this.provider = provider;
  }
}

export function handleError(error, themeManager) {
  if (error instanceof CLIError) {
    console.error(themeManager.color('error')(`✗ ${error.message}`));
    return error.exitCode;
  }
  
  if (error instanceof Error && error.name === 'ExitPromptError') {
    console.log(chalk.yellow('\n👋 Operation cancelled.'));
    return 0;
  }
  
  console.error(themeManager.color('error')(`✗ Unexpected error: ${error.message}`));
  if (process.env.DEBUG && error.stack) {
    console.error(chalk.gray(error.stack));
  }
  return 1;
}

export function setupGlobalErrorHandlers() {
  process.on('uncaughtException', (error) => {
    if (error instanceof Error && error.name === 'ExitPromptError') {
      console.log(chalk.yellow('\n👋 Goodbye!'));
      process.exit(0);
    }
    console.error(chalk.red(`\n❌ Uncaught Exception: ${error.message}`));
    if (error.stack) {
      console.error(chalk.gray(error.stack));
    }
    process.exit(1);
  });

  process.on('unhandledRejection', (reason, promise) => {
    console.error(chalk.red(`\n❌ Unhandled Rejection: ${reason}`));
    if (reason instanceof Error && reason.stack) {
      console.error(chalk.gray(reason.stack));
    }
    process.exit(1);
  });
}

