import React, { useState, useRef, useEffect, useCallback } from 'react';
import { motion } from 'framer-motion';
import { Lock, User, AlertCircle, Eye, EyeOff, ArrowRight } from 'lucide-react';
import { toast } from 'react-hot-toast';
import { useAuth } from '../../contexts/AuthContext';
import { SignUpFlow } from './SignUp/SignUpFlow';
import r5Logo from '../../assets/icons/app-icon.png';

interface LoginProps {
  onLoginSuccess: () => void;
}

export const Login: React.FC<LoginProps> = ({ onLoginSuccess }) => {
  const [username, setUsername] = useState('');
  const [password, setPassword] = useState('');
  const [error, setError] = useState('');
  const [showPassword, setShowPassword] = useState(false);
  const { login, isLoading } = useAuth();
  const [showSignUp, setShowSignUp] = useState(false);

  const handleSubmit = useCallback(async (e: React.FormEvent) => {
    e.preventDefault();
    
    // Validação dos campos
    if (!username.trim()) {
      setError('Por favor, informe seu nome de usuário');
      document.getElementById('username')?.focus();
      return;
    }
    
    if (!password.trim()) {
      setError('Por favor, informe sua senha');
      document.getElementById('password')?.focus();
      return;
    }

    // Limpa erros anteriores
    setError('');
    
    // Adiciona classe de carregamento ao botão
    const submitButton = e.currentTarget.querySelector('button[type="submit"]');
    if (submitButton) {
      submitButton.setAttribute('aria-busy', 'true');
    }

    try {
      console.log('Iniciando processo de login...');
      const success = await login(username, password);
      
      if (success) {
        console.log('Login realizado com sucesso, redirecionando...');
        // Feedback visual para o usuário antes do redirecionamento
        submitButton?.classList.add('success');
        // Aguarda um pouco para mostrar o feedback visual
        await new Promise(resolve => setTimeout(resolve, 500));
        // Chama a função de sucesso para notificar o componente pai
        // O redirecionamento será tratado pelo AppContent quando o estado de autenticação mudar
        onLoginSuccess();
      } else {
        console.warn('Login retornou falso, mas não houve erro');
        setError('Credenciais inválidas. Tente novamente.');
        // Foca no campo de senha para nova tentativa
        document.getElementById('password')?.focus();
      }
    } catch (err) {
      console.error('Erro durante o login:', err);
      
      // Mensagem de erro padrão
      let errorMessage = 'Erro ao fazer login. Verifique suas credenciais e tente novamente.';
      let showToast = true;
      
      // Tratamento detalhado de erros
      if (err instanceof Error) {
        // Erros de rede
        if (err.message.includes('Failed to fetch') || err.message.includes('NetworkError')) {
          errorMessage = 'Não foi possível conectar ao servidor. Verifique sua conexão com a internet.';
        } 
        // Timeout
        else if (err.message.includes('timeout') || err.name === 'AbortError') {
          errorMessage = 'Tempo de conexão esgotado. Verifique sua conexão com a internet.';
        }
        // Erros específicos do backend já traduzidos
        else if (err.message.includes('Usuário não encontrado')) {
          errorMessage = 'Usuário não encontrado. Verifique seu nome de usuário ou e-mail.';
        }
        else if (err.message.includes('Nenhuma senha definida')) {
          errorMessage = 'Nenhuma senha definida para este usuário. Por favor, cadastre uma senha.';
        }
        else if (err.message.includes('Senha incorreta') || err.message.includes('Credenciais inválidas')) {
          errorMessage = 'Senha incorreta. Tente novamente.';
        }
        // Outros erros
        else if (err.message) {
          errorMessage = err.message;
          
          // Não mostra toast para erros de validação específicos
          if (errorMessage.includes('caracteres') || 
              errorMessage.includes('nome de usuário') ||
              errorMessage.includes('senha') ||
              errorMessage === 'Por favor, informe seu nome de usuário' ||
              errorMessage === 'Por favor, informe sua senha') {
            showToast = false;
          }
        }
      }
      
      // Define a mensagem de erro no estado
      setError(errorMessage);
      
      // Exibe o toast apenas se necessário
      if (showToast) {
        const toastId = 'login-error';
        const existingToast = document.querySelector(`[data-toast-id="${toastId}"]`);
        
        if (!existingToast) {
          toast.error(errorMessage, { 
            id: toastId,
            duration: 5000,
            position: 'top-center',
            style: {
              background: '#2d0a42',
              color: '#fff',
              border: '1px solid #6d28d9',
              padding: '12px 16px',
              borderRadius: '8px',
              maxWidth: '90vw',
              textAlign: 'center'
            }
          });
        }
      }
    }
  }, [username, password, login, onLoginSuccess]);

  const videoRef = useRef<HTMLVideoElement>(null);

  useEffect(() => {
    const initialize = async () => {
      // Tenta reproduzir o vídeo
      if (videoRef.current) {
        try {
          await videoRef.current.play().catch(err => {
            console.warn('Não foi possível reproduzir o vídeo automaticamente:', err);
            // Tenta reproduzir com mute se a autoplay policy bloquear
            if (videoRef.current) {
              videoRef.current.muted = true;
              videoRef.current.play().catch(console.warn);
            }
          });
        } catch (err) {
          console.error('Erro ao reproduzir o vídeo:', err);
        }
      }
    };

    initialize();
    
    // Limpeza
    return () => {
      if (videoRef.current) {
        videoRef.current.pause();
        videoRef.current.currentTime = 0;
      }
    };
  }, []);

  return (
    <>
      {showSignUp && (
        <SignUpFlow 
          onClose={() => setShowSignUp(false)}
          onSwitchToLogin={(email) => {
            setShowSignUp(false);
            if (email) {
              setUsername(email);
            }
          }}
          initialEmail={username}
        />
      )}
      <div className="relative min-h-screen overflow-hidden">
        {/* Vídeo de fundo */}
        <div className="absolute inset-0 z-0">
          <video
            ref={videoRef}
            autoPlay
            muted
            loop
            playsInline
            className="w-full h-full object-cover opacity-30"
            aria-label="Background video"
          >
            <source src="/src/assets/movies/r5-bgvideo.mp4" type="video/mp4" />
            Seu navegador não suporta vídeos HTML5.
          </video>
        </div>
        
        {/* Overlay escuro */}
        <div className="absolute inset-0 bg-black/50 z-10"></div>
        
        {/* Conteúdo */}
        <div className="relative z-20 min-h-screen flex items-center justify-center p-4">
          <motion.div 
            initial={{ opacity: 0, y: 20 }}
            animate={{ opacity: 1, y: 0 }}
            transition={{ duration: 0.5 }}
            className="w-full max-w-md"
          >
            {/* Logo e Título */}
            <div className="text-center mb-8">
              <motion.div 
                className="flex justify-center mb-4"
                initial={{ scale: 0.9, opacity: 0 }}
                animate={{ scale: 1, opacity: 1 }}
                transition={{ delay: 0.2, duration: 0.5 }}
              >
                <img 
                  src={r5Logo} 
                  alt="R5 Flowlight Logo" 
                  className="w-48 h-48 object-contain"
                  draggable="false"
                />
              </motion.div>
              <motion.h1 
                className="text-3xl font-bold bg-gradient-to-r from-primary-purple to-accent-red bg-clip-text text-transparent mb-2"
                initial={{ y: -10, opacity: 0 }}
                animate={{ y: 0, opacity: 1 }}
                transition={{ delay: 0.3, duration: 0.5 }}
              >
                R5 Flowlight
              </motion.h1>
              <motion.p 
                className="text-gray-400 text-sm"
                initial={{ y: -5, opacity: 0 }}
                animate={{ y: 0, opacity: 1 }}
                transition={{ delay: 0.4, duration: 0.5 }}
              >
                Sua plataforma de produtividade
              </motion.p>
            </div>
            
            <motion.div 
              className="bg-white/5 backdrop-blur-lg rounded-2xl shadow-xl overflow-hidden border border-white/10"
              initial={{ opacity: 0, y: 20 }}
              animate={{ opacity: 1, y: 0 }}
              transition={{ delay: 0.5, duration: 0.5 }}
              role="dialog"
              aria-labelledby="login-heading"
            >
              <div className="p-8">
                <h2 id="login-heading" className="text-2xl font-semibold text-white text-center mb-2">Bem-vindo de volta</h2>
                <p className="text-gray-400 text-center mb-6">Faça login para acessar sua conta</p>

                {error && (
                  <motion.div 
                    className="mb-6 p-3 bg-red-500/10 border border-red-500/30 rounded-lg text-red-300 text-sm flex items-center"
                    initial={{ opacity: 0, y: -10 }}
                    animate={{ opacity: 1, y: 0 }}
                    exit={{ opacity: 0, y: -10 }}
                  >
                    <AlertCircle className="w-5 h-5 mr-2 flex-shrink-0" />
                    <span>{error}</span>
                  </motion.div>
                )}

                <form onSubmit={handleSubmit} className="space-y-4" aria-label="Formulário de login">
                  <div>
                    <label htmlFor="username" className="block text-sm font-medium text-gray-300 mb-1">
                      Usuário
                    </label>
                    <div className="relative">
                      <div className="absolute inset-y-0 left-0 pl-3 flex items-center pointer-events-none">
                        <User className="h-5 w-5 text-gray-500" />
                      </div>
                      <input
                        id="username"
                        type="text"
                        value={username}
                        onChange={(e) => setUsername(e.target.value)}
                        className="w-full pl-10 pr-3 py-3 bg-white/5 border border-white/10 rounded-lg text-white placeholder-gray-500 focus:outline-none focus:ring-2 focus:ring-primary-purple/50 focus:border-transparent transition-colors duration-200"
                        placeholder="Digite seu usuário"
                        autoComplete="username"
                        disabled={isLoading}
                        aria-label="Nome de usuário"
                        aria-required="true"
                      />
                    </div>
                  </div>

                  <div>
                    <div className="flex items-center justify-between mb-1">
                      <label htmlFor="password" className="block text-sm font-medium text-gray-300">
                        Senha
                      </label>
                      <button
                        type="button"
                        onClick={() => setShowPassword(!showPassword)}
                        className="text-xs text-gray-400 hover:text-primary-purple transition-colors focus:outline-none focus-visible:ring-2 focus-visible:ring-primary-purple/50 focus-visible:ring-offset-2 focus-visible:ring-offset-gray-900 rounded px-1 py-0.5"
                        aria-expanded={showPassword}
                        aria-controls="password"
                      >
                        {showPassword ? 'Ocultar senha' : 'Mostrar senha'}
                      </button>
                    </div>
                    <div className="relative">
                      <div className="absolute inset-y-0 left-0 pl-3 flex items-center pointer-events-none">
                        <Lock className="h-5 w-5 text-gray-500" />
                      </div>
                      <input
                        id="password"
                        type={showPassword ? "text" : "password"}
                        value={password}
                        onChange={(e) => setPassword(e.target.value)}
                        className="w-full pl-10 pr-10 py-3 bg-white/5 border border-white/10 rounded-lg text-white placeholder-gray-500 focus:outline-none focus:ring-2 focus:ring-primary-purple/50 focus:border-transparent transition-colors duration-200"
                        placeholder="••••••••"
                        autoComplete="current-password"
                        disabled={isLoading}
                        aria-required="true"
                      />
                      <button
                        type="button"
                        onClick={() => setShowPassword(!showPassword)}
                        className="absolute inset-y-0 right-0 pr-3 flex items-center text-gray-400 hover:text-white focus:outline-none focus-visible:ring-2 focus-visible:ring-primary-purple/50 focus-visible:ring-offset-2 focus-visible:ring-offset-gray-900 rounded-l-md transition-colors"
                        aria-label={showPassword ? 'Ocultar senha' : 'Mostrar senha'}
                        aria-expanded={showPassword}
                        aria-controls="password"
                        tabIndex={-1}
                      >
                        {showPassword ? (
                          <EyeOff className="h-5 w-5" aria-hidden="true" />
                        ) : (
                          <Eye className="h-5 w-5" aria-hidden="true" />
                        )}
                      </button>
                    </div>
                  </div>

                  <button
                    type="submit"
                    disabled={isLoading}
                    className={`w-full py-3 px-4 rounded-lg font-medium text-white transition-all ${
                      isLoading
                        ? 'bg-primary-purple/50 cursor-not-allowed'
                        : 'bg-gradient-to-r from-primary-purple to-accent-red hover:opacity-90 focus:outline-none focus-visible:ring-2 focus-visible:ring-offset-2 focus-visible:ring-primary-purple/70'
                    } relative overflow-hidden`}
                    aria-busy={isLoading}
                    aria-live="polite"
                  >
                    <span className={`relative z-10 flex items-center justify-center ${isLoading ? 'opacity-0' : 'opacity-100'}`}>
                      Entrar
                    </span>
                    {isLoading && (
                      <span className="absolute inset-0 flex items-center justify-center">
                        <svg className="animate-spin h-5 w-5 text-white" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
                          <circle className="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" strokeWidth="4"></circle>
                          <path className="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
                        </svg>
                        <span className="sr-only">Processando...</span>
                      </span>
                    )}
                  </button>
                </form>
                
                <div className="mt-6 text-center">
                  <button
                    onClick={() => setShowSignUp(true)}
                    className="text-sm text-primary-purple hover:text-primary-purple/80 font-medium inline-flex items-center group"
                  >
                    Primeiro acesso
                    <ArrowRight className="ml-1 w-4 h-4 transition-transform group-hover:translate-x-1" />
                  </button>
                </div>
              </div>
              
              <div className="px-8 py-4 bg-black/20 text-center border-t border-white/5">
                <p className="text-xs text-gray-400">
                  {new Date().getFullYear()} R5 Flowlight. Todos os direitos reservados.
                </p>
              </div>
            </motion.div>
          </motion.div>
        </div>
      </div>
    </>
  );
};

export default Login;
