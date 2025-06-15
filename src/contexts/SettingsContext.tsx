import React, { createContext, useContext, useState, useEffect, ReactNode } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { Settings, DEFAULT_SETTINGS } from '../types/settings';

interface SettingsContextType {
  settings: Settings;
  updateSetting: <T extends keyof Settings, K extends keyof Settings[T]>(
    section: T,
    key: K,
    value: Settings[T][K]
  ) => Promise<void>;
  resetSettings: () => Promise<void>;
  isLoading: boolean;
}

const SettingsContext = createContext<SettingsContextType | undefined>(undefined);

export const SettingsProvider: React.FC<{ children: ReactNode }> = ({ children }) => {
  const [settings, setSettings] = useState<Settings>(DEFAULT_SETTINGS);
  const [isLoading, setIsLoading] = useState(true);

  // Carregar configurações ao iniciar
  useEffect(() => {
    const loadSettings = async () => {
      try {
        // Tenta carregar as configurações salvas
        const savedSettings = await invoke<Settings>('load_settings_cmd');
        setSettings(savedSettings || DEFAULT_SETTINGS);
      } catch (error) {
        console.error('Erro ao carregar configurações:', error);
        // Usa as configurações padrão em caso de erro
        setSettings(DEFAULT_SETTINGS);
      } finally {
        setIsLoading(false);
      }
    };

    loadSettings();
  }, []);

  // Atualizar configuração
  const updateSetting = async <T extends keyof Settings, K extends keyof Settings[T]>(
    section: T,
    key: K,
    value: Settings[T][K]
  ) => {
    const newSettings = {
      ...settings,
      [section]: {
        ...settings[section],
        [key]: value,
      },
    };

    setSettings(newSettings);

    // Salvar no backend
    try {
      await invoke('save_settings', { settings: newSettings });
    } catch (error) {
      console.error('Erro ao salvar configurações:', error);
      // Reverter em caso de erro
      setSettings(settings);
      throw error;
    }
  };

  // Resetar configurações para os padrões
  const resetSettings = async () => {
    try {
      await invoke('reset_settings');
      setSettings(DEFAULT_SETTINGS);
    } catch (error) {
      console.error('Erro ao redefinir configurações:', error);
      throw error;
    }
  };

  return (
    <SettingsContext.Provider value={{ settings, updateSetting, resetSettings, isLoading }}>
      {children}
    </SettingsContext.Provider>
  );
};

export const useSettings = (): SettingsContextType => {
  const context = useContext(SettingsContext);
  if (context === undefined) {
    throw new Error('useSettings must be used within a SettingsProvider');
  }
  return context;
};
