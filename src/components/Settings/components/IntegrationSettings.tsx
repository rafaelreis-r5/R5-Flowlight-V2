import React, { useState } from 'react';
import { FiLink, FiCheck, FiX, FiArrowUpRight } from 'react-icons/fi';

interface Integration {
  id: string;
  name: string;
  description: string;
  icon: string;
  connected: boolean;
  authUrl?: string;
}

export const IntegrationSettings: React.FC = () => {
  const [integrations, setIntegrations] = useState<Integration[]>([
    {
      id: 'github',
      name: 'GitHub',
      description: 'Acesse seus repositórios e códigos diretamente do Flowlight',
      icon: 'github',
      connected: false,
      authUrl: 'https://github.com/login/oauth/authorize?client_id=YOUR_CLIENT_ID&scope=repo'
    },
    {
      id: 'slack',
      name: 'Slack',
      description: 'Receba notificações e interaja com o Slack',
      icon: 'slack',
      connected: true
    },
    {
      id: 'google',
      name: 'Google Workspace',
      description: 'Integração com Gmail, Google Drive e outros serviços do Google',
      icon: 'google',
      connected: false,
      authUrl: 'https://accounts.google.com/o/oauth2/auth?client_id=YOUR_CLIENT_ID&redirect_uri=YOUR_REDIRECT_URI&scope=email profile&response_type=code'
    },
    {
      id: 'notion',
      name: 'Notion',
      description: 'Acesse suas páginas e bancos de dados do Notion',
      icon: 'notion',
      connected: false,
      authUrl: 'https://api.notion.com/v1/oauth/authorize?client_id=YOUR_CLIENT_ID&response_type=code&owner=user&redirect_uri=YOUR_REDIRECT_URI'
    },
    {
      id: 'figma',
      name: 'Figma',
      description: 'Acesse e visualize seus designs do Figma',
      icon: 'figma',
      connected: false,
      authUrl: 'https://www.figma.com/oauth?client_id=YOUR_CLIENT_ID&redirect_uri=YOUR_REDIRECT_URI&scope=file_read&response_type=code&state=YOUR_STATE'
    },
  ]);

  const [customWebhook, setCustomWebhook] = useState('');
  const [webhooks, setWebhooks] = useState<string[]>([]);
  const [activeTab, setActiveTab] = useState('connected');

  const toggleIntegration = (id: string) => {
    setIntegrations(integrations.map(integration => 
      integration.id === id 
        ? { ...integration, connected: !integration.connected } 
        : integration
    ));
  };

  const addWebhook = () => {
    if (customWebhook && !webhooks.includes(customWebhook)) {
      setWebhooks([...webhooks, customWebhook]);
      setCustomWebhook('');
    }
  };

  const removeWebhook = (url: string) => {
    setWebhooks(webhooks.filter(webhook => webhook !== url));
  };

  const renderIntegrationIcon = (icon: string) => {
    // Em uma implementação real, você usaria os ícones reais das integrações
    return (
      <div className="w-10 h-10 rounded-full bg-[#2a1a3a] flex items-center justify-center text-white">
        {icon.charAt(0).toUpperCase()}
      </div>
    );
  };

  const filteredIntegrations = activeTab === 'connected' 
    ? integrations.filter(i => i.connected)
    : activeTab === 'available'
    ? integrations.filter(i => !i.connected)
    : [];

  return (
    <div className="space-y-8">
      <div>
        <h3 className="text-lg font-semibold text-white mb-6">Integrações</h3>
        
        <div className="flex space-x-2 mb-6 border-b border-[#2a1a3a]">
          <button
            onClick={() => setActiveTab('connected')}
            className={`px-4 py-2 text-sm font-medium ${
              activeTab === 'connected'
                ? 'text-[#6405d6] border-b-2 border-[#6405d6]'
                : 'text-gray-400 hover:text-white'
            }`}
          >
            Conectadas
          </button>
          <button
            onClick={() => setActiveTab('available')}
            className={`px-4 py-2 text-sm font-medium ${
              activeTab === 'available'
                ? 'text-[#6405d6] border-b-2 border-[#6405d6]'
                : 'text-gray-400 hover:text-white'
            }`}
          >
            Disponíveis
          </button>
        </div>

        <div className="space-y-4">
          {filteredIntegrations.length > 0 ? (
            filteredIntegrations.map((integration) => (
              <div key={integration.id} className="bg-[#1a0a2e] p-4 rounded-lg border border-[#2a1a3a] flex items-center justify-between">
                <div className="flex items-center space-x-4">
                  {renderIntegrationIcon(integration.icon)}
                  <div>
                    <h4 className="text-sm font-medium text-white">{integration.name}</h4>
                    <p className="text-xs text-gray-400">{integration.description}</p>
                  </div>
                </div>
                <div>
                  {integration.connected ? (
                    <button
                      onClick={() => toggleIntegration(integration.id)}
                      className="flex items-center space-x-1 px-3 py-1.5 text-xs bg-red-900/30 hover:bg-red-900/50 text-red-400 rounded-md border border-red-900/50"
                    >
                      <FiX size={14} />
                      <span>Desconectar</span>
                    </button>
                  ) : integration.authUrl ? (
                    <a
                      href={integration.authUrl}
                      target="_blank"
                      rel="noopener noreferrer"
                      className="flex items-center space-x-1 px-3 py-1.5 text-xs bg-[#6405d6] hover:bg-[#7a1ae6] text-white rounded-md"
                    >
                      <FiLink size={14} />
                      <span>Conectar</span>
                    </a>
                  ) : (
                    <button
                      onClick={() => toggleIntegration(integration.id)}
                      className="flex items-center space-x-1 px-3 py-1.5 text-xs bg-[#6405d6] hover:bg-[#7a1ae6] text-white rounded-md"
                    >
                      <FiCheck size={14} />
                      <span>Conectar</span>
                    </button>
                  )}
                </div>
              </div>
            ))
          ) : (
            <div className="text-center py-8">
              <FiLink className="mx-auto h-12 w-12 text-gray-600" />
              <h4 className="mt-2 text-sm font-medium text-gray-300">
                {activeTab === 'connected' 
                  ? 'Nenhuma integração conectada' 
                  : 'Todas as integrações disponíveis estão conectadas'}
              </h4>
              <p className="mt-1 text-xs text-gray-500">
                {activeTab === 'connected'
                  ? 'Conecte-se a serviços para expandir as funcionalidades do Flowlight.'
                  : 'Todas as integrações disponíveis já estão conectadas.'}
              </p>
            </div>
          )}
        </div>
      </div>

      <div>
        <h4 className="text-md font-medium text-white mb-4">Webhooks Personalizados</h4>
        
        <div className="space-y-4">
          <div className="flex space-x-2">
            <div className="flex-1">
              <input
                type="text"
                value={customWebhook}
                onChange={(e) => setCustomWebhook(e.target.value)}
                placeholder="https://exemplo.com/webhook"
                className="w-full bg-[#1a0a2e] border border-[#2a1a3a] rounded-md px-3 py-2 text-white placeholder-gray-500 focus:outline-none focus:ring-1 focus:ring-[#6405d6] focus:border-transparent"
              />
            </div>
            <button
              onClick={addWebhook}
              disabled={!customWebhook}
              className={`px-4 py-2 rounded-md text-sm font-medium ${
                customWebhook
                  ? 'bg-[#6405d6] hover:bg-[#7a1ae6] text-white'
                  : 'bg-gray-700 text-gray-400 cursor-not-allowed'
              }`}
            >
              Adicionar
            </button>
          </div>
          
          {webhooks.length > 0 && (
            <div className="space-y-2">
              {webhooks.map((webhook, index) => (
                <div key={index} className="flex items-center justify-between bg-[#1a0a2e] p-3 rounded-md border border-[#2a1a3a]">
                  <div className="flex items-center">
                    <FiLink className="text-[#6405d6] mr-2" />
                    <span className="text-sm text-gray-300 truncate max-w-xs">{webhook}</span>
                  </div>
                  <button
                    onClick={() => removeWebhook(webhook)}
                    className="text-gray-400 hover:text-white p-1"
                  >
                    <FiX size={18} />
                  </button>
                </div>
              ))}
            </div>
          )}
          
          <p className="text-xs text-gray-400 mt-2">
            Adicione webhooks personalizados para receber notificações e atualizações.
          </p>
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
            <h3 className="text-sm font-medium text-blue-200">Desenvolvedores</h3>
            <div className="mt-2 text-sm text-blue-100">
              <p>
                Deseja criar uma integração personalizada? Consulte nossa 
                <a 
                  href="#" 
                  className="underline hover:text-white flex items-center"
                  target="_blank"
                  rel="noopener noreferrer"
                >
                  documentação para desenvolvedores <FiArrowUpRight className="ml-1" size={12} />
                </a>
              </p>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
};
