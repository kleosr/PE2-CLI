export const DEFAULT_CONFIG = {
  model: 'openai/gpt-4o-mini',
  provider: 'openrouter',
  apiKey: null
};

export const LLM_CONFIG = {
  maxTokens: 1024,
  temperature: 0.3,
  systemMessage: 'You are a precise prompt optimizer. Follow the instructions and return JSON only.',
  refinementSystemMessage: 'You are a precise prompt optimizer. Return JSON only.'
};

export const PROMPT_LIMITS = {
  minLength: 10,
  maxLength: 10000,
  cacheKeyPrefixLength: 100,
  cacheKeyMaxLength: 50,
  maxCacheSize: 50,
  previewMaxLength: 200,
  processingDisplayMaxLength: 100,
  apiKeyDisplayLength: 8,
  apiKeyMaskLength: 8,
  shortApiKeyThreshold: 12,
  shortApiKeyPrefix: 4,
  shortApiKeySuffix: 4
};

export const COMPLEXITY_THRESHOLDS = {
  wordCount: {
    veryHigh: 400,
    high: 250,
    medium: 120,
    low: 60
  },
  wordCountScores: {
    veryHigh: 4,
    high: 3,
    medium: 2,
    low: 1
  },
  maxTechIndicators: 4,
  maxDomainIndicators: 3,
  maxStructuralMatches: 4,
  maxLogicMatches: 3,
  specialCharsHigh: 5,
  specialCharsMedium: 2,
  specialCharsHighScore: 2,
  specialCharsMediumScore: 1
};

export const DIFFICULTY_LEVELS = [
  { max: 4, level: 'NOVICE', iterations: 1 },
  { max: 8, level: 'INTERMEDIATE', iterations: 2 },
  { max: 12, level: 'ADVANCED', iterations: 3 },
  { max: 16, level: 'EXPERT', iterations: 4 },
  { max: Infinity, level: 'MASTER', iterations: 5 }
];

export const PROGRESS_BAR_CONFIG = {
  format: ' {bar} | {percentage}% | {task}',
  barCompleteChar: '█',
  barIncompleteChar: '▒',
  hideCursor: true,
  clearOnComplete: false,
  stopOnComplete: false,
  minBarSize: 20,
  maxBarSize: 40,
  columnOffset: 40,
  fps: 8,
  notTTYSchedule: 2000
};

export const PROGRESS_PERCENTAGES = {
  initialization: 0,
  initialPromptStart: 30,
  initialPromptComplete: 50,
  finalization: 90,
  complete: 100
};

export const UI_CONFIG = {
  terminalWidth: {
    default: 80,
    max: 100,
    narrowThreshold: 60,
    compactThreshold: 70
  },
  statusBar: {
    borderPadding: 4,
    modelTruncateLength: 20,
    modelTruncateSuffix: '...',
    modelDisplayLength: 17
  },
  separator: {
    defaultLength: 50,
    maxLength: 50
  }
};

export const PERFORMANCE_METRICS = {
  accuracyGainBase: 20,
  accuracyGainMultiplier: 3,
  defaultQualityScore: 8.0,
  complexityScoreMax: 20
};

export const SESSION_CONFIG = {
  maxHistoryItems: 50,
  maxFavoriteModels: 5,
  maxLastUsedCommands: 10,
  commandSuggestionsLimit: 5,
  priorityCommandsLimit: 5
};

export const FILE_PERMISSIONS = {
  configFile: 0o600
};

export const HTTP_HEADERS = {
  referer: 'https://pe2-cli-tool.local',
  title: 'KleoSr PE2-CLI Tool'
};

export const REQUIRED_PROMPT_FIELDS = ['context', 'role', 'task', 'constraints', 'output'];

export const DEFAULT_CONTEXT = {
  domain: 'general',
  history: [],
  avgComplexity: 0
};

export const DEFAULT_STRATEGY = {
  iterations: 2,
  focus: 'optimization',
  adaptiveFeatures: [],
  template: ''
};

export const DEFAULT_EVALUATION = {
  scores: {},
  overallScore: 8.0
};
