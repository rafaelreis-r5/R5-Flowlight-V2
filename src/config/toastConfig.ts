import { toast, ToastOptions, Toast } from 'react-hot-toast';

// Interface para opções de toast personalizadas
interface CustomToastOptions extends Partial<Pick<Toast, 'id' | 'icon' | 'duration' | 'ariaProps' | 'className' | 'style' | 'position' | 'iconTheme'>> {
  // Adicione aqui quaisquer propriedades personalizadas adicionais, se necessário
}
// Estilos base para os toasts
const baseStyle: React.CSSProperties = {
  background: '#2d0a42',
  color: '#fff',
  border: '1px solid #6d28d9',
  padding: '12px 16px',
  borderRadius: '8px',
  maxWidth: '90vw',
  textAlign: 'center',
  fontSize: '14px',
  lineHeight: '1.4',
  boxShadow: '0 4px 12px rgba(0, 0, 0, 0.15)'
};

// Configurações padrão para os toasts
const defaultOptions: ToastOptions = {
  duration: 5000,
  position: 'top-center',
  style: baseStyle
};

// Mensagens de erro comuns
export const errorMessages = {
  network: 'Não foi possível conectar ao servidor. Verifique sua conexão com a internet.',
  timeout: 'Tempo de conexão esgotado. Verifique sua conexão com a internet.',
  serverError: 'Erro interno do servidor. Por favor, tente novamente mais tarde.',
  unauthorized: 'Sessão expirada. Por favor, faça login novamente.',
  forbidden: 'Acesso não autorizado. Verifique suas permissões.',
  tooManyRequests: 'Muitas tentativas. Por favor, aguarde um momento antes de tentar novamente.',
  invalidData: 'Dados inválidos. Verifique as informações fornecidas.',
  emailInUse: 'Este e-mail já está cadastrado. Tente fazer login ou use outro e-mail.',
  userNotFound: 'Usuário não encontrado. Verifique seu nome de usuário ou e-mail.',
  invalidPassword: 'Senha incorreta. Tente novamente.',
  noPasswordSet: 'Nenhuma senha definida para este usuário. Por favor, cadastre uma senha.',
  invalidEmail: 'Por favor, insira um endereço de e-mail válido.',
  invalidName: 'O nome deve ter pelo menos 3 caracteres.',
  weakPassword: 'A senha deve conter pelo menos 6 caracteres, incluindo letras maiúsculas, minúsculas e números.',
  requiredField: 'Por favor, preencha todos os campos obrigatórios.',
};

// Funções auxiliares para exibir toasts
export const showSuccessToast = (message: string, options: CustomToastOptions = {}) => {
  return toast.success(message, {
    ...defaultOptions,
    ...options,
    style: {
      ...baseStyle,
      background: '#166534',
      border: '1px solid #22c55e',
      ...options.style
    },
    iconTheme: {
      primary: '#22c55e',
      secondary: '#fff'
    }
  });
};

export const showErrorToast = (message: string, options: CustomToastOptions = {}) => {
  return toast.error(message, {
    ...defaultOptions,
    ...options,
    style: {
      ...baseStyle,
      background: '#991b1b',
      border: '1px solid #ef4444',
      ...options.style
    },
    iconTheme: {
      primary: '#ef4444',
      secondary: '#fff'
    }
  });
};

export const showLoadingToast = (message: string, options: CustomToastOptions = {}) => {
  return toast.loading(message, {
    ...defaultOptions,
    ...options,
    style: {
      ...baseStyle,
      ...options.style
    }
  });
};
