import React from 'react';
import { FormatExample } from './FormatExample';

interface GeneralSettingsProps {
  settings: {
    startWithSystem: boolean;
    showInTaskbar: boolean;
    closeToTray: boolean;
    language: string;
    dateFormat: string;
    numberFormat: string;
  };
  onChange: (key: string, value: any) => void;
}

export const GeneralSettings: React.FC<GeneralSettingsProps> = ({ settings, onChange }) => {
  return (
    <div className="space-y-6">
      <div>
        <h3 className="text-lg font-semibold text-white mb-4">Comportamento de Inicialização</h3>
        
        <div className="space-y-4">
          <div className="flex items-center justify-between">
            <div>
              <p className="text-sm font-medium text-gray-200">Iniciar com o sistema</p>
              <p className="text-xs text-gray-400">Inicia automaticamente quando o computador liga</p>
            </div>
            <label className="relative inline-flex items-center cursor-pointer">
              <input 
                type="checkbox" 
                className="sr-only peer" 
                checked={settings.startWithSystem}
                onChange={(e) => onChange('startWithSystem', e.target.checked)}
              />
              <div className="w-11 h-6 bg-gray-700 rounded-full peer peer-checked:after:translate-x-full after:content-[''] after:absolute after:top-0.5 after:left-[2px] after:bg-white after:border-gray-300 after:border after:rounded-full after:h-5 after:w-5 after:transition-all peer-checked:bg-gradient-to-r peer-checked:from-[#6405d6] peer-checked:to-[#f1011d]"></div>
            </label>
          </div>

          <div className="space-y-4">
            <div className="flex items-center justify-between">
              <div>
                <p className="text-sm font-medium text-gray-200">Mostrar na barra de tarefas</p>
                <p className="text-xs text-gray-400">Exibe ícone na barra de tarefas/dock</p>
              </div>
              <label className="relative inline-flex items-center cursor-pointer">
                <input 
                  type="checkbox" 
                  className="sr-only peer" 
                  checked={settings.showInTaskbar}
                  onChange={(e) => onChange('showInTaskbar', e.target.checked)}
                />
                <div className="w-11 h-6 bg-gray-700 rounded-full peer peer-checked:after:translate-x-full after:content-[''] after:absolute after:top-0.5 after:left-[2px] after:bg-white after:border-gray-300 after:border after:rounded-full after:h-5 after:w-5 after:transition-all peer-checked:bg-gradient-to-r peer-checked:from-[#6405d6] peer-checked:to-[#f1011d]"></div>
              </label>
            </div>

            <div className="flex items-center justify-between">
              <div>
                <p className="text-sm font-medium text-gray-200">Fechar para bandeja</p>
                <p className="text-xs text-gray-400">Fecha o aplicativo para a bandeja em vez de encerrar</p>
              </div>
              <label className="relative inline-flex items-center cursor-pointer">
                <input 
                  type="checkbox" 
                  className="sr-only peer" 
                  checked={settings.closeToTray}
                  onChange={(e) => onChange('closeToTray', e.target.checked)}
                />
                <div className="w-11 h-6 bg-gray-700 rounded-full peer peer-checked:after:translate-x-full after:content-[''] after:absolute after:top-0.5 after:left-[2px] after:bg-white after:border-gray-300 after:border after:rounded-full after:h-5 after:w-5 after:transition-all peer-checked:bg-gradient-to-r peer-checked:from-[#6405d6] peer-checked:to-[#f1011d]"></div>
              </label>
            </div>
          </div>
        </div>
      </div>

      <div>
        <h3 className="text-lg font-semibold text-white mb-4">Idioma e Região</h3>
        
        <div className="space-y-4">
          <div className="space-y-4">
            <div>
              <label htmlFor="language" className="block text-sm font-medium text-gray-200 mb-1">
                Idioma da interface
              </label>
              <select
                id="language"
                className="w-full bg-[#1a0a2e] border border-[#2a1a3a] rounded-md p-2 text-white focus:ring-2 focus:ring-[#6405d6] focus:border-transparent"
                value={settings.language}
                onChange={(e) => onChange('language', e.target.value)}
              >
                <option value="pt-BR">Português (Brasil)</option>
                <option value="en-US">English (US)</option>
                <option value="es">Español</option>
              </select>
            </div>

            <div>
              <label htmlFor="dateFormat" className="block text-sm font-medium text-gray-200 mb-1">
                Formato de data
              </label>
              <select
                id="dateFormat"
                className="w-full bg-[#1a0a2e] border border-[#2a1a3a] rounded-md p-2 text-white focus:ring-2 focus:ring-[#6405d6] focus:border-transparent"
                value={settings.dateFormat}
                onChange={(e) => onChange('dateFormat', e.target.value)}
              >
                <option value="dd/MM/yyyy">DD/MM/AAAA (31/12/2023)</option>
                <option value="MM/dd/yyyy">MM/DD/AAAA (12/31/2023)</option>
                <option value="yyyy-MM-dd">AAAA-MM-DD (2023-12-31)</option>
                <option value="dd MMM yyyy">DD MMM AAAA (31 Dez 2023)</option>
                <option value="MMMM d, yyyy">MMMM D, AAAA (Dezembro 31, 2023)</option>
              </select>
              <FormatExample type="date" />
            </div>

            <div>
              <label htmlFor="numberFormat" className="block text-sm font-medium text-gray-200 mb-1">
                Formato de números
              </label>
              <select
                id="numberFormat"
                className="w-full bg-[#1a0a2e] border border-[#2a1a3a] rounded-md p-2 text-white focus:ring-2 focus:ring-[#6405d6] focus:border-transparent"
                value={settings.numberFormat}
                onChange={(e) => onChange('numberFormat', e.target.value)}
              >
                <option value="pt-BR">Português (1.234,56)</option>
                <option value="en-US">Inglês (1,234.56)</option>
                <option value="es-ES">Espanhol (1.234,56)</option>
                <option value="fr-FR">Francês (1 234,56)</option>
                <option value="de-DE">Alemão (1.234,56)</option>
              </select>
              <FormatExample type="number" />
            </div>
          </div>
        </div>
      </div>
    </div>
  );
};
