import { Component, ErrorInfo, ReactNode } from 'react';
import { showErrorToast } from '../config/toastConfig';

interface Props {
  children: ReactNode;
  fallback?: ReactNode;
}

interface State {
  hasError: boolean;
  error?: Error;
  errorInfo?: ErrorInfo;
}

class ErrorBoundary extends Component<Props, State> {
  public state: State = {
    hasError: false,
  };

  public static getDerivedStateFromError(error: Error): State {
    // Atualiza o state para que a próxima renderização mostre a UI de fallback
    return { hasError: true, error };
  }

  public componentDidCatch(error: Error, errorInfo: ErrorInfo) {
    // Você também pode registrar o erro em um serviço de relatórios de erro
    console.error('Erro capturado pelo ErrorBoundary:', error, errorInfo);
    this.setState({ error, errorInfo });
    
    // Mostra um toast de erro amigável
    showErrorToast('Ocorreu um erro inesperado. Por favor, recarregue a página.');
  }

  public render() {
    if (this.state.hasError) {
      // Você pode renderizar qualquer UI de fallback
      return this.props.fallback || (
        <div className="min-h-screen flex items-center justify-center bg-gray-900 text-white p-4">
          <div className="max-w-md w-full bg-gray-800 p-6 rounded-lg shadow-lg">
            <h2 className="text-2xl font-bold text-red-400 mb-4">Algo deu errado</h2>
            <p className="mb-4">
              Ocorreu um erro inesperado. Por favor, recarregue a página ou tente novamente mais tarde.
            </p>
            {this.state.error && (
              <details className="bg-gray-700 p-3 rounded text-sm overflow-auto max-h-40">
                <summary className="font-medium mb-2 cursor-pointer">Detalhes do erro</summary>
                <p className="text-red-300 font-mono break-words">
                  {this.state.error.toString()}
                </p>
                {this.state.errorInfo?.componentStack && (
                  <pre className="mt-2 text-gray-400 text-xs overflow-auto">
                    {this.state.errorInfo.componentStack}
                  </pre>
                )}
              </details>
            )}
            <button
              onClick={() => window.location.reload()}
              className="mt-4 px-4 py-2 bg-blue-600 hover:bg-blue-700 rounded text-white font-medium transition-colors"
            >
              Recarregar Página
            </button>
          </div>
        </div>
      );
    }

    return this.props.children;
  }
}

export default ErrorBoundary;
