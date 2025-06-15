// Configurações da API
export const API_CONFIG = {
  // URL base da API de autenticação
  AUTH_API_URL: import.meta.env.VITE_AUTH_API_URL || 'http://localhost:3030',
  
  // Timeout padrão para requisições (em milissegundos)
  REQUEST_TIMEOUT: 10000,
  
  // Cabeçalhos padrão
  DEFAULT_HEADERS: {
    'Content-Type': 'application/json',
    'Accept': 'application/json',
  },
} as const;
