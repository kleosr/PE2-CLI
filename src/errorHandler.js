const ERROR_CODES = {
  CLI_ERROR: { exitCode: 1 },
  VALIDATION_ERROR: { exitCode: 2 },
  CONFIG_ERROR: { exitCode: 3 },
  PROVIDER_ERROR: { exitCode: 4 },
  NETWORK_ERROR: { exitCode: 5 },
  AUTH_ERROR: { exitCode: 6 },
  RUNTIME_ERROR: { exitCode: 7 }
};

export class CLIError extends Error {
  constructor(message, code = 'CLI_ERROR', exitCode = ERROR_CODES.CLI_ERROR.exitCode, details = {}) {
    super(message);
    this.name = 'CLIError';
    this.code = code;
    this.exitCode = exitCode;
    this.details = details;
    if (Error.captureStackTrace) {
      Error.captureStackTrace(this, this.constructor);
    }
  }
}

export class ValidationError extends CLIError {
  constructor(message, details = {}) {
    super(message, 'VALIDATION_ERROR', ERROR_CODES.VALIDATION_ERROR.exitCode, details);
    this.name = 'ValidationError';
  }
}

export class ConfigError extends CLIError {
  constructor(message, configPath) {
    super(message, 'CONFIG_ERROR', ERROR_CODES.CONFIG_ERROR.exitCode, { configPath });
    this.name = 'ConfigError';
  }
}

export class ProviderError extends CLIError {
  constructor(message, provider, details = {}) {
    super(`[${provider}] ${message}`, 'PROVIDER_ERROR', ERROR_CODES.PROVIDER_ERROR.exitCode, { provider, ...details });
    this.name = 'ProviderError';
    this.provider = provider;
  }
}

export function handleError(error, themeManager, { isDebug = process.env.DEBUG } = {}) {
  const formatError = themeManager.color('error');
  const formatWarning = themeManager.color('warning');
  const formatMuted = themeManager.color('muted');

  if (error instanceof CLIError) {
    console.error(formatError(`✗ [${error.constructor.name}] ${error.message}`));
    if (error.details && Object.keys(error.details).length > 0) {
      console.error(formatMuted(JSON.stringify(error.details, null, 2)));
    }
    return error.exitCode;
  }

  if (error.name === 'ExitPromptError') {
    console.log(formatWarning('\n👋 Operation cancelled.'));
    return 0;
  }

  const errorMessage = error.message || 'An unknown error occurred';
  console.error(formatError(`✗ Unexpected error: ${errorMessage}`));

  if (isDebug && error.stack) {
    console.error(formatMuted(error.stack));
  } else if (!isDebug) {
    console.error(formatMuted('Run with DEBUG=1 for more details'));
  }

  return ERROR_CODES.RUNTIME_ERROR.exitCode;
}

export function setupGlobalErrorHandlers(themeManager) {
  process.on('uncaughtException', (error) => {
    const exitCode = handleError(error, themeManager, { isDebug: true });
    process.exit(exitCode);
  });

  process.on('unhandledRejection', (reason) => {
    const exitCode = handleError(reason, themeManager, { isDebug: true });
    process.exit(exitCode);
  });
}

export { ERROR_CODES };
