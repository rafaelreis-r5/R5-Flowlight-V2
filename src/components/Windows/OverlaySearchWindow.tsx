import React, { useState, useEffect, useRef } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { showErrorToast } from '../../config/toastConfig';
import { Search, Bot, DollarSign, Heart, Apple, Palette, Calendar, X } from 'lucide-react';
import { motion, AnimatePresence } from 'framer-motion';
import '../../index.css';

// Types
interface SearchResult {
  title: string;
  path: string;
  content_preview: string;
  score: number;
  file_type: string;
  size: number;
  modified: string;
  icon?: string;
}

interface AppResult {
  name: string;
  path: string;
  icon: string;
}

interface ModuleFunction {
  id: string;
  name: string;
  icon: React.ReactNode;
  hotkey: string;
}

const moduleFunctions: Record<string, ModuleFunction[]> = {
  finance: [
    { id: 'portfolio', name: 'Portfólio', icon: <DollarSign className="w-4 h-4" />, hotkey: '⌘+1' },
    { id: 'analysis', name: 'Análise', icon: <DollarSign className="w-4 h-4" />, hotkey: '⌘+2' },
  ],
  health: [
    { id: 'symptoms', name: 'Sintomas', icon: <Heart className="w-4 h-4" />, hotkey: '⌘+1' },
    { id: 'diagnosis', name: 'Diagnóstico', icon: <Heart className="w-4 h-4" />, hotkey: '⌘+2' },
  ],
  nutrition: [
    { id: 'meal-plan', name: 'Cardápio', icon: <Apple className="w-4 h-4" />, hotkey: '⌘+1' },
    { id: 'calories', name: 'Calorias', icon: <Apple className="w-4 h-4" />, hotkey: '⌘+2' },
  ],
  creator: [
    { id: 'content', name: 'Conteúdo', icon: <Palette className="w-4 h-4" />, hotkey: '⌘+1' },
    { id: 'design', name: 'Design', icon: <Palette className="w-4 h-4" />, hotkey: '⌘+2' },
  ],
  daily: [
    { id: 'tasks', name: 'Tarefas', icon: <Calendar className="w-4 h-4" />, hotkey: '⌘+1' },
    { id: 'schedule', name: 'Agenda', icon: <Calendar className="w-4 h-4" />, hotkey: '⌘+2' },
  ]
};

const OverlaySearchWindow: React.FC = () => {
  const [searchQuery, setSearchQuery] = useState('');
  const [selectedModule, setSelectedModule] = useState<string>('daily');
  const [searchResults, setSearchResults] = useState<SearchResult[]>([]);
  const [appResults, setAppResults] = useState<AppResult[]>([]);
  const [isAIMode, setIsAIMode] = useState(false);
  const [aiResponse, setAiResponse] = useState<string | null>(null);
  const [isLoading, setIsLoading] = useState(false);
  const inputRef = useRef<HTMLInputElement>(null);

  // Get selected module on mount and setup focus management
  useEffect(() => {
    const getSelectedModule = async () => {
      try {
        const module = await invoke<string>('get_selected_module');
        setSelectedModule(module);
      } catch (error) {
        console.error('Error getting selected module:', error);
        setSelectedModule('daily'); // fallback
      }
    };

    getSelectedModule();

    // Focus management
    const focusInput = () => {
      if (inputRef.current) {
        inputRef.current.focus();
        inputRef.current.select();
      }
    };

    focusInput();
    const timeoutId = setTimeout(focusInput, 50);

    return () => clearTimeout(timeoutId);
  }, []);

  // Real-time search as user types
  useEffect(() => {
    const delaySearch = setTimeout(() => {
      if (searchQuery.trim() && !isAIMode) {
        handleSearch(searchQuery);
      } else if (!searchQuery.trim()) {
        setSearchResults([]);
        setAppResults([]);
      }
    }, 300);

    return () => clearTimeout(delaySearch);
  }, [searchQuery, isAIMode]);

  // Search functionality
  const handleSearch = async (query: string) => {
    if (!query.trim()) {
      setSearchResults([]);
      setAppResults([]);
      return;
    }

    setIsLoading(true);
    try {
      // Search files and apps simultaneously
      const [fileResults, appResultsData] = await Promise.all([
        invoke<SearchResult[]>('search_files', { query }),
        invoke<AppResult[]>('search_apps', { query })
      ]);

      setSearchResults(fileResults);
      setAppResults(appResultsData);
    } catch (error) {
      console.error('Search error:', error);
      showErrorToast('Erro ao buscar arquivos e aplicativos. Tente novamente.');
      setSearchResults([]);
      setAppResults([]);
    }
    setIsLoading(false);
  };

  // AI functionality
  const handleAIQuery = async (prompt: string) => {
    if (!prompt.trim()) return;

    setIsLoading(true);
    try {
      const context = selectedModule || 'general';
      const response = await invoke<string>('ai_query', { prompt, context });
      setAiResponse(response);
    } catch (error) {
      console.error('AI error:', error);
      const errMsg = error instanceof Error ? error.message : String(error);
      showErrorToast(errMsg);
      setAiResponse(errMsg);
    }
    setIsLoading(false);
  };

  // Handle form submit
  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    if (isAIMode) {
      handleAIQuery(searchQuery);
    } else {
      handleSearch(searchQuery);
    }
  };

  // Handle module function click
  const handleModuleFunctionClick = async (functionId: string) => {
    try {
      setIsLoading(true);
      const response = await invoke<string>('execute_module_function', {
        moduleId: selectedModule,
        functionId
      });
      setAiResponse(response);
      setSearchQuery('');
    } catch (error) {
      console.error('Module function error:', error);
      showErrorToast('Erro ao executar função do módulo. Tente novamente.');
      setAiResponse('Erro ao executar função do módulo. Tente novamente.');
    }
    setIsLoading(false);
  };

  // Handle keyboard shortcuts
  const handleKeyDown = (e: React.KeyboardEvent) => {
    if (e.key === 'Escape') {
      if (searchQuery.trim()) {
        setSearchQuery('');
        setSearchResults([]);
        setAppResults([]);
        setAiResponse(null);
      } else {
        // Close overlay via invoke
        invoke('toggle_overlay').catch(console.error);
      }
    } else if (e.key === 'Tab') {
      e.preventDefault();
      setIsAIMode(!isAIMode);
    }
  };

  // Handle global keyboard shortcuts
  useEffect(() => {
    const handleGlobalKeyDown = (e: KeyboardEvent) => {
      if ((e.metaKey || e.ctrlKey) && e.key >= '1' && e.key <= '5' && selectedModule) {
        e.preventDefault();

        const functions = moduleFunctions[selectedModule];
        if (functions) {
          const functionIndex = parseInt(e.key) - 1;
          if (functions[functionIndex]) {
            handleModuleFunctionClick(functions[functionIndex].id);
          }
        }
      }
    };

    document.addEventListener('keydown', handleGlobalKeyDown);
    return () => document.removeEventListener('keydown', handleGlobalKeyDown);
  }, [selectedModule]);

  const currentModuleFunctions = selectedModule ? moduleFunctions[selectedModule] || [] : [];

  return (
    <div 
      data-search-container
      className="search-container" 
      style={{
        position: 'fixed',
        top: '50px',
        left: '50%',
        transform: 'translateX(-50%)',
        display: 'flex',
        flexDirection: 'column',
        gap: '0px',
        width: '750px',
        zIndex: 10000,
        pointerEvents: 'auto' // Re-enable pointer events for search container
      }}>
      {/* Search Bar */}
      <motion.div
        className="search-window-glass p-5"
        style={{ 
          cursor: 'move', 
          WebkitAppRegion: 'drag',
          borderTopLeftRadius: '12px',
          borderTopRightRadius: '12px',
          borderBottomLeftRadius: currentModuleFunctions.length > 0 || searchResults.length > 0 || appResults.length > 0 || aiResponse ? '0px' : '12px',
          borderBottomRightRadius: currentModuleFunctions.length > 0 || searchResults.length > 0 || appResults.length > 0 || aiResponse ? '0px' : '12px'
        }}
        initial={{ opacity: 0, scale: 0.95 }}
        animate={{ opacity: 1, scale: 1 }}
        transition={{ duration: 0.2 }}
      >
          <form onSubmit={handleSubmit} className="relative">
            <div className="flex items-center space-x-4">
              <Search className="w-5 h-5 text-primary-purple flex-shrink-0" />
              <input
                ref={inputRef}
                type="text"
                value={searchQuery}
                onChange={(e) => setSearchQuery(e.target.value)}
                onKeyDown={handleKeyDown}
                placeholder={isAIMode ? "Pergunte à IA..." : "Buscar arquivos e aplicativos..."}
                className="flex-1 bg-transparent outline-none text-base placeholder-gray-400 min-w-0"
                style={{ WebkitAppRegion: 'no-drag', cursor: 'text' }}
                autoFocus
              />
              <motion.button
                type="button"
                onClick={() => { setIsAIMode(true); handleAIQuery(searchQuery); }}
                className="p-2 rounded-lg transition-all flex-shrink-0 text-white"
                style={{
                  background: isAIMode 
                    ? 'linear-gradient(135deg, #6405d6, #f1011d)' 
                    : 'rgba(100, 100, 100, 0.5)',
                  WebkitAppRegion: 'no-drag',
                  cursor: 'pointer'
                }}
                whileHover={{ scale: 1.05 }}
                whileTap={{ scale: 0.95 }}
              >
                <Bot className="w-5 h-5" />
              </motion.button>
            </div>
          </form>
      </motion.div>

      {/* Module Functions */}
      {currentModuleFunctions.length > 0 && (
        <motion.div
          className="search-window-glass px-4 py-3"
          style={{ 
            WebkitAppRegion: 'no-drag',
            borderRadius: '0px',
            borderBottomLeftRadius: searchResults.length > 0 || appResults.length > 0 || aiResponse ? '0px' : '12px',
            borderBottomRightRadius: searchResults.length > 0 || appResults.length > 0 || aiResponse ? '0px' : '12px'
          }}
          initial={{ opacity: 0 }}
          animate={{ opacity: 1 }}
          transition={{ duration: 0.2 }}
        >
            <div className="flex space-x-2">
              {currentModuleFunctions.map((func) => (
                <button
                  key={func.id}
                  onClick={() => handleModuleFunctionClick(func.id)}
                  className="flex items-center space-x-1 px-2 py-1 rounded text-xs transition-all text-white hover:opacity-80"
                  style={{ background: 'linear-gradient(135deg, #6405d6, #f1011d)' }}
                >
                  {func.icon}
                  <span>{func.name}</span>
                  <span className="text-white/70 text-xs">{func.hotkey}</span>
                </button>
              ))}
            </div>
        </motion.div>
      )}

      {/* AI Response */}
      <AnimatePresence>
        {aiResponse !== null && (
          <motion.div
            className="search-window-glass p-5"
            style={{ 
              WebkitAppRegion: 'no-drag',
              borderRadius: '0px',
              borderBottomLeftRadius: '12px',
              borderBottomRightRadius: '12px'
            }}
            initial={{ opacity: 0, y: 20 }}
            animate={{ opacity: 1, y: 0 }}
            exit={{ opacity: 0, y: 20 }}
            transition={{ duration: 0.2 }}
          >
              <button
                onClick={() => setAiResponse(null)}
                className="absolute top-2 right-2 text-gray-400 hover:text-white z-10"
              >
                <X className="w-4 h-4" />
              </button>
              <div className="prose prose-invert max-w-none">
                <div className="whitespace-pre-wrap text-sm leading-relaxed pr-6">
                  {aiResponse}
                </div>
              </div>
          </motion.div>
        )}
      </AnimatePresence>

      {/* Search Results */}
      <AnimatePresence>
        {(searchResults.length > 0 || appResults.length > 0) && (
          <motion.div
            className="search-window-glass p-4 max-h-96 overflow-y-auto"
            style={{ 
              WebkitAppRegion: 'no-drag',
              borderRadius: '0px',
              borderBottomLeftRadius: '12px',
              borderBottomRightRadius: '12px'
            }}
            initial={{ opacity: 0, y: 10 }}
            animate={{ opacity: 1, y: 0 }}
            exit={{ opacity: 0, y: 10 }}
            transition={{ duration: 0.2 }}
          >
              {isLoading ? (
                <div className="flex items-center justify-center h-16">
                  <div className="animate-spin rounded-full h-4 w-4 border-b-2 border-primary-purple"></div>
                </div>
              ) : (
                <div className="space-y-1">
                  {/* App Results */}
                  {appResults.map((result, index) => (
                    <motion.div
                      key={`app-${index}`}
                      className="flex items-center space-x-3 p-2 rounded-lg bg-secondary-gray hover:bg-opacity-20 transition-all cursor-pointer"
                      initial={{ opacity: 0, x: -10 }}
                      animate={{ opacity: 1, x: 0 }}
                      transition={{ duration: 0.1, delay: index * 0.02 }}
                      whileHover={{ scale: 1.01 }}
                    >
                      <div className="w-5 h-5 bg-primary-purple rounded flex items-center justify-center">
                        <span className="text-xs text-white">{result.icon}</span>
                      </div>
                      <div className="flex-1 min-w-0">
                        <div className="font-medium text-sm truncate">{result.name}</div>
                        <div className="text-xs text-gray-400 truncate">{result.path}</div>
                      </div>
                    </motion.div>
                  ))}

                  {/* File Results */}
                  {searchResults.map((result, index) => (
                    <motion.div
                      key={`file-${index}`}
                      className="flex items-center space-x-3 p-2 rounded-lg bg-secondary-gray hover:bg-opacity-80 transition-all cursor-pointer"
                      initial={{ opacity: 0, x: -10 }}
                      animate={{ opacity: 1, x: 0 }}
                      transition={{ duration: 0.1, delay: (appResults.length + index) * 0.02 }}
                      whileHover={{ scale: 1.01 }}
                    >
                      <div className="w-5 h-5 bg-gray-600 rounded flex items-center justify-center flex-shrink-0">
                        <span className="text-xs text-white">{result.icon || '📄'}</span>
                      </div>
                      <div className="flex-1 min-w-0">
                        <div className="font-medium text-sm truncate">{result.title}</div>
                        <div className="text-xs text-gray-400 truncate">{result.path}</div>
                      </div>
                    </motion.div>
                  ))}
                </div>
              )}
            </motion.div>
          )}
        </AnimatePresence>
    </div>
  );
};

export default OverlaySearchWindow;