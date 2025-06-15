import React from 'react';
import { Settings, DEFAULT_SETTINGS } from '../../../types/settings';
import { Folder, FolderOpen, FolderGit2, FolderHeart, FolderLock, FolderSearch, FolderX } from 'lucide-react';

interface FolderIconsSettingsProps {
  settings: Settings['folderIcons'];
  onChange: <K extends keyof Settings['folderIcons']>(
    key: K,
    value: Settings['folderIcons'][K]
  ) => void;
}

export const FolderIconsSettings: React.FC<FolderIconsSettingsProps> = ({
  settings,
  onChange,
}) => {
  const handleToggle = (key: keyof Settings['folderIcons'], value: any) => {
    onChange(key, value);
  };

  const handleCustomIconChange = (folderType: string, value: string) => {
    const newCustomIcons = { ...settings.customIcons };
    if (value) {
      newCustomIcons[folderType] = value;
    } else {
      delete newCustomIcons[folderType];
    }
    onChange('customIcons', newCustomIcons);
  };

  const folderTypes = [
    { id: 'default', name: 'Pasta Padrão', icon: <Folder className="w-5 h-5 text-blue-400" /> },
    { id: 'open', name: 'Pasta Aberta', icon: <FolderOpen className="w-5 h-5 text-blue-500" /> },
    { id: 'git', name: 'Repositório Git', icon: <FolderGit2 className="w-5 h-5 text-orange-400" /> },
    { id: 'favorite', name: 'Favoritos', icon: <FolderHeart className="w-5 h-5 text-red-400" /> },
    { id: 'private', name: 'Privado', icon: <FolderLock className="w-5 h-5 text-yellow-400" /> },
    { id: 'search', name: 'Pesquisa', icon: <FolderSearch className="w-5 h-5 text-green-400" /> },
    { id: 'archive', name: 'Arquivo', icon: <FolderX className="w-5 h-5 text-gray-400" /> },
  ];

  const iconSizes = [
    { value: 'small', label: 'Pequeno' },
    { value: 'medium', label: 'Médio' },
    { value: 'large', label: 'Grande' },
  ];

  const themes = [
    { value: 'default', label: 'Padrão' },
    { value: 'colorful', label: 'Colorido' },
    { value: 'minimal', label: 'Mínimo' },
    { value: 'custom', label: 'Personalizado' },
  ];

  return (
    <div className="space-y-6">
      <div>
        <h3 className="text-lg font-semibold text-white mb-4">Ícones de Pastas</h3>
        <p className="text-sm text-gray-400 mb-6">
          Personalize a aparência dos ícones de pastas no explorador de arquivos.
        </p>
        
        <div className="space-y-6">
          <div className="flex items-center justify-between">
            <div>
              <h4 className="text-sm font-medium text-gray-200">Ativar Ícones de Pastas</h4>
              <p className="text-xs text-gray-400">Habilite ou desabilite os ícones personalizados para pastas</p>
            </div>
            <label className="relative inline-flex items-center cursor-pointer">
              <input 
                type="checkbox" 
                className="sr-only peer" 
                checked={settings.enabled}
                onChange={(e) => handleToggle('enabled', e.target.checked)}
              />
              <div className="w-11 h-6 bg-gray-700 rounded-full peer peer-checked:after:translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-0.5 after:left-[2px] after:bg-white after:border-gray-300 after:border after:rounded-full after:h-5 after:w-5 after:transition-all peer-checked:bg-[#6405d6]"></div>
            </label>
          </div>

          {settings.enabled && (
            <>
              <div className="space-y-4">
                <div>
                  <label htmlFor="icon-size" className="block text-sm font-medium text-gray-200 mb-1">
                    Tamanho do Ícone
                  </label>
                  <select
                    id="icon-size"
                    className="w-full bg-[#1a0a2e] border border-[#2a1a3a] rounded-md py-2 px-3 text-sm text-white focus:outline-none focus:ring-2 focus:ring-[#6405d6] focus:border-transparent"
                    value={settings.size}
                    onChange={(e) => handleToggle('size', e.target.value as any)}
                  >
                    {iconSizes.map((size) => (
                      <option key={size.value} value={size.value}>
                        {size.label}
                      </option>
                    ))}
                  </select>
                </div>

                <div>
                  <label htmlFor="theme" className="block text-sm font-medium text-gray-200 mb-1">
                    Tema
                  </label>
                  <select
                    id="theme"
                    className="w-full bg-[#1a0a2e] border border-[#2a1a3a] rounded-md py-2 px-3 text-sm text-white focus:outline-none focus:ring-2 focus:ring-[#6405d6] focus:border-transparent"
                    value={settings.theme}
                    onChange={(e) => handleToggle('theme', e.target.value as any)}
                  >
                    {themes.map((theme) => (
                      <option key={theme.value} value={theme.value}>
                        {theme.label}
                      </option>
                    ))}
                  </select>
                </div>

                {settings.theme === 'custom' && (
                  <div className="space-y-4">
                    <h4 className="text-sm font-medium text-gray-200">Ícones Personalizados</h4>
                    <div className="space-y-3">
                      {folderTypes.map((folderType) => (
                        <div key={folderType.id} className="flex items-center">
                          <div className="w-8">
                            {folderType.icon}
                          </div>
                          <span className="text-sm text-gray-300 ml-2 w-32">{folderType.name}</span>
                          <input
                            type="text"
                            className="flex-1 bg-[#1a0a2e] border border-[#2a1a3a] rounded-md py-1 px-2 text-sm text-white focus:outline-none focus:ring-1 focus:ring-[#6405d6] focus:border-transparent"
                            placeholder="URL do ícone"
                            value={settings.customIcons[folderType.id] || ''}
                            onChange={(e) => handleCustomIconChange(folderType.id, e.target.value)}
                          />
                        </div>
                      ))}
                    </div>
                  </div>
                )}

                <div className="space-y-3 pt-2">
                  <div className="flex items-center justify-between">
                    <div>
                      <h4 className="text-sm font-medium text-gray-200">Mostrar na Barra Lateral</h4>
                      <p className="text-xs text-gray-400">Exibir ícones personalizados na barra lateral</p>
                    </div>
                    <label className="relative inline-flex items-center cursor-pointer">
                      <input 
                        type="checkbox" 
                        className="sr-only peer" 
                        checked={settings.showInSidebar}
                        onChange={(e) => handleToggle('showInSidebar', e.target.checked)}
                      />
                      <div className="w-11 h-6 bg-gray-700 rounded-full peer peer-checked:after:translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-0.5 after:left-[2px] after:bg-white after:border-gray-300 after:border after:rounded-full after:h-5 after:w-5 after:transition-all peer-checked:bg-[#6405d6]"></div>
                    </label>
                  </div>

                  <div className="flex items-center justify-between">
                    <div>
                      <h4 className="text-sm font-medium text-gray-200">Mostrar no Explorador de Arquivos</h4>
                      <p className="text-xs text-gray-400">Exibir ícones personalizados no explorador de arquivos</p>
                    </div>
                    <label className="relative inline-flex items-center cursor-pointer">
                      <input 
                        type="checkbox" 
                        className="sr-only peer" 
                        checked={settings.showInFileExplorer}
                        onChange={(e) => handleToggle('showInFileExplorer', e.target.checked)}
                      />
                      <div className="w-11 h-6 bg-gray-700 rounded-full peer peer-checked:after:translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-0.5 after:left-[2px] after:bg-white after:border-gray-300 after:border after:rounded-full after:h-5 after:w-5 after:transition-all peer-checked:bg-[#6405d6]"></div>
                    </label>
                  </div>
                </div>
              </div>

              <div className="pt-4 border-t border-[#2a1a3a]">
                <button
                  onClick={() => {
                    // Lógica para redefinir as configurações
                    Object.entries(DEFAULT_SETTINGS.folderIcons).forEach(([key, value]) => {
                      handleToggle(key as keyof Settings['folderIcons'], value);
                    });
                  }}
                  className="text-sm text-[#6405d6] hover:text-[#7a1ae6]"
                >
                  Redefinir para padrão
                </button>
              </div>
            </>
          )}
        </div>
      </div>
    </div>
  );
};
