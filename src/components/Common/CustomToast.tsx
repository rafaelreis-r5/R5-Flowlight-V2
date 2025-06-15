import React from 'react';
import { Toaster, toast, Toast } from 'react-hot-toast';
import { useSettings } from '../../contexts/SettingsContext';

const getTypeStyles = (type: string) => {
  switch (type) {
    case 'success':
      return 'bg-green-500 border-green-600';
    case 'error':
      return 'bg-red-500 border-red-600';
    case 'warning':
      return 'bg-yellow-500 border-yellow-600';
    case 'update':
      return 'bg-blue-500 border-blue-600';
    case 'reminder':
      return 'bg-purple-500 border-purple-600';
    case 'info':
    default:
      return 'bg-indigo-500 border-indigo-600';
  }
};

export const CustomToaster: React.FC = () => {
  const { settings } = useSettings();
  
  return (
    <Toaster
      position={settings.notifications?.position as any || 'top-right'}
      toastOptions={{
        duration: (settings.notifications?.duration || 5) * 1000,
        style: {
          background: 'transparent',
          boxShadow: 'none',
          padding: 0,
          margin: 0,
        },
      }}
    >
      {(t: Toast) => {
        const toastType = t.type as string;
        const duration = t.duration || 5000;
        const timeRemaining = 'timeRemaining' in t ? (t as any).timeRemaining : 0;
        
        return (
          <div
            className={`transform transition-all duration-300 ${
              t.visible ? 'animate-enter' : 'animate-leave'
            }`}
          >
            <div
              className={`${getTypeStyles(toastType)} text-white px-4 py-3 rounded-lg border-l-4 shadow-lg mb-2 flex items-start max-w-md`}
            >
              <div className="flex-1">
                <div className="flex items-center">
                  {t.icon || (
                    <span className="mr-2">
                      {toastType === 'success' && '✅'}
                      {toastType === 'error' && '❌'}
                      {toastType === 'warning' && '⚠️'}
                      {toastType === 'update' && '🔄'}
                      {toastType === 'reminder' && '⏰'}
                      {!['success', 'error', 'warning', 'update', 'reminder'].includes(toastType) && 'ℹ️'}
                    </span>
                  )}
                  <div className="flex-1">
                    {typeof t.message === 'function' ? t.message(t) : t.message}
                  </div>
                </div>
                {settings.notifications?.showProgress && t.visible && (
                  <div className="w-full bg-black/20 h-1 mt-2 rounded-full overflow-hidden">
                    <div
                      className="bg-white h-full transition-all duration-300"
                      style={{
                        width: `${(1 - (duration - timeRemaining) / duration) * 100}%`,
                      }}
                    />
                  </div>
                )}
              </div>
              <button
                onClick={() => toast.dismiss(t.id)}
                className="ml-2 text-white/70 hover:text-white transition-colors"
                aria-label="Fechar notificação"
              >
                ×
              </button>
            </div>
          </div>
        );
      }}
    </Toaster>
  );
};

export default CustomToaster;
