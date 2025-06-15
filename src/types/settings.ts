export interface Settings {
  general: {
    startWithSystem: boolean;
    showInTaskbar: boolean;
    closeToTray: boolean;
    language: string;
    dateFormat: string;
    numberFormat: string;
  };
  search: {
    maxResults: number;
    searchDelay: number;
    fuzzySearch: boolean;
    includedPaths: string[];
    excludedPatterns: string[];
    contentTypes: string[];
  };
  appearance: {
    theme: 'light' | 'dark' | 'system';
    fontSize: number;
  };
  hotkeys: {
    openApp: string;
    search: string;
    newNote: string;
    settings: string;
  };
  ai: {
    apiKey: string;
    model: string;
    temperature: number;
    contextMemory: boolean;
  };
  privacy: {
    telemetry: boolean;
    crashReports: boolean;
  };
  performance: {
    hardwareAcceleration: boolean;
    backgroundThrottling: boolean;
    maxConcurrent: number;
    cacheSize: number;
  };
  integrations: {
    github: boolean;
    slack: boolean;
    google: boolean;
    notion: boolean;
    figma: boolean;
    webhooks: string[];
  };
  notifications: {
    enabled: boolean;
    sounds: boolean;
    position: 'top-right' | 'top-center' | 'top-left' | 'bottom-right' | 'bottom-center' | 'bottom-left';
    duration: number; // in seconds
    showProgress: boolean;
    useNative: boolean; // Usar notificações nativas do sistema operacional
    showWhenFocused: boolean; // Mostrar notificações mesmo quando a janela estiver em foco
    types: {
      info: boolean;
      success: boolean;
      warning: boolean;
      error: boolean;
      update: boolean;
      reminder: boolean;
    };
  };
  folderIcons: {
    enabled: boolean;
    size: 'small' | 'medium' | 'large';
    theme: 'default' | 'colorful' | 'minimal' | 'custom';
    customIcons: {
      [key: string]: string; // Mapeia tipos de pasta para URLs de ícones
    };
    showInSidebar: boolean;
    showInFileExplorer: boolean;
  };
}

export const DEFAULT_SETTINGS: Settings = {
  general: {
    startWithSystem: true,
    showInTaskbar: true,
    closeToTray: true,
    language: 'pt-BR',
    dateFormat: 'dd/MM/yyyy',
    numberFormat: 'pt-BR',
  },
  search: {
    maxResults: 50,
    searchDelay: 300,
    fuzzySearch: true,
    includedPaths: [],
    excludedPatterns: [],
    contentTypes: ['apps', 'files', 'system'],
  },
  appearance: {
    theme: 'system',
    fontSize: 14,
  },
  hotkeys: {
    openApp: 'CommandOrControl+Space',
    search: 'CommandOrControl+K',
    newNote: 'CommandOrControl+N',
    settings: 'CommandOrControl+,',
  },
  ai: {
    apiKey: '',
    model: 'gpt-3.5-turbo',
    temperature: 0.7,
    contextMemory: true,
  },
  privacy: {
    telemetry: true,
    crashReports: true,
  },
  performance: {
    hardwareAcceleration: true,
    backgroundThrottling: true,
    maxConcurrent: 4,
    cacheSize: 500, // MB
  },
  integrations: {
    github: false,
    slack: false,
    google: false,
    notion: false,
    figma: false,
    webhooks: [],
  },
  notifications: {
    enabled: true,
    sounds: true,
    position: 'bottom-right' as const,
    duration: 5,
    showProgress: true,
    useNative: true,
    showWhenFocused: false,
    types: {
      info: true,
      success: true,
      warning: true,
      error: true,
      update: true,
      reminder: true,
    },
  },
  folderIcons: {
    enabled: true,
    size: 'medium',
    theme: 'default',
    customIcons: {},
    showInSidebar: true,
    showInFileExplorer: true,
  },
};
