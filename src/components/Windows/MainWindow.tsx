import * as React from 'react';
import { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import { DollarSign, Heart, Apple, Palette, Calendar } from 'lucide-react';
import r5Logo from '../../assets/icons/app-icon.png';
import { motion, AnimatePresence } from 'framer-motion';
import { useAuth } from '../../contexts/AuthContext';
import { Login } from '../Auth/Login';
import { SettingsWindow } from '../Settings/SettingsWindow';
import { toast } from 'react-hot-toast';
import { showErrorToast } from '../../config/toastConfig';
import { CustomToaster } from '../Common/CustomToast';
import { Sidebar } from '../Common/Sidebar';
import { StatusFooter } from '../Common/StatusFooter';
import { Header } from '../Common/Header';

// Types
interface ModuleType {
  id: string;
  name: string;
  icon: React.ReactElement;
  description: string;
  hotkey: string;
  color: {
    light: string;
    dark: string;
  };
}

const modules: ModuleType[] = [
  {
    id: 'finance',
    name: 'Finance',
    icon: <DollarSign className="w-8 h-8" />,
    description: 'Análise financeira e investimentos',
    hotkey: '⌘+1',
    color: {
      light: '#4f46e5',
      dark: '#3730a3'
    }
  },
  {
    id: 'health',
    name: 'Health',
    icon: <Heart className="w-8 h-8" />,
    description: 'Medicina clínica e do trabalho',
    hotkey: '⌘+2',
    color: {
      light: '#ec4899',
      dark: '#9d174d'
    }
  },
  {
    id: 'nutrition',
    name: 'Nutrition',
    icon: <Apple className="w-8 h-8" />,
    description: 'Planejamento nutricional',
    hotkey: '⌘+3',
    color: {
      light: '#10b981',
      dark: '#047857'
    }
  },
  {
    id: 'creator',
    name: 'Creator',
    icon: <Palette className="w-8 h-8" />,
    description: 'Criação de conteúdo',
    hotkey: '⌘+4',
    color: {
      light: '#8b5cf6',
      dark: '#6d28d9'
    }
  },
  {
    id: 'daily',
    name: 'Daily',
    icon: <Calendar className="w-8 h-8" />,
    description: 'Utilitários do dia a dia',
    hotkey: '⌘+5',
    color: {
      light: '#f59e0b',
      dark: '#b45309'
    }
  }
];

const MainWindow: React.FC = () => {
  const { isAuthenticated, isLoading: isAuthLoading } = useAuth();
  const [selectedModule, setSelectedModule] = useState<string | null>(null);
  const [isSettingsOpen, setIsSettingsOpen] = useState(false);
  const [isVerifyingAuth, setIsVerifyingAuth] = useState(true);

  // Load selected module on mount
  useEffect(() => {
    const loadSelectedModule = async () => {
      try {
        const module = await invoke<string>('get_selected_module');
        if (module !== 'general') {
          setSelectedModule(module);
        }
      } catch (error) {
        console.error('Error loading selected module:', error);
      }
    };

    loadSelectedModule();
  }, []);

  // Efeito para verificar o estado de autenticação
  useEffect(() => {
    if (!isAuthLoading) {
      setIsVerifyingAuth(false);
    }
  }, [isAuthLoading]);

  // Efeito para lidar com mudanças no estado de autenticação
  useEffect(() => {
    const handleAuthChange = () => {
      setSelectedModule(prev => prev);
    };

    window.addEventListener('authStateChanged', handleAuthChange);

    const handleStorageChange = (e: StorageEvent) => {
      if (e.key === 'is_admin' || e.key === 'user') {
        window.dispatchEvent(new Event('authStateChanged'));
      }
    };

    window.addEventListener('storage', handleStorageChange);

    return () => {
      window.removeEventListener('authStateChanged', handleAuthChange);
      window.removeEventListener('storage', handleStorageChange);
    };
  }, [isAuthenticated, isAuthLoading]);

  // Efeito para configurar o listener da bandeja do sistema e avisos de módulo
  useEffect(() => {
    const setupTrayListener = async () => {
      try {
        const unlisten = await listen('tray_show', () => {
          toast('O aplicativo continua em execução na bandeja do sistema.', {
            icon: 'ℹ️',
            duration: 3000,
          });
        });
        return unlisten;
      } catch (error) {
        console.error('Error setting up tray listener:', error);
      }
    };

    const setupModuleWarningListener = async () => {
      try {
        const unlisten = await listen<string>('show_module_warning', (event) => {
          showErrorToast(event.payload);
        });
        return unlisten;
      } catch (error) {
        console.error('Error setting up module warning listener:', error);
      }
    };

    // LISTENER REMOVIDO - Global shortcuts agora gerenciados pelo real-daemon
    // que se comunica diretamente com o real-overlay
    //
    // const setupGlobalShortcutListener = async () => {
    //   try {
    //     const unlisten = await listen('trigger_global_shortcut', async () => {
    //       console.log('🔥 Global shortcut event received from daemon');
    //       try {
    //         await invoke('toggle_search_launcher');
    //         console.log('✅ Search launcher toggled via daemon event');
    //       } catch (error) {
    //         console.error('❌ Error toggling search launcher:', error);
    //       }
    //     });
    //     return unlisten;
    //   } catch (error) {
    //     console.error('Error setting up global shortcut listener:', error);
    //   }
    // };

    const cleanup = setupTrayListener();
    const cleanup2 = setupModuleWarningListener();
    // CLEANUP REMOVIDO - setupGlobalShortcutListener não existe mais

    return () => {
      cleanup.then(unlisten => {
        if (unlisten && typeof unlisten === 'function') {
          unlisten();
        }
      }).catch(console.error);

      cleanup2.then(unlisten => {
        if (unlisten && typeof unlisten === 'function') {
          unlisten();
        }
      }).catch(console.error);
    };
  }, []);

  // Tela de carregamento
  if (isVerifyingAuth || isAuthLoading) {
    return (
      <div className="min-h-screen flex items-center justify-center bg-gradient-to-br from-[#0f0a1a] to-[#1a0a2e] text-white">
        <div className="animate-pulse text-center">
          <img
            src={r5Logo}
            alt="R5 Flowlight Logo"
            className="w-16 h-16 mx-auto mb-4 object-contain"
          />
          <p className="text-gray-400">Carregando...</p>
        </div>
      </div>
    );
  }

  // Tela de login
  if (!isAuthenticated) {
    return (
      <Login onLoginSuccess={() => {
        // O estado de autenticação será atualizado pelo AuthContext
      }} />
    );
  }

  const handleModuleSelect = async (moduleId: string) => {
    setSelectedModule(moduleId);
    try {
      await invoke('set_selected_module', { moduleId });
      toast.success(`Módulo ${moduleId} selecionado! Use ⌘+Space para abrir o launcher.`);
    } catch (error) {
      console.error('Module selection error:', error);
      toast.error('Erro ao selecionar módulo');
    }
  };

  return (
    <div className="min-h-screen bg-gradient-to-b from-[#0f0a1a] to-[#1a0a2e] text-white flex flex-col">
      <Header onSettingsClick={() => setIsSettingsOpen(true)} />

      {/* Conteúdo principal */}
      <div className="flex-1 flex overflow-hidden">
        <Sidebar />

        <main className="flex-1 overflow-auto p-6">
          <AnimatePresence>
            {isSettingsOpen && (
              <SettingsWindow
                isOpen={isSettingsOpen}
                onClose={() => setIsSettingsOpen(false)}
              />
            )}
          </AnimatePresence>

          <div className="max-w-6xl mx-auto">
            <motion.div
              className="text-center mb-12"
              initial={{ opacity: 0, y: -20 }}
              animate={{ opacity: 1, y: 0 }}
              transition={{ duration: 0.5 }}
            >
              <h1 className="text-4xl font-bold bg-gradient-to-r from-primary-purple to-accent-red bg-clip-text text-transparent mb-4">
                Selecione um Módulo
              </h1>
              <p className="text-gray-400">Escolha um dos módulos abaixo e use ⌘+Space para abrir o launcher</p>
            </motion.div>

            <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
              {modules.map((module, index) => (
                <motion.div
                  key={module.id}
                  onClick={() => handleModuleSelect(module.id)}
                  className={`relative overflow-hidden rounded-2xl p-6 cursor-pointer transition-all duration-300 ${
                    selectedModule === module.id
                      ? 'ring-2 ring-offset-2 ring-offset-gray-900'
                      : 'hover:bg-white/5'
                  }`}
                  style={{
                    background: selectedModule === module.id
                      ? 'linear-gradient(135deg, #6405d6, #f1011d)'
                      : `linear-gradient(135deg, ${module.color.dark}20 0%, ${module.color.light}10 100%)`,
                    border: selectedModule === module.id
                      ? '2px solid #6405d6'
                      : '1px solid rgba(255, 255, 255, 0.05)',
                  }}
                  initial={{ opacity: 0, y: 20 }}
                  animate={{ opacity: 1, y: 0 }}
                  transition={{ duration: 0.3, delay: index * 0.1 }}
                  whileHover={{
                    y: -5,
                    boxShadow: '0 10px 25px -5px rgba(0, 0, 0, 0.1)'
                  }}
                >
                  <div className="flex items-center mb-4">
                    <div
                      className="p-3 rounded-lg mr-4"
                      style={{
                        background: selectedModule === module.id
                          ? 'rgba(255, 255, 255, 0.2)'
                          : module.color.dark
                      }}
                    >
                      {React.cloneElement(module.icon, {
                        className: 'w-6 h-6 text-white'
                      })}
                    </div>
                    <h3 className={`text-xl font-semibold ${
                      selectedModule === module.id ? 'text-white' : 'text-white'
                    }`}>{module.name}</h3>
                  </div>
                  <p className={`mb-4 ${
                    selectedModule === module.id ? 'text-white/90' : 'text-gray-300'
                  }`}>{module.description}</p>
                  <div className="flex justify-between items-center">
                    <div className={`text-xs ${
                      selectedModule === module.id ? 'text-white/70' : 'text-gray-500'
                    }`}>{module.hotkey}</div>
                    {selectedModule === module.id && (
                      <div className="text-xs text-white bg-white/20 px-2 py-1 rounded-full">
                        ✓ Selecionado
                      </div>
                    )}
                  </div>
                </motion.div>
              ))}
            </div>
          </div>
        </main>
      </div>

      {/* Rodapé */}
      <StatusFooter />

      {/* Notificações */}
      <CustomToaster />
    </div>
  );
};

export default MainWindow;
