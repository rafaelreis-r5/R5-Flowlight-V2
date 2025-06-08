import React, { useState, useEffect, useRef } from 'react';
import ReactDOM from 'react-dom/client';
import { invoke } from '@tauri-apps/api/tauri';
import { Search, Bot, DollarSign, Heart, Apple, Palette, Calendar, X } from 'lucide-react';
import { motion, AnimatePresence } from 'framer-motion';
import './index.css';

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

function SearchWindow() {
  const [searchQuery, setSearchQuery] = useState('');
  const [selectedModule, setSelectedModule] = useState<string>('');
  const [searchResults, setSearchResults] = useState<SearchResult[]>([]);
  const [appResults, setAppResults] = useState<AppResult[]>([]);
  const [isAIMode, setIsAIMode] = useState(false);
  const [aiResponse, setAiResponse] = useState('');
  const [isLoading, setIsLoading] = useState(false);
  const [windowHeight, setWindowHeight] = useState(60);
  const inputRef = useRef<HTMLInputElement>(null);

  // Get selected module on mount
  useEffect(() => {
    const getSelectedModule = async () => {
      try {
        const module = await invoke<string>('get_selected_module');
        setSelectedModule(module);
      } catch (error) {
        console.error('Error getting selected module:', error);
      }
    };

    getSelectedModule();

    // Focus input when window opens
    if (inputRef.current) {
      inputRef.current.focus();
    }
  }, []);

  // Real-time search as user types
  useEffect(() => {
    const delaySearch = setTimeout(() => {
      if (searchQuery.trim() && !isAIMode) {
        handleSearch(searchQuery);
      } else if (!searchQuery.trim()) {
        setSearchResults([]);
        setAppResults([]);
        updateWindowHeight();
      }
    }, 300);

    return () => clearTimeout(delaySearch);
  }, [searchQuery, isAIMode]);

  // Update window height based on content
  const updateWindowHeight = async () => {
    let newHeight = 60; // Base height for search bar
    
    if (aiResponse) {
      newHeight += Math.min(200, aiResponse.length / 3); // AI response above
    }
    
    if (searchResults.length > 0 || appResults.length > 0) {
      newHeight += Math.min(300, (searchResults.length + appResults.length) * 60); // Results below
    }

    if (selectedModule && moduleFunctions[selectedModule]) {
      newHeight += 40; // Module functions
    }

    setWindowHeight(newHeight);
    
    try {
      await invoke('resize_search_window', { height: newHeight });
    } catch (error) {
      console.error('Error resizing window:', error);
    }
  };

  useEffect(() => {
    updateWindowHeight();
  }, [searchResults, appResults, aiResponse, selectedModule]);

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
      setAiResponse('Erro ao consultar IA. Tente novamente.');
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
      setSearchQuery(''); // Clear search query to show the function response
    } catch (error) {
      console.error('Module function error:', error);
      setAiResponse('Erro ao executar função do módulo. Tente novamente.');
    }
    setIsLoading(false);
  };

  // Handle keyboard shortcuts
  const handleKeyDown = (e: React.KeyboardEvent) => {
    if (e.key === 'Escape') {
      handleCloseWindow();
    } else if (e.key === 'Tab') {
      e.preventDefault();
      setIsAIMode(!isAIMode);
    }
  };

  // Handle global keyboard shortcuts
  useEffect(() => {
    const handleGlobalKeyDown = (e: KeyboardEvent) => {
      // Check for Cmd/Ctrl + number shortcuts
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

  const handleCloseWindow = async () => {
    try {
      await invoke('hide_search_window');
    } catch (error) {
      console.error('Error hiding search window:', error);
    }
  };

  const currentModuleFunctions = selectedModule ? moduleFunctions[selectedModule] || [] : [];

  return (
    <div className="w-full bg-transparent" style={{ height: `${windowHeight}px` }}>
      {/* AI Response (appears above search bar) */}
      <AnimatePresence>
        {aiResponse && (
          <motion.div
            className="glass-effect rounded-t-xl p-4 mb-1 relative"
            initial={{ opacity: 0, y: -20 }}
            animate={{ opacity: 1, y: 0 }}
            exit={{ opacity: 0, y: -20 }}
            transition={{ duration: 0.2 }}
          >
            <button
              onClick={() => setAiResponse('')}
              className="absolute top-2 right-2 text-gray-400 hover:text-white"
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

      {/* Search Bar */}
      <motion.div 
        className={`glass-effect p-3 ${aiResponse ? 'rounded-none' : 'rounded-t-xl'} ${
          (searchResults.length > 0 || appResults.length > 0 || currentModuleFunctions.length > 0) ? '' : 'rounded-b-xl'
        }`}
        initial={{ opacity: 0, scale: 0.95 }}
        animate={{ opacity: 1, scale: 1 }}
        transition={{ duration: 0.2 }}
      >
        <form onSubmit={handleSubmit} className="relative">
          <div className="flex items-center space-x-3">
            <Search className="w-4 h-4 text-primary-purple flex-shrink-0" />
            <input
              ref={inputRef}
              type="text"
              value={searchQuery}
              onChange={(e) => setSearchQuery(e.target.value)}
              onKeyDown={handleKeyDown}
              placeholder={isAIMode ? "Pergunte à IA..." : "Buscar arquivos e aplicativos..."}
              className="flex-1 bg-transparent outline-none text-sm placeholder-gray-400 min-w-0"
              autoFocus
            />
            <motion.button
              type="button"
              onClick={() => setIsAIMode(!isAIMode)}
              className={`p-1.5 rounded-lg transition-all flex-shrink-0 ${
                isAIMode ? 'bg-accent-red text-white' : 'bg-secondary-gray text-gray-300'
              }`}
              whileHover={{ scale: 1.05 }}
              whileTap={{ scale: 0.95 }}
            >
              <Bot className="w-4 h-4" />
            </motion.button>
          </div>
        </form>
      </motion.div>

      {/* Module Functions */}
      {currentModuleFunctions.length > 0 && (
        <motion.div
          className="glass-effect rounded-none px-3 py-2 border-t border-gray-600"
          initial={{ opacity: 0 }}
          animate={{ opacity: 1 }}
          transition={{ duration: 0.2 }}
        >
          <div className="flex space-x-2">
            {currentModuleFunctions.map((func) => (
              <button
                key={func.id}
                onClick={() => handleModuleFunctionClick(func.id)}
                className="flex items-center space-x-1 px-2 py-1 bg-secondary-gray rounded text-xs hover:bg-opacity-80 transition-all"
              >
                {func.icon}
                <span>{func.name}</span>
                <span className="text-primary-purple text-xs">{func.hotkey}</span>
              </button>
            ))}
          </div>
        </motion.div>
      )}

      {/* Search Results (appear below search bar) */}
      <AnimatePresence>
        {(searchResults.length > 0 || appResults.length > 0) && (
          <motion.div
            className="glass-effect rounded-b-xl p-2 border-t border-gray-600 max-h-60 overflow-y-auto"
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
                    className="flex items-center space-x-3 p-2 rounded-lg bg-secondary-gray hover:bg-opacity-80 transition-all cursor-pointer"
                    initial={{ opacity: 0, x: -10 }}
                    animate={{ opacity: 1, x: 0 }}
                    transition={{ duration: 0.1, delay: index * 0.02 }}
                    whileHover={{ scale: 1.01 }}
                  >
                    {result.icon ? (
                      <img src={result.icon} alt={result.name} className="w-5 h-5" />
                    ) : (
                      <div className="w-5 h-5 bg-primary-purple rounded flex items-center justify-center">
                        <span className="text-xs text-white">{result.name.charAt(0)}</span>
                      </div>
                    )}
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
                      <span className="text-xs text-white">{result.file_type.charAt(0).toUpperCase()}</span>
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
}

// Render the component
const container = document.getElementById('search-root');
if (container) {
  const root = ReactDOM.createRoot(container);
  root.render(<SearchWindow />);
}