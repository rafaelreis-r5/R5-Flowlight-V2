import React, { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { toast } from 'react-hot-toast';
import { Settings } from '../../types/settings';
import { PerformanceSettings } from './components/PerformanceSettings';
import { AppearanceSettings } from './components/AppearanceSettings';
import { GeneralSettings } from './components/GeneralSettings';
import { NotificationSettings } from './components/NotificationSettings';
import { FolderIconsSettings } from './components/FolderIconsSettings';
import { SearchSettings, SearchSettingsType } from './components/SearchSettings';
import { ContentType } from '../../types/content';
import { AISettings } from './components/AISettings';

interface SettingsWindowProps {
  isOpen: boolean;
  onClose: () => void;
}



export const SettingsWindow: React.FC<SettingsWindowProps> = ({ isOpen, onClose }) => {
  const [settings, setSettings] = useState<Settings | null>(null);
  const [activeTab, setActiveTab] = useState('general');
  const [hasChanges, setHasChanges] = useState(false);

  useEffect(() => {
    if (isOpen) {
      console.log('Abrindo janela de configurações, carregando configurações...');
      loadSettings().then(settings => {
        console.log('Configurações carregadas com sucesso:', settings);
      }).catch(error => {
        console.error('Falha ao carregar configurações:', error);
      });
    }
  }, [isOpen]);

  const loadSettings = async (resetToDefaults = false) => {
    console.log(`Carregando configurações... resetToDefaults: ${resetToDefaults}`);
    try {
      let loadedSettings;
      if (resetToDefaults) {
        console.log('Reiniciando para configurações padrão...');
        await invoke('reset_settings_cmd');
        loadedSettings = await invoke<Settings>('load_settings_cmd');
        console.log('Configurações padrão carregadas:', loadedSettings);
      } else {
        console.log('Obtendo configurações salvas...');
        loadedSettings = await invoke<Settings>('load_settings_cmd');
        console.log('Configurações carregadas com sucesso:', loadedSettings);
      }
      setSettings(loadedSettings);
      setHasChanges(false);
      console.log('Configurações definidas no estado');
      return loadedSettings;
    } catch (error) {
      console.error('Erro ao carregar configurações:', error);
      toast.error('Erro ao carregar configurações');
      throw error;
    }
  };

  const saveSettings = async () => {
    console.log('Salvando configurações...', settings);
    if (!settings) {
      console.error('Nenhuma configuração para salvar');
      return false;
    }

    try {
      console.log('Chamando save_settings_cmd...');
      await invoke('save_settings_cmd', { settings });
      console.log('Configurações salvas com sucesso no backend');
      setHasChanges(false);
      toast.success('Configurações salvas com sucesso!');
      return true;
    } catch (error) {
      console.error('Erro ao salvar configurações:', error);
      toast.error('Erro ao salvar configurações');
      return false;
    }
  };

  // Handle close with confirmation if there are unsaved changes
  const handleClose = async () => {
    if (!hasChanges) {
      onClose();
      return;
    }

    // Se houver alterações, perguntar ao usuário
    if (confirm('Existem alterações não salvas. Deseja salvar antes de sair?')) {
      const saved = await saveSettings();
      if (saved) {
        onClose();
      }
    } else {
      // Descartar alterações e recarregar as configurações originais
      await loadSettings();
      onClose();
    }
  };

  const resetSettings = async () => {
    try {
      console.log('Reiniciando configurações para os padrões...');
      await invoke('reset_settings_cmd');
      await loadSettings();
      setHasChanges(true);
      toast.success('Configurações restauradas para os padrões (não salvas)');
    } catch (error) {
      console.error('Erro ao restaurar configurações:', error);
      toast.error('Erro ao restaurar configurações');
    }
  };

  type SectionKey<T extends keyof Settings> = keyof Settings[T];
  type SectionValue<T extends keyof Settings, K extends SectionKey<T>> = Settings[T][K];

  const updateSetting = async <T extends keyof Settings, K extends SectionKey<T>>(
    section: T,
    key: K,
    value: SectionValue<T, K>
  ) => {
    if (!settings) return;

    // Atualiza o estado local
    setSettings(prev => ({
      ...prev!,
      [section]: {
        ...prev![section],
        [key]: value
      }
    }) as Settings);

    setHasChanges(true);

    // Se for a configuração de inicialização com o sistema, atualiza imediatamente
    if (section === 'general' && key === 'startWithSystem') {
      try {
        await invoke('update_autostart', { startWithSystem: value });
      } catch (error) {
        console.error('Erro ao atualizar inicialização automática:', error);
        toast.error('Erro ao atualizar inicialização automática');

        // Reverte a mudança em caso de erro
        setSettings(prev => ({
          ...prev!,
          general: {
            ...prev!.general,
            startWithSystem: !(value as boolean)
          }
        }) as Settings);
      }
    }
  };

  void updateSetting;

  if (!isOpen || !settings) return null;

  const renderActiveTab = () => {
    if (!settings) return null;

    switch (activeTab) {
      case 'general':
        return (
          <GeneralSettings
            settings={settings.general}
            onChange={(key, value) => {
              setSettings(prev => ({
                ...prev!,
                general: {
                  ...prev!.general,
                  [key]: value
                }
              }));
              setHasChanges(true);
            }}
          />
        );
      case 'search':
        return (
          <SearchSettings
            settings={{
              ...settings.search,
              advancedIndexing: {
                enabled: false,
                maxFileSize: 10,
                indexFileTypes: [],
                excludeHiddenFiles: true,
                indexFileMetadata: true,
                indexFileContent: true,
                contentTypesToIndex: {
                  documents: true,
                  images: false,
                  audio: false,
                  video: false,
                  archives: false,
                  code: true,
                  spreadsheets: true,
                  presentations: true,
                  pdfs: true,
                  emails: true
                },
                indexSchedule: 'realtime',
                lastIndexed: null
              },
              contentTypes: (settings.search.contentTypes || []) as ContentType[]
            }}
            onChange={(key: keyof SearchSettingsType, value: any) => {
              if (key === 'advancedIndexing') {
                setSettings(prev => ({
                  ...prev!,
                  search: {
                    ...prev!.search,
                    advancedIndexing: value
                  }
                }));
              } else {
                setSettings(prev => ({
                  ...prev!,
                  search: {
                    ...prev!.search,
                    [key]: value
                  }
                }));
              }
              setHasChanges(true);
            }}
          />
        );
      case 'appearance':
        return (
          <AppearanceSettings
            settings={settings.appearance}
            onChange={(key, value) => {
              setSettings(prev => ({
                ...prev!,
                appearance: {
                  ...prev!.appearance,
                  [key]: value
                }
              }));
              setHasChanges(true);
            }}
          />
        );
      case 'hotkeys':
        return <div>Hotkey Settings</div>;
      case 'ai':
        return (
          <AISettings
            settings={settings.ai}
            onChange={(key, value) => {
              void updateSetting('ai', key, value);
            }}
          />
        );
      case 'privacy':
        return <div>Privacy Settings</div>;
      case 'performance':
        return <PerformanceSettings
          settings={settings.performance}
          onChange={(
            key: keyof typeof settings.performance,
            value: (typeof settings.performance)[keyof typeof settings.performance]
          ) => {
            setSettings(prev => ({
              ...prev!,
              performance: {
                ...prev!.performance,
                [key]: value
              }
            }));
            setHasChanges(true);
          }}
        />;

      case 'folder-icons':
        return (
          <FolderIconsSettings
            settings={settings.folderIcons}
            onChange={(key, value) => {
              setSettings(prev => ({
                ...prev!,
                folderIcons: {
                  ...prev!.folderIcons,
                  [key]: value
                }
              }));
              setHasChanges(true);
            }}
          />
        );

      case 'notifications':
        return <NotificationSettings
          settings={settings.notifications}
          onChange={(key: string, value: any) => {
            if (key.startsWith('notifications.')) {
              const subKey = key.replace('notifications.', '');
              setSettings(prev => ({
                ...prev!,
                notifications: {
                  ...prev!.notifications,
                  [subKey]: value
                }
              }));
            } else if (key === 'notifications.types') {
              setSettings(prev => ({
                ...prev!,
                notifications: {
                  ...prev!.notifications,
                  types: value
                }
              }));
            }
            setHasChanges(true);
          }}
        />;
      case 'integrations':
        return <div>Integration Settings</div>;
      case 'about':
        return <div>About</div>;
      default:
        return <div>Configuração não encontrada</div>;
    }
  };

  return (
    <div className="fixed inset-0 bg-black/50 flex items-center justify-center z-50 p-4">
      <div className="bg-[#0f0a1a] border border-[#2a1a3a] rounded-lg w-full max-w-5xl h-[80vh] flex flex-col overflow-hidden">
        {/* Header */}
        <div className="flex items-center justify-between p-4 border-b border-[#2a1a3a] bg-[#1a0a2e]">
          <button
            onClick={handleClose}
            className="text-[#f5ecee] hover:text-white transition-colors"
          >
            ← Voltar
          </button>
          <h2 className="text-xl font-bold text-white">R5 Flowlight - Configurações</h2>
          <div className="w-8"></div> {/* Para alinhamento */}
        </div>

        <div className="flex flex-1 overflow-hidden">
          {/* Sidebar */}
          <div className="w-56 bg-[#1a0a2e] p-4 border-r border-[#2a1a3a] overflow-y-auto">
            <nav className="space-y-1">
              {[
                { id: 'general', label: 'Geral', icon: '⚙️' },
                { id: 'notifications', label: 'Notificações', icon: '🔔' },
                { id: 'appearance', label: 'Aparência', icon: '🎨' },
                { id: 'folder-icons', label: 'Ícones de Pastas', icon: '📁' },
                { id: 'search', label: 'Busca', icon: '🔍' },
                { id: 'hotkeys', label: 'Atalhos', icon: '⌨️' },
                { id: 'ai', label: 'IA', icon: '🤖' },
                { id: 'privacy', label: 'Privacidade', icon: '🔒' },
                { id: 'performance', label: 'Performance', icon: '⚡' },
                { id: 'integrations', label: 'Integrações', icon: '🔗' },
                { id: 'about', label: 'Sobre', icon: 'ℹ️' },
              ].map((tab) => (
                <button
                  key={tab.id}
                  onClick={() => setActiveTab(tab.id)}
                  className={`w-full text-left px-4 py-2 rounded-md transition-colors ${
                    activeTab === tab.id
                      ? 'bg-gradient-to-r from-[#6405d6] to-[#f1011d] text-white'
                      : 'text-[#f5ecee] hover:bg-[#2a1a3a]'
                  }`}
                >
                  <span className="mr-2">{tab.icon}</span>
                  {tab.label}
                </button>
              ))}
            </nav>
          </div>

          {/* Main Content */}
          <div className="flex-1 flex flex-col overflow-hidden">
            <div className="flex-1 p-6 overflow-y-auto">
              {renderActiveTab()}
            </div>

            {/* Footer */}
            <div className="p-4 border-t border-[#2a1a3a] bg-[#1a0a2e] flex justify-between">
              <button
                onClick={handleClose}
                className="px-4 py-2 text-[#f5ecee] hover:bg-[#2a1a3a] rounded-md transition-colors"
              >
                {hasChanges ? 'Descartar Alterações' : 'Fechar'}
              </button>

              <div className="space-x-2">
                <button
                  onClick={resetSettings}
                  className="px-4 py-2 text-[#f5ecee] hover:bg-[#2a1a3a] rounded-md transition-colors"
                >
                  Restaurar Padrões
                </button>

                <button
                  onClick={saveSettings}
                  disabled={!hasChanges}
                  className={`px-4 py-2 rounded-md transition-opacity ${
                    hasChanges
                      ? 'bg-gradient-to-r from-[#6405d6] to-[#f1011d] text-white hover:opacity-90'
                      : 'bg-gray-600 text-gray-400 cursor-not-allowed'
                  }`}
                >
                  Salvar Alterações
                </button>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
};
