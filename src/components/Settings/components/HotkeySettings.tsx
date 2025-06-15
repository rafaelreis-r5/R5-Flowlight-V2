import React from 'react';

interface Hotkey {
  id: string;
  label: string;
  default: string;
  custom?: string;
}

interface HotkeySettingsProps {
  settings: Record<string, string>;
  onChange: (key: string, value: string) => void;
}

export const HotkeySettings: React.FC<HotkeySettingsProps> = ({ settings, onChange }) => {
  const hotkeys: Hotkey[] = [
    { id: 'openApp', label: 'Abrir Flowlight', default: '⌘ + Space' },
    { id: 'search', label: 'Buscar', default: '⌘ + K' },
    { id: 'newNote', label: 'Nova Nota', default: '⌘ + N' },
    { id: 'settings', label: 'Configurações', default: '⌘ + ,' },
  ].map(hotkey => ({
    ...hotkey,
    custom: settings[hotkey.id] || ''
  }));

  return (
    <div className="space-y-6">
      <div>
        <h3 className="text-lg font-semibold text-white mb-4">Atalhos Globais</h3>
        <p className="text-sm text-gray-400 mb-4">
          Personalize os atalhos de teclado para acessar rapidamente as funcionalidades do Flowlight.
        </p>
        
        <div className="bg-[#1a0a2e] rounded-lg border border-[#2a1a3a] overflow-hidden">
          <table className="min-w-full divide-y divide-[#2a1a3a]">
            <thead>
              <tr className="bg-[#2a1a3a]">
                <th className="px-6 py-3 text-left text-xs font-medium text-gray-300 uppercase tracking-wider">
                  Ação
                </th>
                <th className="px-6 py-3 text-left text-xs font-medium text-gray-300 uppercase tracking-wider">
                  Atalho
                </th>
                <th className="px-6 py-3 text-left text-xs font-medium text-gray-300 uppercase tracking-wider">
                  Padrão
                </th>
              </tr>
            </thead>
            <tbody className="divide-y divide-[#2a1a3a]">
              {hotkeys.map((hotkey) => (
                <tr key={hotkey.id} className="hover:bg-[#2a1a3a]/50">
                  <td className="px-6 py-4 whitespace-nowrap text-sm font-medium text-gray-200">
                    {hotkey.label}
                  </td>
                  <td className="px-6 py-4 whitespace-nowrap">
                    <button 
                      onClick={() => {
                        // Lógica para capturar atalho do teclado
                        const newShortcut = prompt(`Digite o novo atalho para ${hotkey.label}`, hotkey.custom || hotkey.default);
                        if (newShortcut !== null) {
                          onChange(hotkey.id, newShortcut);
                        }
                      }}
                      className="px-3 py-1 bg-[#1a0a2e] border border-[#3a2a4a] rounded-md text-sm text-gray-300 hover:bg-[#2a1a3a] transition-colors"
                    >
                      {hotkey.custom || hotkey.default}
                    </button>
                  </td>
                  <td className="px-6 py-4 whitespace-nowrap text-sm text-gray-400">
                    {hotkey.default}
                  </td>
                </tr>
              ))}
            </tbody>
          </table>
        </div>
      </div>

      <div className="pt-4">
        <div className="flex items-center justify-between">
          <div>
            <h4 className="text-sm font-medium text-gray-200">Detecção de Conflitos</h4>
            <p className="text-xs text-gray-400">Avisar sobre atalhos em conflito com outros aplicativos</p>
          </div>
          <label className="relative inline-flex items-center cursor-pointer">
            <input type="checkbox" className="sr-only peer" defaultChecked />
            <div className="w-11 h-6 bg-gray-700 rounded-full peer peer-checked:after:translate-x-full after:content-[''] after:absolute after:top-0.5 after:left-[2px] after:bg-white after:border-gray-300 after:border after:rounded-full after:h-5 after:w-5 after:transition-all peer-checked:bg-gradient-to-r peer-checked:from-[#6405d6] peer-checked:to-[#f1011d]"></div>
          </label>
        </div>
      </div>

      <div className="bg-[#1a0a2e] border-l-4 border-yellow-500 p-4 rounded-md">
        <div className="flex">
          <div className="flex-shrink-0">
            <svg className="h-5 w-5 text-yellow-400" viewBox="0 0 20 20" fill="currentColor">
              <path fillRule="evenodd" d="M8.257 3.099c.765-1.36 2.722-1.36 3.486 0l5.58 9.92c.75 1.334-.213 2.98-1.742 2.98H4.42c-1.53 0-2.493-1.646-1.743-2.98l5.58-9.92zM11 13a1 1 0 11-2 0 1 1 0 012 0zm-1-8a1 1 0 00-1 1v3a1 1 0 002 0V6a1 1 0 00-1-1z" clipRule="evenodd" />
            </svg>
          </div>
          <div className="ml-3">
            <p className="text-sm text-yellow-200">
              O atalho <span className="font-mono bg-yellow-900/50 px-1.5 py-0.5 rounded">⌘ + Space</span> está em conflito com a pesquisa do sistema.
            </p>
          </div>
        </div>
      </div>
    </div>
  );
};
