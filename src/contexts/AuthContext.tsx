import { invoke } from '@tauri-apps/api/core';
import React, { createContext, ReactNode, useCallback, useContext, useEffect, useState } from 'react';
import { showErrorToast, showSuccessToast } from '../config/toastConfig';

interface User {
  id: string;
  email: string;
  name: string;
  user_metadata?: {
    full_name?: string;
    [key: string]: any;
  };
}

interface CompleteFirstAccessParams {
  name: string;
  email: string;
  password: string;
}

interface AuthContextType {
  isAuthenticated: boolean;
  user: User | null;
  login: (username: string, password: string) => Promise<boolean>;
  logout: () => void;
  completeFirstAccess: (userData: CompleteFirstAccessParams) => Promise<{ success: boolean; error?: string }>;
  isLoading: boolean;
}

const AuthContext = createContext<AuthContextType | undefined>(undefined);

interface AuthProviderProps {
  children: ReactNode;
}

export const AuthProvider: React.FC<AuthProviderProps> = ({ children }) => {
  const [user, setUser] = useState<User | null>(null);
  const [isAuthenticated, setIsAuthenticated] = useState<boolean>(false);
  const [isLoading, setIsLoading] = useState<boolean>(true);

  // Função para fazer login
  const login = useCallback(async (username: string, password: string): Promise<boolean> => {
    console.log('Iniciando processo de login para:', username);

    // Validação básica
    if (!username || !password) {
      const errorMsg = !username ? 'Por favor, informe seu nome de usuário.' : 'Por favor, informe sua senha.';
      console.error('Validação falhou:', errorMsg);
      showErrorToast(errorMsg);
      return false;
    }

    // Verifica se é o usuário admin (case-sensitive)
    const isAdminUser = username === 'SudoAdmin' && password === 'R5hub2025flowlight';

    if (isAdminUser) {
      console.log('Tentando autenticar como administrador local...');
      try {
        setIsLoading(true);
        console.log('Autenticação local de administrador em andamento...');

        const adminUser: User = {
          id: 'admin-001',
          email: 'admin@r5hub.com',
          name: 'Administrador',
          user_metadata: {
            full_name: 'Administrador do Sistema',
            is_admin: true,
            avatar_url: '',
            created_at: new Date().toISOString()
          }
        };

        console.log('Admin autenticado com sucesso - Antes de atualizar o estado');

        // Primeiro atualiza o estado
        setUser(adminUser);
        setIsAuthenticated(true);

        console.log('Estado atualizado - isAuthenticated:', true, 'user:', adminUser);

        // Depois armazena no localStorage
        try {
          localStorage.setItem('user', JSON.stringify(adminUser));
          localStorage.setItem('is_admin', 'true');
          console.log('Dados do admin armazenados no localStorage com sucesso');

          // Força um evento de mudança de autenticação
          window.dispatchEvent(new Event('authStateChanged'));

          showSuccessToast('Login de administrador realizado com sucesso!');
          console.log('Login de admin concluído com sucesso');
          return true;
        } catch (error) {
          console.error('Erro ao salvar no localStorage:', error);
          showErrorToast('Erro ao salvar dados de autenticação');
          return false;
        }
      } catch (error) {
        console.error('Erro ao autenticar admin:', error);
        showErrorToast('Erro ao autenticar como administrador');
        return false;
      } finally {
        setIsLoading(false);
      }
    }

    // Se não for o admin, tenta autenticar via Supabase
    try {
      setIsLoading(true);
      console.log('Chamando backend para autenticação via Supabase...');

      // Chama o comando Tauri para autenticar com o Supabase
      const response = await invoke<{ user: User; access_token: string }>('login', {
        email: username.toLowerCase(), // Garante que o email esteja em minúsculas
        password
      });

      console.log('Resposta do backend:', response ? 'sucesso' : 'vazia');

      if (!response || !response.user) {
        console.error('Resposta de autenticação inválida:', response);
        throw new Error('Resposta de autenticação inválida');
      }

      const userData = response.user;
      const user: User = {
        id: userData.id,
        email: userData.email,
        name: userData.user_metadata?.full_name || userData.user_metadata?.name || userData.email.split('@')[0],
        user_metadata: userData.user_metadata
      };

      console.log('Usuário autenticado com sucesso:', { id: user.id, email: user.email });

      setUser(user);
      setIsAuthenticated(true);

      // Armazena o token de acesso e os dados do usuário
      localStorage.setItem('access_token', response.access_token);
      localStorage.setItem('user', JSON.stringify(user));

      console.log('Dados do usuário armazenados no localStorage');
      showSuccessToast('Login realizado com sucesso!');
      return true;
    } catch (error) {
      console.error('Erro durante o login:', error);
      let errorMessage = 'Falha ao fazer login. Verifique suas credenciais.';

      if (error instanceof Error) {
        // Tratamento de erros específicos
        if (error.message.includes('Invalid login credentials')) {
          errorMessage = 'Credenciais inválidas. Verifique seu e-mail e senha.';
        } else if (error.message.includes('Email not confirmed')) {
          errorMessage = 'E-mail não confirmado. Por favor, verifique sua caixa de entrada.';
        } else if (error.message.includes('User not found')) {
          errorMessage = 'Usuário não encontrado. Verifique seu e-mail ou cadastre-se.';
        } else if (error.message.includes('NetworkError')) {
          errorMessage = 'Erro de conexão. Verifique sua conexão com a internet.';
        } else {
          errorMessage = error.message;
        }
      }

      showErrorToast(errorMessage);
      return false;
    } finally {
      setIsLoading(false);
    }
  }, []);

  // Função para logout
  const logout = useCallback(() => {
    console.log('Efetuando logout...');

    // Limpa os dados de autenticação
    localStorage.removeItem('access_token');
    localStorage.removeItem('refresh_token');
    localStorage.removeItem('user');
    localStorage.removeItem('is_admin');

    // Atualiza o estado
    setUser(null);
    setIsAuthenticated(false);

    console.log('Logout realizado com sucesso');
    showSuccessToast('Logout realizado com sucesso!');

    // Redireciona para a página de login
    window.location.href = '/login';
  }, []);

  // Função para completar o primeiro acesso
  const completeFirstAccess = useCallback(async (userData: { name: string; email: string; password: string }) => {
    try {
      setIsLoading(true);
      console.log('Completando primeiro acesso para:', userData.email);

      // Valida os dados antes de enviar
      if (!userData.email || !userData.password || !userData.name) {
        console.error('Dados incompletos para completar o primeiro acesso');
        return { success: false, error: 'Por favor, preencha todos os campos obrigatórios.' };
      }

      console.log('Chamando comando Tauri complete_first_access com:', {
        email: userData.email.toLowerCase(),
        name: userData.name,
        password: '***' // Não logar a senha real
      });

      // Prepara os parâmetros para corresponder exatamente ao que o Tauri espera
      const params = {
        email: userData.email.toLowerCase().trim(),
        password: userData.password,
        fullName: userData.name.trim() // Tauri está esperando fullName em camelCase
      };

      console.log('Parâmetros sendo enviados para o backend:', params);

      // Chama o comando Tauri para atualizar o usuário existente
      const result = await invoke('complete_first_access', params).catch(error => {
        console.error('Erro ao chamar complete_first_access:', error);
        throw error; // Rejeita a promessa para ser tratada no catch externo
      });

      console.log('Resposta do complete_first_access:', result);

      // Mostra mensagem de sucesso
      showSuccessToast('Cadastro concluído com sucesso! Faça login para continuar.');
      return { success: true };

    } catch (error) {
      console.error('Erro ao completar o primeiro acesso:', error);
      let errorMessage = 'Falha ao completar o cadastro. Tente novamente mais tarde.';

      if (error instanceof Error) {
        console.error('Detalhes do erro:', {
          name: error.name,
          message: error.message,
          stack: error.stack
        });

        if (error.message.includes('Usuário não encontrado') || error.message.includes('não encontrado')) {
          errorMessage = 'E-mail não encontrado. Verifique se digitou corretamente ou entre em contato com o suporte.';
        } else if (error.message.includes('senha') || error.message.includes('password')) {
          errorMessage = 'Erro ao definir a senha. A senha deve ter pelo menos 6 caracteres.';
        } else if (error.message.includes('e-mail') || error.message.includes('email')) {
          errorMessage = 'E-mail inválido. Verifique o formato do e-mail.';
        } else if (error.message.includes('422') || error.message.includes('422')) {
          errorMessage = 'Dados inválidos fornecidos. Verifique os campos e tente novamente.';
        } else if (error.message.includes('timeout') || error.message.includes('network')) {
          errorMessage = 'Erro de conexão. Verifique sua conexão com a internet.';
        } else {
          // Mostra a mensagem de erro original para facilitar o debug
          errorMessage = `Erro: ${error.message}`;
        }
      }

      console.error('Mensagem de erro a ser exibida:', errorMessage);
      showErrorToast(errorMessage);
      return { success: false, error: errorMessage };
    } finally {
      setIsLoading(false);
    }
  }, []);

  // Verifica a autenticação ao carregar o componente
  useEffect(() => {
    const checkAuth = async () => {
      if (import.meta.env.DEV) {
        console.log('Verificando autenticação...');
        console.log('Conteúdo do localStorage:', {
          is_admin: localStorage.getItem('is_admin'),
          user: localStorage.getItem('user'),
          access_token: localStorage.getItem('access_token') ? 'presente' : 'ausente'
        });
      }

      // Verifica se é o admin local
      const isAdmin = localStorage.getItem('is_admin') === 'true';
      const storedUser = localStorage.getItem('user');

      if (isAdmin && storedUser) {
        console.log('Verificando credenciais de administrador local...');
        try {
          const user = JSON.parse(storedUser);
          console.log('Dados do admin recuperados do localStorage:', user);

          // Verifica se o usuário tem a estrutura esperada
          if (user && user.id === 'admin-001' && user.email === 'admin@r5hub.com') {
            console.log('Admin local validado com sucesso');
            setUser(user);
            setIsAuthenticated(true);
            console.log('Admin autenticado com sucesso no useEffect');
            setIsLoading(false);
            return;
          } else {
            console.warn('Dados de admin inválidos no localStorage');
            // Limpa os dados inválidos
            localStorage.removeItem('user');
            localStorage.removeItem('is_admin');
          }
        } catch (error) {
          console.error('Erro ao analisar dados do admin:', error);
          // Limpa os dados inválidos
          localStorage.removeItem('user');
          localStorage.removeItem('is_admin');
        }
      }

      // Verifica autenticação via Supabase para usuários regulares
      const accessToken = localStorage.getItem('access_token');

      if (!accessToken || !storedUser) {
        console.log('Nenhum token de acesso ou usuário encontrado no localStorage');
        setIsLoading(false);
        return;
      }

      console.log('Token de acesso encontrado, validando...');

      try {
        setIsLoading(true);

        // Verifica se o token ainda é válido
        console.log('Chamando backend para validar token...');
        const userData = await invoke<User>('get_user', { accessToken });

        if (userData) {
          console.log('Token válido, usuário autenticado:', userData.email);

          const user = {
            id: userData.id,
            email: userData.email,
            name: userData.user_metadata?.full_name ||
                 userData.user_metadata?.name ||
                 userData.email.split('@')[0],
            user_metadata: userData.user_metadata
          };

          setUser(user);
          setIsAuthenticated(true);
          // Atualiza os dados do usuário no localStorage
          localStorage.setItem('user', JSON.stringify(user));
          console.log('Dados do usuário atualizados no localStorage');
        } else {
          console.warn('Token inválido ou expirado, limpando dados...');
          // Se não encontrou o usuário, limpa os dados
          localStorage.removeItem('access_token');
          localStorage.removeItem('user');
        }
      } catch (error) {
        console.warn('Erro ao verificar autenticação:', error);

        // Tratamento de erros específicos
        let errorMessage = 'Erro ao verificar autenticação';

        if (error instanceof Error) {
          if (error.message.includes('invalid_token') ||
              error.message.includes('Token expired') ||
              error.message.includes('401')) {
            errorMessage = 'Sessão expirada. Por favor, faça login novamente.';
          } else if (error.message.includes('network error')) {
            errorMessage = 'Erro de conexão. Verifique sua conexão com a internet.';
          } else {
            errorMessage = `Erro de autenticação: ${error.message}`;
          }
        }

        console.error(errorMessage);
        showErrorToast(errorMessage);

        // Limpa os dados de autenticação em caso de erro
        localStorage.removeItem('access_token');
        localStorage.removeItem('user');
      } finally {
        console.log('Verificação de autenticação concluída');
        setIsLoading(false);
      }
    };

    // Adiciona um pequeno atraso para evitar múltiplas verificações simultâneas
    const timer = setTimeout(() => {
      checkAuth();
    }, 100);

    return () => {
      clearTimeout(timer);
    };
  }, []);

  return (
    <AuthContext.Provider
      value={{
        isAuthenticated,
        user,
        login,
        logout,
        completeFirstAccess,
        isLoading,
      }}
    >
      {children}
    </AuthContext.Provider>
  );
};

export const useAuth = (): AuthContextType => {
  const context = useContext(AuthContext);
  if (context === undefined) {
    throw new Error('useAuth deve ser usado dentro de um AuthProvider');
  }
  return context;
};
