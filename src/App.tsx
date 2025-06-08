import React, { useState } from 'react';
import { invoke } from '@tauri-apps/api/tauri';
import { DollarSign, Heart, Apple, Palette, Calendar } from 'lucide-react';
import { motion } from 'framer-motion';

// Types
interface ModuleType {
  id: string;
  name: string;
  icon: React.ReactNode;
  description: string;
  hotkey: string;
}

const modules: ModuleType[] = [
  {
    id: 'finance',
    name: 'Finance',
    icon: <DollarSign className="w-8 h-8" />,
    description: 'Análise financeira e investimentos',
    hotkey: '⌘+1'
  },
  {
    id: 'health',
    name: 'Health',
    icon: <Heart className="w-8 h-8" />,
    description: 'Medicina clínica e do trabalho',
    hotkey: '⌘+2'
  },
  {
    id: 'nutrition',
    name: 'Nutrition',
    icon: <Apple className="w-8 h-8" />,
    description: 'Planejamento nutricional',
    hotkey: '⌘+3'
  },
  {
    id: 'creator',
    name: 'Creator',
    icon: <Palette className="w-8 h-8" />,
    description: 'Criação de conteúdo',
    hotkey: '⌘+4'
  },
  {
    id: 'daily',
    name: 'Daily',
    icon: <Calendar className="w-8 h-8" />,
    description: 'Utilitários do dia a dia',
    hotkey: '⌘+5'
  }
];

function App() {
  const [selectedModule, setSelectedModule] = useState<string | null>(null);

  // Module selection and window management
  const handleModuleSelect = async (moduleId: string) => {
    setSelectedModule(moduleId);
    try {
      // Save selected module
      await invoke('set_selected_module', { moduleId });
      
      // Hide main window and setup search window
      await invoke('hide_main_window');
      await invoke('setup_search_window', { moduleId });
      
      // TEMPORARY: Show search window automatically for testing
      // (Em produção, isso seria feito pelo atalho global Cmd+Space)
      setTimeout(async () => {
        try {
          await invoke('show_search_window');
          console.log('Search window shown for testing');
        } catch (error) {
          console.error('Error showing search window:', error);
        }
      }, 500);
      
      console.log('Module selected:', moduleId);
    } catch (error) {
      console.error('Module selection error:', error);
    }
  };

  return (
    <div className="w-full h-full bg-primary-dark text-text-light overflow-hidden">
      <div className="flex flex-col h-full p-6 space-y-6">
        
        {/* Header */}
        <motion.div 
          className="text-center space-y-2"
          initial={{ opacity: 0, y: -20 }}
          animate={{ opacity: 1, y: 0 }}
          transition={{ duration: 0.5 }}
        >
          <h1 className="text-3xl font-bold text-primary-purple">R5 Flowlight</h1>
          <p className="text-gray-400">Escolha seu nicho especializado</p>
        </motion.div>

        {/* Module Selector */}
        <motion.div 
          className="glass-effect rounded-xl p-6 flex-1"
          initial={{ opacity: 0, y: 20 }}
          animate={{ opacity: 1, y: 0 }}
          transition={{ duration: 0.5, delay: 0.2 }}
        >
          <div className="grid grid-cols-3 gap-4 h-full">
            {modules.slice(0, 3).map((module, index) => (
              <motion.button
                key={module.id}
                onClick={() => handleModuleSelect(module.id)}
                className={`p-6 rounded-xl text-center transition-all h-full flex flex-col justify-center ${
                  selectedModule === module.id
                    ? 'bg-gradient-primary text-white'
                    : 'bg-secondary-gray hover:bg-opacity-80 hover:scale-105'
                }`}
                initial={{ opacity: 0, y: 20 }}
                animate={{ opacity: 1, y: 0 }}
                transition={{ duration: 0.3, delay: 0.3 + index * 0.1 }}
                whileHover={{ scale: 1.05 }}
                whileTap={{ scale: 0.95 }}
              >
                <div className="flex flex-col items-center space-y-4">
                  {module.icon}
                  <div>
                    <div className="text-lg font-semibold">{module.name}</div>
                    <div className="text-sm text-gray-400 mt-1">{module.description}</div>
                    <div className="text-xs text-primary-purple mt-2 font-mono">{module.hotkey}</div>
                  </div>
                </div>
              </motion.button>
            ))}
          </div>
          
          <div className="grid grid-cols-2 gap-4 mt-4">
            {modules.slice(3).map((module, index) => (
              <motion.button
                key={module.id}
                onClick={() => handleModuleSelect(module.id)}
                className={`p-6 rounded-xl text-center transition-all flex flex-col justify-center ${
                  selectedModule === module.id
                    ? 'bg-gradient-primary text-white'
                    : 'bg-secondary-gray hover:bg-opacity-80 hover:scale-105'
                }`}
                initial={{ opacity: 0, y: 20 }}
                animate={{ opacity: 1, y: 0 }}
                transition={{ duration: 0.3, delay: 0.6 + index * 0.1 }}
                whileHover={{ scale: 1.05 }}
                whileTap={{ scale: 0.95 }}
              >
                <div className="flex flex-col items-center space-y-4">
                  {module.icon}
                  <div>
                    <div className="text-lg font-semibold">{module.name}</div>
                    <div className="text-sm text-gray-400 mt-1">{module.description}</div>
                    <div className="text-xs text-primary-purple mt-2 font-mono">{module.hotkey}</div>
                  </div>
                </div>
              </motion.button>
            ))}
          </div>
        </motion.div>

        {/* Footer */}
        <motion.div
          className="text-center text-xs text-gray-500"
          initial={{ opacity: 0 }}
          animate={{ opacity: 1 }}
          transition={{ duration: 0.5, delay: 0.8 }}
        >
          Selecione um nicho para começar • Após a seleção use Cmd+Space para buscar
        </motion.div>
      </div>
    </div>
  );
}

export default App;