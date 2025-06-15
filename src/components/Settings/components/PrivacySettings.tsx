import React, { useState } from 'react';

interface PrivacySettingsProps {
  settings: {
    telemetry: boolean;
    crashReports: boolean;
    analytics: boolean;
    errorReporting: boolean;
  };
  onChange: (key: string, value: any) => void;
}

export const PrivacySettings: React.FC<PrivacySettingsProps> = ({ settings, onChange }) => {
  const [showExportOptions, setShowExportOptions] = useState(false);
  const [selectedDataTypes, setSelectedDataTypes] = useState<string[]>([]);
  const [isClearingCache, setIsClearingCache] = useState(false);
  const [isExportingData, setIsExportingData] = useState(false);
  const [isDeletingData, setIsDeletingData] = useState(false);

  const dataTypes = [
    { id: 'history', label: 'Histórico de Buscas' },
    { id: 'preferences', label: 'Preferências' },
    { id: 'usage', label: 'Dados de Uso' },
    { id: 'cache', label: 'Cache de Dados' },
  ];

  const toggleDataType = (id: string) => {
    setSelectedDataTypes(prev => 
      prev.includes(id) 
        ? prev.filter(item => item !== id) 
        : [...prev, id]
    );
  };

  const handleToggleSetting = (key: string, value: boolean) => {
    onChange(key, value);
  };

  const clearCache = async () => {
    try {
      setIsClearingCache(true);
      // Simulando limpeza de cache
      await new Promise(resolve => setTimeout(resolve, 1500));
      // Aqui você chamaria a API para limpar o cache
      // await invoke('clear_cache');
      alert('Cache limpo com sucesso!');
    } catch (error) {
      console.error('Erro ao limpar cache:', error);
      alert('Ocorreu um erro ao limpar o cache.');
    } finally {
      setIsClearingCache(false);
    }
  };

  const exportData = async () => {
    if (selectedDataTypes.length === 0) {
      alert('Selecione pelo menos um tipo de dado para exportar');
      return;
    }

    try {
      setIsExportingData(true);
      // Simulando exportação de dados
      await new Promise(resolve => setTimeout(resolve, 2000));
      // Aqui você chamaria a API para exportar os dados
      // const result = await invoke('export_data', { dataTypes: selectedDataTypes });
      alert(`Dados exportados com sucesso: ${selectedDataTypes.join(', ')}`);
      setShowExportOptions(false);
      setSelectedDataTypes([]);
    } catch (error) {
      console.error('Erro ao exportar dados:', error);
      alert('Ocorreu um erro ao exportar os dados.');
    } finally {
      setIsExportingData(false);
    }
  };

  const deleteAllData = async () => {
    if (!confirm('Tem certeza que deseja excluir todos os seus dados? Esta ação não pode ser desfeita.')) {
      return;
    }

    try {
      setIsDeletingData(true);
      // Simulando exclusão de dados
      await new Promise(resolve => setTimeout(resolve, 2000));
      // Aqui você chamaria a API para excluir os dados
      // await invoke('delete_user_data');
      alert('Todos os seus dados foram excluídos com sucesso!');
      // Recarregar a aplicação ou redirecionar
      // window.location.reload();
    } catch (error) {
      console.error('Erro ao excluir dados:', error);
      alert('Ocorreu um erro ao excluir seus dados.');
    } finally {
      setIsDeletingData(false);
    }
  };

  return (
    <div className="space-y-8">
      <div>
        <h3 className="text-lg font-semibold text-white mb-4">Configurações de Privacidade</h3>
        
        <div className="space-y-6">
          <div className="bg-[#1a0a2e] p-4 rounded-lg border border-[#2a1a3a]">
            <div className="flex items-start">
              <div className="flex items-center h-5">
                <input
                  id="telemetry"
                  name="telemetry"
                  type="checkbox"
                  className="focus:ring-[#6405d6] h-4 w-4 text-[#6405d6] border-[#2a1a3a] rounded"
                  checked={settings.telemetry}
                  onChange={(e) => handleToggleSetting('telemetry', e.target.checked)}
                />
              </div>
              <div className="ml-3 text-sm">
                <label htmlFor="telemetry" className="font-medium text-gray-200">
                  Compartilhar dados de telemetria anônimos
                </label>
                <p className="text-gray-400">
                  Ajude-nos a melhorar o Flowlight enviando dados de uso anônimos.
                </p>
              </div>
            </div>
          </div>

          <div className="bg-[#1a0a2e] p-4 rounded-lg border border-[#2a1a3a]">
            <div className="flex items-start">
              <div className="flex items-center h-5">
                <input
                  id="crash-reports"
                  name="crash-reports"
                  type="checkbox"
                  className="focus:ring-[#6405d6] h-4 w-4 text-[#6405d6] border-[#2a1a3a] rounded"
                  checked={settings.crashReports}
                  onChange={(e) => handleToggleSetting('crashReports', e.target.checked)}
                />
              </div>
              <div className="ml-3 text-sm">
                <label htmlFor="crash-reports" className="font-medium text-gray-200">
                  Enviar relatórios de falhas automaticamente
                </label>
                <p className="text-gray-400">
                  Ajude-nos a identificar e corrigir problemas mais rapidamente.
                </p>
              </div>
            </div>
          </div>

          <div className="space-y-4">
            <h4 className="text-md font-medium text-gray-200">Dados e Armazenamento</h4>
            
            <div className="space-y-4">
              <div className="flex justify-between items-center">
                <div>
                  <p className="text-sm font-medium text-gray-200">Limpar cache</p>
                  <p className="text-xs text-gray-400">Remove arquivos temporários e dados em cache</p>
                </div>
                <button
                  onClick={clearCache}
                  disabled={isClearingCache}
                  className={`px-4 py-2 text-sm rounded-md border ${
                    isClearingCache
                      ? 'bg-gray-700 text-gray-400 border-gray-700 cursor-not-allowed'
                      : 'bg-[#1a0a2e] hover:bg-[#2a1a3a] text-gray-200 border-[#2a1a3a]'
                  }`}
                >
                  {isClearingCache ? 'Limpando...' : 'Limpar Agora'}
                </button>
              </div>

              <div className="flex justify-between items-center">
                <div>
                  <p className="text-sm font-medium text-gray-200">Exportar meus dados</p>
                  <p className="text-xs text-gray-400">Baixe uma cópia dos seus dados</p>
                </div>
                <div className="relative">
                  <button
                    onClick={() => setShowExportOptions(!showExportOptions)}
                    className="px-4 py-2 text-sm bg-[#1a0a2e] hover:bg-[#2a1a3a] text-gray-200 rounded-md border border-[#2a1a3a]"
                  >
                    Exportar Dados
                  </button>
                  
                  {showExportOptions && (
                    <div className="absolute right-0 mt-2 w-64 bg-[#1a0a2e] rounded-md shadow-lg border border-[#2a1a3a] z-10">
                      <div className="p-2">
                        <p className="text-xs text-gray-400 mb-2">Selecione os dados para exportar:</p>
                        {dataTypes.map((item) => (
                          <div key={item.id} className="flex items-center p-1 hover:bg-[#2a1a3a] rounded">
                            <input
                              type="checkbox"
                              id={`export-${item.id}`}
                              checked={selectedDataTypes.includes(item.id)}
                              onChange={() => toggleDataType(item.id)}
                              className="h-4 w-4 text-[#6405d6] focus:ring-[#6405d6] border-[#2a1a3a] rounded"
                            />
                            <label htmlFor={`export-${item.id}`} className="ml-2 block text-sm text-gray-200">
                              {item.label}
                            </label>
                          </div>
                        ))}
                        <div className="mt-2 pt-2 border-t border-[#2a1a3a] flex justify-end">
                          <button
                            onClick={() => setShowExportOptions(false)}
                            className="px-3 py-1 text-sm text-gray-300 hover:text-white mr-2"
                          >
                            Cancelar
                          </button>
                          <button
                            onClick={exportData}
                            disabled={selectedDataTypes.length === 0 || isExportingData}
                            className={`px-3 py-1 text-sm rounded-md ${
                              selectedDataTypes.length > 0 && !isExportingData
                                ? 'bg-[#6405d6] text-white hover:bg-[#7a1ae6]'
                                : 'bg-gray-700 text-gray-400 cursor-not-allowed'
                            }`}
                          >
                            {isExportingData ? (
                              <>
                                <svg className="animate-spin -ml-1 mr-2 h-4 w-4 text-white inline" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
                                  <circle className="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" strokeWidth="4"></circle>
                                  <path className="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
                                </svg>
                                Exportando...
                              </>
                            ) : 'Exportar'}
                          </button>
                        </div>
                      </div>
                    </div>
                  )}
                </div>
              </div>

              <div className="flex justify-between items-center">
                <div>
                  <p className="text-sm font-medium text-gray-200">Excluir todos os meus dados</p>
                  <p className="text-xs text-gray-400">Remove permanentemente todos os seus dados do Flowlight</p>
                </div>
                <button
                  onClick={deleteAllData}
                  disabled={isDeletingData}
                  className={`px-4 py-2 text-sm rounded-md border ${
                    isDeletingData
                      ? 'bg-gray-700 text-gray-500 border-gray-700 cursor-not-allowed'
                      : 'bg-red-900/30 hover:bg-red-900/50 text-red-400 border-red-900/50'
                  }`}
                >
                  {isDeletingData ? 'Excluindo...' : 'Excluir Tudo'}
                </button>
              </div>
            </div>
          </div>
        </div>
      </div>

      <div className="bg-[#1a0a2e] border-l-4 border-blue-500 p-4 rounded-md">
        <div className="flex">
          <div className="flex-shrink-0">
            <svg className="h-5 w-5 text-blue-400" viewBox="0 0 20 20" fill="currentColor">
              <path fillRule="evenodd" d="M18 10a8 8 0 11-16 0 8 8 0 0116 0zm-7-4a1 1 0 11-2 0 1 1 0 012 0zM9 9a1 1 0 000 2v3a1 1 0 001 1h1a1 1 0 100-2v-3a1 1 0 00-1-1H9z" clipRule="evenodd" />
            </svg>
          </div>
          <div className="ml-3">
            <h3 className="text-sm font-medium text-blue-200">Sua privacidade é importante</h3>
            <div className="mt-2 text-sm text-blue-100">
              <p>
                Nós levamos sua privacidade a sério. Seus dados são armazenados localmente no seu computador e nunca são compartilhados sem sua permissão explícita.
              </p>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
};
