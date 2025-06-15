import React, { useState, useEffect } from 'react';
import { FiGithub, FiGlobe, FiDownload, FiRefreshCw, FiCheck } from 'react-icons/fi';

interface UpdateInfo {
  available: boolean;
  version?: string;
  releaseDate?: string;
  changelog?: string;
}

export const AboutSettings: React.FC = () => {
  const [isChecking, setIsChecking] = useState(false);
  const [updateInfo, setUpdateInfo] = useState<UpdateInfo>({ available: false });
  const [appVersion, setAppVersion] = useState('1.0.0');
  const [isUpdating, setIsUpdating] = useState(false);
  const [updateProgress, setUpdateProgress] = useState(0);

  // Simular carregamento da versão do app
  useEffect(() => {
    // Em um app real, você buscaria isso do package.json ou similar
    setAppVersion('1.2.3');
  }, []);

  const checkForUpdates = () => {
    setIsChecking(true);
    
    // Simular verificação de atualização
    setTimeout(() => {
      const hasUpdate = Math.random() > 0.5; // 50% de chance de ter atualização
      
      if (hasUpdate) {
        setUpdateInfo({
          available: true,
          version: '1.3.0',
          releaseDate: '2023-05-15',
          changelog: '• Melhorias de desempenho\n• Novos atalhos de teclado\n• Correções de bugs',
        });
      } else {
        setUpdateInfo({ available: false });
      }
      
      setIsChecking(false);
    }, 1500);
  };

  const installUpdate = () => {
    setIsUpdating(true);
    setUpdateProgress(0);
    
    // Simular download da atualização
    const interval = setInterval(() => {
      setUpdateProgress(prev => {
        const newProgress = prev + Math.floor(Math.random() * 10) + 5;
        if (newProgress >= 100) {
          clearInterval(interval);
          // Aqui você instalaria a atualização
          return 100;
        }
        return newProgress;
      });
    }, 300);
  };

  const openExternalLink = (url: string) => {
    window.open(url, '_blank', 'noopener,noreferrer');
  };

  return (
    <div className="space-y-8">
      <div className="text-center">
        <div className="mx-auto h-20 w-20 bg-gradient-to-r from-[#6405d6] to-[#f1011d] rounded-2xl flex items-center justify-center text-white text-2xl font-bold mb-4">
          FL
        </div>
        <h2 className="text-2xl font-bold text-white">R5 Flowlight</h2>
        <p className="text-gray-400 mt-1">Versão {appVersion}</p>
        
        <div className="mt-6 flex justify-center space-x-4">
          <button
            onClick={() => openExternalLink('https://github.com/rafaelreis-r5/flowlight')}
            className="inline-flex items-center px-4 py-2 border border-[#2a1a3a] rounded-md text-sm font-medium text-gray-300 bg-[#1a0a2e] hover:bg-[#2a1a3a]"
          >
            <FiGithub className="mr-2" />
            GitHub
          </button>
          <button
            onClick={() => openExternalLink('https://flowlight.app')}
            className="inline-flex items-center px-4 py-2 border border-[#2a1a3a] rounded-md text-sm font-medium text-gray-300 bg-[#1a0a2e] hover:bg-[#2a1a3a]"
          >
            <FiGlobe className="mr-2" />
            Website
          </button>
        </div>
      </div>

      <div className="bg-[#1a0a2e] p-6 rounded-lg border border-[#2a1a3a]">
        <h3 className="text-lg font-semibold text-white mb-4">Atualizações</h3>
        
        {updateInfo.available ? (
          <div className="space-y-4">
            <div className="p-4 bg-[#0f0a1a] rounded-md border border-[#6405d6]/30">
              <div className="flex items-start">
                <div className="flex-shrink-0 pt-0.5">
                  <div className="flex items-center justify-center h-6 w-6 rounded-full bg-[#6405d6]/20">
                    <FiDownload className="h-4 w-4 text-[#6405d6]" />
                  </div>
                </div>
                <div className="ml-3 flex-1">
                  <p className="text-sm font-medium text-white">
                    Nova versão disponível: v{updateInfo.version}
                  </p>
                  <p className="mt-1 text-xs text-gray-400">
                    Lançada em {updateInfo.releaseDate}
                  </p>
                  
                  <div className="mt-2">
                    <h4 className="text-xs font-medium text-gray-300 mb-1">Novidades:</h4>
                    <pre className="text-xs text-gray-400 whitespace-pre-wrap font-sans">
                      {updateInfo.changelog}
                    </pre>
                  </div>
                  
                  {!isUpdating ? (
                    <div className="mt-3">
                      <button
                        onClick={installUpdate}
                        className="inline-flex items-center px-3 py-1.5 border border-transparent text-xs font-medium rounded-md shadow-sm text-white bg-[#6405d6] hover:bg-[#7a1ae6] focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-[#6405d6]"
                      >
                        <FiDownload className="mr-1.5 h-3.5 w-3.5" />
                        Instalar Atualização
                      </button>
                    </div>
                  ) : (
                    <div className="mt-3">
                      <div className="flex items-center">
                        <div className="w-full bg-gray-700 rounded-full h-2">
                          <div 
                            className="bg-gradient-to-r from-[#6405d6] to-[#f1011d] h-2 rounded-full transition-all duration-300 ease-out" 
                            style={{ width: `${updateProgress}%` }}
                          ></div>
                        </div>
                        <span className="ml-2 text-xs text-gray-400 w-10">{updateProgress}%</span>
                      </div>
                      <p className="mt-1 text-xs text-gray-400">
                        Baixando atualização...
                      </p>
                    </div>
                  )}
                </div>
              </div>
            </div>
          </div>
        ) : (
          <div className="text-center py-4">
            <FiCheck className="mx-auto h-8 w-8 text-green-500" />
            <h4 className="mt-2 text-sm font-medium text-gray-200">
              Você está usando a versão mais recente
            </h4>
            <p className="mt-1 text-xs text-gray-400">
              Última verificação: {new Date().toLocaleDateString()}
            </p>
            <button
              onClick={checkForUpdates}
              disabled={isChecking}
              className={`mt-4 inline-flex items-center px-3 py-1.5 border border-transparent text-xs font-medium rounded-md shadow-sm ${
                isChecking
                  ? 'bg-gray-600 text-gray-300 cursor-not-allowed'
                  : 'text-white bg-[#6405d6] hover:bg-[#7a1ae6]'
              }`}
            >
              {isChecking ? (
                <>
                  <FiRefreshCw className="animate-spin mr-1.5 h-3.5 w-3.5" />
                  Verificando...
                </>
              ) : (
                <>
                  <FiRefreshCw className="mr-1.5 h-3.5 w-3.5" />
                  Verificar Atualizações
                </>
              )}
            </button>
          </div>
        )}
      </div>

      <div className="space-y-4">
        <h3 className="text-lg font-semibold text-white">Sobre</h3>
        
        <div className="bg-[#1a0a2e] p-4 rounded-lg border border-[#2a1a3a]">
          <p className="text-sm text-gray-300">
            Flowlight é um lançador de produtividade de código aberto projetado para desenvolvedores e profissionais de tecnologia.
          </p>
          
          <div className="mt-4 pt-4 border-t border-[#2a1a3a] grid grid-cols-1 md:grid-cols-2 gap-4">
            <div>
              <h4 className="text-xs font-medium text-gray-400 uppercase tracking-wider mb-2">Desenvolvido por</h4>
              <p className="text-sm text-gray-300">R5 Tech</p>
            </div>
            <div>
              <h4 className="text-xs font-medium text-gray-400 uppercase tracking-wider mb-2">Licença</h4>
              <p className="text-sm text-gray-300">MIT License</p>
            </div>
            <div>
              <h4 className="text-xs font-medium text-gray-400 uppercase tracking-wider mb-2">Versão</h4>
              <p className="text-sm text-gray-300">{appVersion} (Build 230501)</p>
            </div>
            <div>
              <h4 className="text-xs font-medium text-gray-400 uppercase tracking-wider mb-2">Lançamento</h4>
              <p className="text-sm text-gray-300">1 de Maio, 2023</p>
            </div>
          </div>
          
          <div className="mt-6 pt-4 border-t border-[#2a1a3a] flex flex-wrap gap-4">
            <button
              onClick={() => openExternalLink('https://github.com/rafaelreis-r5/flowlight/issues')}
              className="text-sm text-gray-400 hover:text-white"
            >
              Reportar um problema
            </button>
            <button
              onClick={() => openExternalLink('https://github.com/rafaelreis-r5/flowlight/discussions')}
              className="text-sm text-gray-400 hover:text-white"
            >
              Sugerir uma funcionalidade
            </button>
            <button
              onClick={() => openExternalLink('https://github.com/rafaelreis-r5/flowlight/blob/main/CHANGELOG.md')}
              className="text-sm text-gray-400 hover:text-white"
            >
              Histórico de alterações
            </button>
            <button
              onClick={() => openExternalLink('https://github.com/rafaelreis-r5/flowlight/graphs/contributors')}
              className="text-sm text-gray-400 hover:text-white"
            >
              Contribuidores
            </button>
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
            <h3 className="text-sm font-medium text-blue-200">Créditos e Agradecimentos</h3>
            <div className="mt-2 text-sm text-blue-100">
              <p>
                Agradecimentos a todos os contribuidores de código aberto e bibliotecas que tornaram este projeto possível.
              </p>
              <p className="mt-2">
                Feito com ❤️ pela comunidade de código aberto.
              </p>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
};
