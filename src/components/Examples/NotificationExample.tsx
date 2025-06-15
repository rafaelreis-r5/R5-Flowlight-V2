import React from 'react';
import { useNotification } from '../../hooks/useNotification';

const NotificationExample: React.FC = () => {
  const { notify } = useNotification();

  const showNotification = (type: string) => {
    const messages = {
      info: 'Esta é uma notificação informativa',
      success: 'Operação concluída com sucesso!',
      warning: 'Atenção: Esta é um aviso importante',
      error: 'Ocorreu um erro ao processar sua solicitação',
      update: 'Nova atualização disponível',
      reminder: 'Lembrete: Reunião em 10 minutos'
    };

    notify(messages[type as keyof typeof messages] || 'Notificação de exemplo', type as any);
  };

  return (
    <div className="p-6 space-y-4">
      <h2 className="text-2xl font-bold text-white mb-6">Exemplo de Notificações</h2>
      
      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
        {['info', 'success', 'warning', 'error', 'update', 'reminder'].map((type) => (
          <button
            key={type}
            onClick={() => showNotification(type)}
            className={`px-4 py-3 rounded-lg text-white font-medium transition-colors ${
              type === 'info' ? 'bg-blue-500 hover:bg-blue-600' :
              type === 'success' ? 'bg-green-500 hover:bg-green-600' :
              type === 'warning' ? 'bg-yellow-500 hover:bg-yellow-600' :
              type === 'error' ? 'bg-red-500 hover:bg-red-600' :
              type === 'update' ? 'bg-indigo-500 hover:bg-indigo-600' :
              'bg-purple-500 hover:bg-purple-600'
            }`}
          >
            Mostrar notificação {type}
          </button>
        ))}
      </div>

      <div className="mt-8 p-4 bg-gray-800 rounded-lg">
        <h3 className="text-lg font-semibold text-white mb-2">Como usar:</h3>
        <pre className="bg-gray-900 p-4 rounded-md overflow-x-auto text-sm text-gray-300">
          {`import { useNotification } from '../../hooks/useNotification';

const MyComponent = () => {
  const { notify } = useNotification();

  // Exemplo de uso
  const handleAction = () => {
    notify('Operação concluída com sucesso!', 'success', {
      duration: 5, // segundos
      position: 'top-right',
      showProgress: true,
      sound: true,
      action: {
        label: 'Desfazer',
        onClick: () => console.log('Ação desfeita')
      }
    });
  };

  return (
    <button onClick={handleAction}>
      Executar ação
    </button>
  );
};`}
        </pre>
      </div>
    </div>
  );
};

export default NotificationExample;
