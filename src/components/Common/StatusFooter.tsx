import { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';

export const StatusFooter = () => {
  interface IndexingStatus {
    current: number;
    total: number;
    isIndexing: boolean;
  }

  interface AppStatus {
    indexing: IndexingStatus;
    isOnline: boolean;
    iaStatus: 'active' | 'inactive' | 'error';
    version: string;
    updateAvailable: boolean;
  }

  const [status, setStatus] = useState<AppStatus>({
    indexing: { current: 0, total: 0, isIndexing: false },
    isOnline: true,
    iaStatus: 'active',
    version: 'v1.0.0',
    updateAvailable: false
  });

  useEffect(() => {
    // Simulando atualização de status
    const interval = setInterval(() => {
      // Em produção, isso viria de uma chamada para o backend
      setStatus(prev => ({
        ...prev,
        indexing: {
          ...prev.indexing,
          current: Math.min(prev.indexing.current + 1000, 15000),
          total: 15000,
          isIndexing: prev.indexing.current < 15000
        },
        isOnline: navigator.onLine
      }));
    }, 5000);

    return () => clearInterval(interval);
  }, []);

  const indexingProgress = status.indexing.total > 0 
    ? Math.round((status.indexing.current / status.indexing.total) * 100) 
    : 0;

  return (
    <footer className="bg-[#0f0a1a] border-t border-gray-800 p-2 text-xs text-gray-400">
      <div className="container mx-auto px-4">
        <div className="flex flex-wrap items-center justify-between">
          {/* Status da indexação */}
          <div className="flex items-center space-x-4">
            {status.indexing.isIndexing ? (
              <div className="flex items-center">
                <span className="inline-block w-2 h-2 rounded-full bg-yellow-500 mr-2 animate-pulse"></span>
                <span>Indexando... {indexingProgress}%</span>
              </div>
            ) : (
              <div className="flex items-center">
                <span className="inline-block w-2 h-2 rounded-full bg-green-500 mr-2"></span>
                <span>Índice atualizado</span>
              </div>
            )}

            {/* Status de conexão */}
            <div className="hidden md:flex items-center">
              <span className="inline-block w-2 h-2 rounded-full bg-green-500 mr-2"></span>
              <span>{status.isOnline ? 'Online' : 'Offline'}</span>
            </div>

            {/* Status da IA */}
            <div className="hidden md:flex items-center">
              <span className="inline-block w-2 h-2 rounded-full bg-green-500 mr-2"></span>
              <span>IA {status.iaStatus === 'active' ? 'Ativa' : 'Inativa'}</span>
            </div>
          </div>

          {/* Links do rodapé */}
          <div className="flex items-center space-x-4">
            <a 
              href="#" 
              className="hover:text-white transition-colors"
              onClick={(e) => {
                e.preventDefault();
                // Abrir tela de ajuda
              }}
            >
              Ajuda (?) 
              <span className="hidden md:inline">Pressione '?' para ajuda</span>
            </a>
            <a href="#" className="hover:text-white transition-colors hidden md:inline">
              Política de Privacidade
            </a>
            <a href="#" className="hover:text-white transition-colors hidden md:inline">
              Termos de Serviço
            </a>
            <button 
              className="text-primary-purple hover:text-purple-400 transition-colors"
              onClick={async () => {
                try {
                  await invoke('open_feedback_form');
                } catch (error) {
                  console.error('Erro ao abrir formulário de feedback:', error);
                }
              }}
            >
              Enviar Feedback
            </button>
          </div>

          {/* Versão */}
          <div className="flex items-center">
            <span>{status.version} </span>
            {status.updateAvailable && (
              <span className="ml-2 px-2 py-0.5 bg-blue-900 text-blue-100 text-xs rounded-full">
                Atualização disponível
              </span>
            )}
          </div>
        </div>
      </div>
    </footer>
  );
};
