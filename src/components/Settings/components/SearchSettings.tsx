import React, { useState } from 'react';
import { FiFolder, FiX, FiPlus } from 'react-icons/fi';
import { open } from '@tauri-apps/plugin-dialog';
import { ContentType } from '../../../types/content';
import { ContentTypeSettings } from './ContentTypeSettings';
import { AdvancedIndexingSettings } from './AdvancedIndexingSettings';

export type SearchSettingsType = {
  maxResults: number;
  searchDelay: number;
  fuzzySearch: boolean;
  includedPaths: string[];
  excludedPatterns: string[];
  contentTypes: ContentType[];
  advancedIndexing: {
    enabled: boolean;
    maxFileSize: number; // in MB
    indexFileTypes: string[];
    excludeHiddenFiles: boolean;
    indexFileMetadata: boolean;
    indexFileContent: boolean;
    contentTypesToIndex: {
      documents: boolean;
      images: boolean;
      audio: boolean;
      video: boolean;
      archives: boolean;
      code: boolean;
      spreadsheets: boolean;
      presentations: boolean;
      pdfs: boolean;
      emails: boolean;
    };
    indexSchedule: 'realtime' | 'hourly' | 'daily' | 'weekly' | 'monthly';
    lastIndexed: string | null;
  };
};

interface SearchSettingsProps {
  settings: SearchSettingsType;
  onChange: <K extends keyof SearchSettingsType>(
    key: K, 
    value: SearchSettingsType[K]
  ) => void;
}

export const SearchSettings: React.FC<SearchSettingsProps> = ({ settings, onChange }) => {
  const [newExclusion, setNewExclusion] = useState('');

  const addIncludedPath = async () => {
    try {
      // Usando a API do Tauri v2 para abrir o diálogo de seleção de pasta
      const selected = await open({
        directory: true,
        multiple: false,
      });
      
      if (selected && !settings.includedPaths.includes(selected as string)) {
        onChange('includedPaths', [...settings.includedPaths, selected as string]);
      }
    } catch (error) {
      console.error('Erro ao selecionar pasta:', error);
    }
  };

  const removeIncludedPath = (pathToRemove: string) => {
    onChange('includedPaths', settings.includedPaths.filter(p => p !== pathToRemove));
  };

  const addExclusion = () => {
    if (newExclusion.trim() && !settings.excludedPatterns.includes(newExclusion.trim())) {
      onChange('excludedPatterns', [...settings.excludedPatterns, newExclusion.trim()]);
      setNewExclusion('');
    }
  };

  const removeExclusion = (patternToRemove: string) => {
    onChange('excludedPatterns', settings.excludedPatterns.filter(p => p !== patternToRemove));
  };

  const getContentTypeCount = () => {
    return settings.contentTypes?.length || 0;
  };

  const getContentTypeSummary = () => {
    const count = getContentTypeCount();
    if (count === 3) return 'Todos os tipos ativos';
    if (count === 0) return 'Nenhum tipo selecionado';
    return `${count} ${count === 1 ? 'tipo' : 'tipos'} ativo${count > 1 ? 's' : ''}`;
  };

  const handleAdvancedIndexingChange = (key: string, value: any) => {
    onChange('advancedIndexing', {
      ...settings.advancedIndexing,
      [key]: value
    });
  };

  const handleAdvancedIndexingToggle = (key: string, value: any) => {
    onChange('advancedIndexing', {
      ...settings.advancedIndexing,
      [key]: value
    });
  };

  return (
    <div className="space-y-8">
      {/* Content Type Settings */}
      <div className="bg-gray-900/50 p-6 rounded-xl border border-gray-700/50">
        <div className="flex items-center justify-between mb-6">
          <h2 className="text-xl font-semibold text-white">Tipos de Conteúdo</h2>
          <span className="text-sm text-gray-400">
            {getContentTypeSummary()}
          </span>
        </div>
        <ContentTypeSettings
          enabledTypes={settings.contentTypes || []}
          onChange={(types) => onChange('contentTypes', types)}
        />
      </div>
      <div>
        <h3 className="text-lg font-semibold text-white mb-4">Configurações Gerais</h3>
        
        <div className="space-y-6">
          <div>
            <label htmlFor="maxResults" className="block text-sm font-medium text-gray-200 mb-1">
              Número máximo de resultados: {settings.maxResults}
            </label>
            <input
              id="maxResults"
              type="range"
              min="5"
              max="50"
              value={settings.maxResults}
              onChange={(e) => onChange('maxResults', parseInt(e.target.value))}
              className="w-full h-2 bg-gray-700 rounded-lg appearance-none cursor-pointer"
            />
            <div className="flex justify-between text-xs text-gray-400 mt-1">
              <span>5</span>
              <span>50</span>
            </div>
          </div>

          <div>
            <label htmlFor="searchDelay" className="block text-sm font-medium text-gray-200 mb-1">
              Delay de busca: {settings.searchDelay}ms
            </label>
            <input
              id="searchDelay"
              type="range"
              min="0"
              max="1000"
              step="50"
              value={settings.searchDelay}
              onChange={(e) => onChange('searchDelay', parseInt(e.target.value))}
              className="w-full h-2 bg-gray-700 rounded-lg appearance-none cursor-pointer"
            />
            <div className="flex justify-between text-xs text-gray-400 mt-1">
              <span>0ms</span>
              <span>1000ms</span>
            </div>
          </div>

          <div className="flex items-center justify-between">
            <div>
              <p className="text-sm font-medium text-gray-200">Busca difusa</p>
              <p className="text-xs text-gray-400">Permite encontrar itens com erros de digitação</p>
            </div>
            <label className="relative inline-flex items-center cursor-pointer">
              <input 
                type="checkbox" 
                className="sr-only peer" 
                checked={settings.fuzzySearch}
                onChange={(e) => onChange('fuzzySearch', e.target.checked)}
              />
              <div className="w-11 h-6 bg-gray-700 rounded-full peer peer-checked:after:translate-x-full after:content-[''] after:absolute after:top-0.5 after:left-[2px] after:bg-white after:border-gray-300 after:border after:rounded-full after:h-5 after:w-5 after:transition-all peer-checked:bg-gradient-to-r peer-checked:from-[#6405d6] peer-checked:to-[#f1011d]"></div>
            </label>
          </div>
        </div>
      </div>

      <div>
        <h3 className="text-lg font-semibold text-white mb-4">Pastas Incluídas</h3>
        <div className="space-y-2">
          {settings.includedPaths.map((path) => (
            <div key={path} className="flex items-center justify-between bg-[#1a0a2e] p-3 rounded-md">
              <div className="flex items-center">
                <FiFolder className="text-[#6405d6] mr-2" />
                <span className="text-gray-200 text-sm truncate max-w-xs">{path}</span>
              </div>
              <button
                onClick={() => removeIncludedPath(path)}
                className="text-gray-400 hover:text-white p-1"
              >
                <FiX size={18} />
              </button>
            </div>
          ))}
          <button
            onClick={addIncludedPath}
            className="w-full flex items-center justify-center space-x-2 bg-[#1a0a2e] hover:bg-[#2a1a3a] text-[#6405d6] p-3 rounded-md transition-colors"
          >
            <FiPlus />
            <span>Adicionar Pasta</span>
          </button>
        </div>
      </div>

      <div>
        <h3 className="text-lg font-semibold text-white mb-4">Exclusões</h3>
        <div className="space-y-2">
          {settings.excludedPatterns.map((pattern) => (
            <div key={pattern} className="flex items-center justify-between bg-[#1a0a2e] p-3 rounded-md">
              <span className="text-gray-200 text-sm">{pattern}</span>
              <button
                onClick={() => removeExclusion(pattern)}
                className="text-gray-400 hover:text-white p-1"
              >
                <FiX size={18} />
              </button>
            </div>
          ))}
          <div className="flex space-x-2">
            <input
              type="text"
              value={newExclusion}
              onChange={(e) => setNewExclusion(e.target.value)}
              placeholder="Ex: node_modules, *.tmp"
              className="flex-1 bg-[#1a0a2e] border border-[#2a1a3a] rounded-md px-3 py-2 text-white placeholder-gray-500 focus:outline-none focus:ring-2 focus:ring-[#6405d6] focus:border-transparent"
              onKeyDown={(e) => e.key === 'Enter' && addExclusion()}
            />
            <button
              onClick={addExclusion}
              className="bg-[#6405d6] hover:bg-[#7a1ae6] text-white px-4 py-2 rounded-md transition-colors"
            >
              Adicionar
            </button>
          </div>
          <p className="text-xs text-gray-400 mt-2">
            Use * para curinga. Ex: *.tmp exclui todos os arquivos .tmp
          </p>
        </div>
      </div>

      {/* Advanced Indexing Settings */}
      <AdvancedIndexingSettings
        enabled={settings.advancedIndexing.enabled}
        maxFileSize={settings.advancedIndexing.maxFileSize}
        excludeHiddenFiles={settings.advancedIndexing.excludeHiddenFiles}
        indexFileMetadata={settings.advancedIndexing.indexFileMetadata}
        indexFileContent={settings.advancedIndexing.indexFileContent}
        contentTypesToIndex={settings.advancedIndexing.contentTypesToIndex}
        indexSchedule={settings.advancedIndexing.indexSchedule}
        lastIndexed={settings.advancedIndexing.lastIndexed}
        onToggle={handleAdvancedIndexingToggle}
        onChange={handleAdvancedIndexingChange}
      />
    </div>
  );
};
