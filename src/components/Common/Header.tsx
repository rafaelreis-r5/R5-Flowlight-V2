import React, { useState } from 'react';
import { useAuth } from '../../contexts/AuthContext';
// Navegação personalizada sem depender do React Router
import r5Logo from '../../assets/icons/app-icon.png';
import { LogOut, Settings, Loader2, Check, X } from 'lucide-react';
import { motion, AnimatePresence } from 'framer-motion';
import { toast } from 'react-hot-toast';

interface HeaderProps {
  onSettingsClick: () => void;
}

export const Header: React.FC<HeaderProps> = ({ onSettingsClick }) => {
  const { user, logout, isLoading } = useAuth();
  const [isLoggingOut, setIsLoggingOut] = useState(false);
  const [showLogoutConfirm, setShowLogoutConfirm] = useState(false);
  const handleLogout = async () => {
    try {
      setIsLoggingOut(true);
      await logout();
      toast.success('Você saiu da sua conta com sucesso');
      // Recarrega a página para voltar para a tela de login
      window.location.reload();
    } catch (error) {
      console.error('Erro ao fazer logout:', error);
      toast.error('Ocorreu um erro ao sair da conta');
    } finally {
      setIsLoggingOut(false);
      setShowLogoutConfirm(false);
    }
  };
  
  // Extrai o primeiro nome do usuário ou usa o username como fallback
  const userName = user?.name?.split(' ')[0] || user?.username || '';
  
  return (
    <header 
      className="h-16 bg-black/30 backdrop-blur-md border-b border-white/10 px-4 sm:px-6 flex items-center justify-between sticky top-0 z-50"
      aria-label="Cabeçalho da aplicação"
    >
      {/* Logo e Nome do App */}
      <motion.div 
        className="flex items-center cursor-pointer"
        onClick={() => window.location.href = '/'}
        whileHover={{ scale: 1.02 }}
        whileTap={{ scale: 0.98 }}
        role="button"
        aria-label="Ir para a página inicial"
        tabIndex={0}
        onKeyDown={(e) => e.key === 'Enter' && (window.location.href = '/')}
      >
        <img 
          src={r5Logo} 
          alt="" 
          className="w-8 h-8 mr-3"
          aria-hidden="true"
        />
        <h1 className="text-xl font-bold bg-gradient-to-r from-primary-purple to-accent-red bg-clip-text text-transparent">
          R5 Flowlight
        </h1>
      </motion.div>
      
      {/* Mensagem de Boas-vindas */}
      <div className="flex-1 flex justify-center px-2">
        <AnimatePresence mode="wait">
          {user && (
            <motion.p 
              className="text-gray-300 text-sm sm:text-base text-center"
              initial={{ opacity: 0, y: -10 }}
              animate={{ opacity: 1, y: 0 }}
              exit={{ opacity: 0, y: 10 }}
              key="welcome-message"
            >
              Olá, <span className="text-white font-medium">{userName}</span>!
              <span className="hidden sm:inline"> Bem-vindo(a) de volta</span>
            </motion.p>
          )}
        </AnimatePresence>
      </div>
      
      {/* Botões de Ação */}
      <div className="flex items-center space-x-1 sm:space-x-2">
        {/* Botão de Logout */}
        <div className="relative">
          {showLogoutConfirm ? (
            <motion.div 
              className="absolute right-0 top-0 flex items-center bg-gray-900/95 backdrop-blur-sm rounded-lg p-1 shadow-lg border border-white/10"
              initial={{ opacity: 0, scale: 0.9, x: 20 }}
              animate={{ opacity: 1, scale: 1, x: 0 }}
              exit={{ opacity: 0, scale: 0.9, x: 20 }}
              transition={{ type: 'spring', damping: 25, stiffness: 300 }}
            >
              <span className="text-xs text-white px-2 whitespace-nowrap">Sair da conta?</span>
              <button
                onClick={handleLogout}
                disabled={isLoading || isLoggingOut}
                className="p-1.5 rounded-md hover:bg-green-500/20 text-green-400 hover:text-green-300 transition-colors"
                aria-label="Confirmar saída"
              >
                {isLoggingOut ? (
                  <Loader2 className="h-4 w-4 animate-spin" />
                ) : (
                  <Check className="h-4 w-4" />
                )}
              </button>
              <button
                onClick={() => setShowLogoutConfirm(false)}
                disabled={isLoading || isLoggingOut}
                className="p-1.5 rounded-md hover:bg-red-500/20 text-red-400 hover:text-red-300 transition-colors"
                aria-label="Cancelar"
              >
                <X className="h-4 w-4" />
              </button>
            </motion.div>
          ) : (
            <button
              onClick={() => setShowLogoutConfirm(true)}
              disabled={isLoading}
              className={`p-2 rounded-full transition-colors ${
                isLoading ? 'opacity-50 cursor-not-allowed' : 'hover:bg-white/10'
              }`}
              aria-label="Sair da conta"
              title="Sair"
            >
              <LogOut className={`h-5 w-5 ${isLoading ? 'text-gray-400' : 'text-gray-300 hover:text-white'}`} />
            </button>
          )}
        </div>
        
        {/* Botão de Configurações */}
        <button
          onClick={onSettingsClick}
          disabled={isLoading}
          className={`p-2 rounded-full transition-colors ${
            isLoading ? 'opacity-50 cursor-not-allowed' : 'hover:bg-white/10'
          }`}
          aria-label="Abrir configurações"
          title="Configurações"
        >
          <Settings className={`h-5 w-5 ${isLoading ? 'text-gray-400' : 'text-gray-300 hover:text-white'}`} />
        </button>
      </div>
    </header>
  );
};
