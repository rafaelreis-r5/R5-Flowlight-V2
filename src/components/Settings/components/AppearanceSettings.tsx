import React from 'react';

interface AppearanceSettingsProps {
  settings: {
    theme: 'light' | 'dark' | 'system';
    fontSize: number;
  };
  onChange: (key: string, value: any) => void;
}

export const AppearanceSettings: React.FC<AppearanceSettingsProps> = ({ settings, onChange }) => {
  const themes = [
    { id: 'light', name: 'Claro', icon: '☀️' },
    { id: 'dark', name: 'Escuro', icon: '🌙' },
    { id: 'system', name: 'Sistema', icon: '💻' },
  ];

  return (
    <div className="space-y-8">
      <div>
        <h3 className="text-lg font-semibold text-white mb-4">Tema</h3>
        
        <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
          {themes.map((theme) => (
            <button
              key={theme.id}
              onClick={() => onChange('theme', theme.id)}
              className={`p-4 rounded-lg border-2 transition-all ${
                settings.theme === theme.id
                  ? 'border-[#6405d6] bg-[#1a0a2e]'
                  : 'border-[#2a1a3a] hover:border-[#3a2a4a] bg-[#1a0a2e] hover:bg-[#2a1a3a]'
              }`}
            >
              <div className="text-4xl mb-2">{theme.icon}</div>
              <div className="font-medium text-gray-200">{theme.name}</div>
              {settings.theme === theme.id && (
                <div className="mt-2 text-xs text-[#6405d6]">• Selecionado</div>
              )}
            </button>
          ))}
        </div>
      </div>

      <div>
        <h3 className="text-lg font-semibold text-white mb-4">Tamanho da Fonte</h3>
        
        <div className="space-y-4">
          <div className="flex items-center justify-between">
            <span className="text-sm text-gray-300">Pequeno</span>
            <span className="text-sm text-gray-300">Médio</span>
            <span className="text-sm text-gray-300">Grande</span>
          </div>
          
          <div className="relative">
            <input
              type="range"
              min="12"
              max="20"
              step="1"
              value={settings.fontSize}
              onChange={(e) => onChange('fontSize', parseInt(e.target.value))}
              className="w-full h-2 bg-gray-700 rounded-lg appearance-none cursor-pointer"
            />
            <div className="absolute text-xs text-gray-400 top-4 left-0">A</div>
            <div className="absolute text-xs text-gray-400 top-4 right-0">A</div>
          </div>
          
          <div className="p-4 bg-[#1a0a2e] rounded-lg border border-[#2a1a3a] mt-4">
            <p style={{ fontSize: `${settings.fontSize}px` }} className="text-gray-200">
              Texto de exemplo com o tamanho de fonte selecionado
            </p>
          </div>
        </div>
      </div>

      <div>
        <h3 className="text-lg font-semibold text-white mb-4">Personalização</h3>
        
        <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
          <div>
            <label className="block text-sm font-medium text-gray-200 mb-2">
              Cor de Destaque
            </label>
            <div className="flex space-x-2">
              {['#6405d6', '#f1011d', '#1a73e8', '#0b8043', '#f6bf26'].map((color) => (
                <button
                  key={color}
                  onClick={() => {}}
                  className="w-8 h-8 rounded-full focus:outline-none focus:ring-2 focus:ring-white"
                  style={{ backgroundColor: color }}
                  title={color}
                />
              ))}
              <button className="w-8 h-8 rounded-full border-2 border-dashed border-gray-500 flex items-center justify-center text-gray-400 hover:text-white">
                +
              </button>
            </div>
          </div>
          
          <div>
            <label className="block text-sm font-medium text-gray-200 mb-2">
              Opacidade da Janela
            </label>
            <div className="flex items-center space-x-4">
              <div className="flex-1">
                <input
                  type="range"
                  min="70"
                  max="100"
                  value="100"
                  className="w-full h-2 bg-gray-700 rounded-lg appearance-none cursor-pointer"
                />
              </div>
              <span className="text-sm text-gray-300 w-10 text-right">100%</span>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
};
