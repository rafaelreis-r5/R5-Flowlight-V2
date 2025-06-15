import React, { useState, useEffect } from 'react';
import { Settings } from '../../../types/settings';
import { FiCopy, FiCheck } from 'react-icons/fi';

interface AISettingsProps {
  settings: Settings['ai'];
  onChange: (key: keyof Settings['ai'], value: Settings['ai'][keyof Settings['ai']]) => void;
}

export const AISettings: React.FC<AISettingsProps> = ({ settings, onChange }) => {
  const [copied, setCopied] = useState(false);
  const [apiKey, setApiKey] = useState(settings.apiKey);
  useEffect(() => {
    setApiKey(settings.apiKey);
  }, [settings.apiKey]);
  const [showApiKey, setShowApiKey] = useState(false);

  const copyToClipboard = () => {
    navigator.clipboard.writeText(apiKey);
    setCopied(true);
    setTimeout(() => setCopied(false), 2000);
  };

  const toggleApiKeyVisibility = () => {
    setShowApiKey(!showApiKey);
  };

  const models = [
    { id: 'gpt-4', name: 'GPT-4', description: 'Mais avançado, ótimo para tarefas complexas' },
    { id: 'gpt-3.5-turbo', name: 'GPT-3.5 Turbo', description: 'Rápido e eficiente para a maioria das tarefas' },
    { id: 'claude-2', name: 'Claude 2', description: 'Bom para tarefas de raciocínio' },
  ];

  return (
    <div className="space-y-8">
      <div>
        <h3 className="text-lg font-semibold text-white mb-4">Configurações da IA</h3>

        <div className="space-y-6">
          <div>
            <label htmlFor="apiKey" className="block text-sm font-medium text-gray-200 mb-1">
              Chave da API
            </label>
            <div className="flex rounded-md shadow-sm">
              <div className="relative flex items-stretch flex-grow focus-within:z-10">
                <input
                  type={showApiKey ? 'text' : 'password'}
                  id="apiKey"
                  value={showApiKey ? apiKey : '•'.repeat(apiKey.length)}
                  onChange={(e) => {
                    const value = e.target.value;
                    setApiKey(value);
                    onChange('apiKey', value);
                  }}
                  className="focus:ring-1 focus:ring-[#6405d6] focus:border-[#6405d6] block w-full rounded-none rounded-l-md bg-[#1a0a2e] border-[#2a1a3a] text-white"
                  placeholder="Insira sua chave da API"
                />
              </div>
              <button
                type="button"
                onClick={toggleApiKeyVisibility}
                className="-ml-px relative inline-flex items-center space-x-2 px-4 py-2 border border-[#2a1a3a] text-sm font-medium rounded-r-md text-gray-300 bg-[#1a0a2e] hover:bg-[#2a1a3a] focus:outline-none focus:ring-1 focus:ring-[#6405d6]"
              >
                {showApiKey ? 'Ocultar' : 'Mostrar'}
              </button>
              <button
                type="button"
                onClick={copyToClipboard}
                className="ml-2 relative inline-flex items-center px-3 py-2 border border-[#2a1a3a] text-sm font-medium rounded-md text-gray-300 bg-[#1a0a2e] hover:bg-[#2a1a3a] focus:outline-none focus:ring-1 focus:ring-[#6405d6]"
                title="Copiar para a área de transferência"
              >
                {copied ? <FiCheck className="text-green-400" /> : <FiCopy />}
              </button>
            </div>
            <p className="mt-1 text-xs text-gray-400">
              Sua chave é armazenada localmente e nunca é enviada para nossos servidores.
            </p>
          </div>

          <div>
            <label htmlFor="model" className="block text-sm font-medium text-gray-200 mb-1">
              Modelo de IA
            </label>
            <select
              id="model"
              className="mt-1 block w-full pl-3 pr-10 py-2 text-base bg-[#1a0a2e] border-[#2a1a3a] text-white focus:outline-none focus:ring-1 focus:ring-[#6405d6] focus:border-[#6405d6] sm:text-sm rounded-md"
              value={settings.model}
              onChange={(e) => onChange('model', e.target.value)}
            >
              {models.map((model) => (
                <option key={model.id} value={model.id}>
                  {model.name} - {model.description}
                </option>
              ))}
            </select>
          </div>

          <div>
            <label htmlFor="temperature" className="block text-sm font-medium text-gray-200 mb-1">
              Criatividade: <span className="text-[#6405d6]">{settings.temperature || '0.7'}</span>
            </label>
            <div className="space-y-2">
              <input
                type="range"
                id="temperature"
                min="0"
                max="1"
                step="0.1"
                value={settings.temperature}
                onChange={(e) => onChange('temperature', parseFloat(e.target.value))}
                className="w-full h-2 bg-gray-700 rounded-lg appearance-none cursor-pointer"
              />
              <div className="flex justify-between text-xs text-gray-400">
                <span>Preciso</span>
                <span>Balanceado</span>
                <span>Criativo</span>
              </div>
            </div>
          </div>

          <div className="flex items-start">
            <div className="flex items-center h-5">
              <input
                id="memory"
                name="memory"
                type="checkbox"
                className="focus:ring-[#6405d6] h-4 w-4 text-[#6405d6] border-[#2a1a3a] rounded"
                checked={settings.contextMemory}
                onChange={(e) => onChange('contextMemory', e.target.checked)}
              />
            </div>
            <div className="ml-3 text-sm">
              <label htmlFor="memory" className="font-medium text-gray-200">
                Habilitar memória de contexto
              </label>
              <p className="text-gray-400">
                Permite que a IA lembre do contexto das mensagens anteriores na conversa.
              </p>
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
            <h3 className="text-sm font-medium text-blue-200">Como obter uma chave de API</h3>
            <div className="mt-2 text-sm text-blue-100">
              <p>
                1. Acesse{' '}
                <a href="https://platform.openai.com/account/api-keys" target="_blank" rel="noopener noreferrer" className="underline hover:text-white">
                  OpenAI API Keys
                </a>
              </p>
              <p>2. Cate uma nova chave secreta</p>
              <p>3. Cole a chave no campo acima</p>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
};
