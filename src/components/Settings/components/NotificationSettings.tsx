import React from 'react';
import { Settings } from '../../../types/settings';

interface NotificationSettingsProps {
  settings: Settings['notifications'];
  onChange: (key: string, value: any) => void;
}

export const NotificationSettings: React.FC<NotificationSettingsProps> = ({
  settings,
  onChange,
}) => {
  const handleToggle = (key: keyof typeof settings, value: any) => {
    onChange(`notifications.${key}`, value);
  };

  const handleTypeToggle = (type: keyof typeof settings.types, value: boolean) => {
    const updatedTypes = { ...settings.types, [type]: value };
    onChange('notifications.types', updatedTypes);
  };

  return (
    <div className="space-y-6">
      <h3 className="text-lg font-semibold text-white mb-4">Configurações de Notificação</h3>
      
      <div className="space-y-4">
        <div className="flex items-center justify-between">
          <div>
            <h4 className="text-sm font-medium text-gray-200">Ativar notificações</h4>
            <p className="text-xs text-gray-400">Habilita ou desabilita todas as notificações</p>
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
            <div className="flex items-center justify-between">
              <div>
                <h4 className="text-sm font-medium text-gray-200">Sons</h4>
                <p className="text-xs text-gray-400">Reproduz sons quando uma notificação for exibida</p>
              </div>
              <label className="relative inline-flex items-center cursor-pointer">
                <input
                  type="checkbox"
                  className="sr-only peer"
                  checked={settings.sounds}
                  onChange={(e) => handleToggle('sounds', e.target.checked)}
                />
                <div className="w-11 h-6 bg-gray-700 rounded-full peer peer-checked:after:translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-0.5 after:left-[2px] after:bg-white after:border-gray-300 after:border after:rounded-full after:h-5 after:w-5 after:transition-all peer-checked:bg-[#6405d6]"></div>
              </label>
            </div>

            <div className="grid grid-cols-1 gap-4 md:grid-cols-2">
              <div>
                <label htmlFor="notificationPosition" className="block text-sm font-medium text-gray-200 mb-1">
                  Posição das notificações
                </label>
                <select
                  id="notificationPosition"
                  className="w-full bg-[#1a0a2e] border border-[#2a1a3a] rounded-md p-2 text-white focus:ring-2 focus:ring-[#6405d6] focus:border-transparent"
                  value={settings.position}
                  onChange={(e) => handleToggle('position', e.target.value)}
                >
                  <option value="top-right">Canto superior direito</option>
                  <option value="top-center">Superior centralizado</option>
                  <option value="top-left">Canto superior esquerdo</option>
                  <option value="bottom-right">Canto inferior direito</option>
                  <option value="bottom-center">Inferior centralizado</option>
                  <option value="bottom-left">Canto inferior esquerdo</option>
                </select>
              </div>
              <div>
                <label htmlFor="notificationDuration" className="block text-sm font-medium text-gray-200 mb-1">
                  Duração (segundos)
                </label>
                <input
                  type="number"
                  id="notificationDuration"
                  min="1"
                  max="30"
                  className="w-full bg-[#1a0a2e] border border-[#2a1a3a] rounded-md p-2 text-white focus:ring-2 focus:ring-[#6405d6] focus:border-transparent"
                  value={settings.duration}
                  onChange={(e) => handleToggle('duration', parseInt(e.target.value, 10))}
                />
              </div>
            </div>

            <div className="space-y-4">
              <div className="flex items-center justify-between">
                <div>
                  <h4 className="text-sm font-medium text-gray-200">Mostrar barra de progresso</h4>
                  <p className="text-xs text-gray-400">Exibe uma barra que diminui conforme o tempo da notificação acaba</p>
                </div>
                <label className="relative inline-flex items-center cursor-pointer">
                  <input
                    type="checkbox"
                    className="sr-only peer"
                    checked={settings.showProgress}
                    onChange={(e) => handleToggle('showProgress', e.target.checked)}
                  />
                  <div className="w-11 h-6 bg-gray-700 rounded-full peer peer-checked:after:translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-0.5 after:left-[2px] after:bg-white after:border-gray-300 after:border after:rounded-full after:h-5 after:w-5 after:transition-all peer-checked:bg-[#6405d6]"></div>
                </label>
              </div>

              <div className="flex items-center justify-between">
                <div>
                  <h4 className="text-sm font-medium text-gray-200">Usar notificações nativas</h4>
                  <p className="text-xs text-gray-400">Usa as notificações do sistema operacional quando disponível</p>
                </div>
                <label className="relative inline-flex items-center cursor-pointer">
                  <input
                    type="checkbox"
                    className="sr-only peer"
                    checked={settings.useNative}
                    onChange={(e) => handleToggle('useNative', e.target.checked)}
                  />
                  <div className="w-11 h-6 bg-gray-700 rounded-full peer peer-checked:after:translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-0.5 after:left-[2px] after:bg-white after:border-gray-300 after:border after:rounded-full after:h-5 after:w-5 after:transition-all peer-checked:bg-[#6405d6]"></div>
                </label>
              </div>

              <div className="flex items-center justify-between">
                <div>
                  <h4 className="text-sm font-medium text-gray-200">Mostrar quando em foco</h4>
                  <p className="text-xs text-gray-400">Mostra notificações mesmo quando a janela está em primeiro plano</p>
                </div>
                <label className="relative inline-flex items-center cursor-pointer">
                  <input
                    type="checkbox"
                    className="sr-only peer"
                    checked={settings.showWhenFocused}
                    onChange={(e) => handleToggle('showWhenFocused', e.target.checked)}
                    disabled={!settings.useNative}
                  />
                  <div className={`w-11 h-6 rounded-full peer peer-checked:after:translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-0.5 after:left-[2px] after:bg-white after:border-gray-300 after:border after:rounded-full after:h-5 after:w-5 after:transition-all peer-checked:bg-[#6405d6] ${!settings.useNative ? 'bg-gray-800 opacity-50 cursor-not-allowed' : 'bg-gray-700'}`}></div>
                </label>
              </div>
            </div>

            <div className="pt-4 border-t border-gray-700">
              <h4 className="text-sm font-medium text-gray-200 mb-3">Tipos de notificação</h4>
              <div className="space-y-2">
                {Object.entries(settings.types).map(([type, enabled]) => (
                  <div key={type} className="flex items-center justify-between">
                    <span className="text-sm text-gray-300 capitalize">
                      {type === 'update' ? 'Atualizações' : 
                       type === 'reminder' ? 'Lembretes' : 
                       type.charAt(0).toUpperCase() + type.slice(1)}
                    </span>
                    <label className="relative inline-flex items-center cursor-pointer">
                      <input
                        type="checkbox"
                        className="sr-only peer"
                        checked={enabled}
                        onChange={(e) => handleTypeToggle(type as keyof typeof settings.types, e.target.checked)}
                      />
                      <div className="w-11 h-6 bg-gray-700 rounded-full peer peer-checked:after:translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-0.5 after:left-[2px] after:bg-white after:border-gray-300 after:border after:rounded-full after:h-5 after:w-5 after:transition-all peer-checked:bg-[#6405d6]"></div>
                    </label>
                  </div>
                ))}
              </div>
            </div>
          </>
        )}
      </div>
    </div>
  );
};
