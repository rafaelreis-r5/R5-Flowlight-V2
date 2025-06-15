import { useCallback, useEffect, useRef } from 'react';
import { toast, ToastPosition, ToastOptions as HotToastOptions } from 'react-hot-toast';
import { useSettings } from '../contexts/SettingsContext';

interface CustomNotificationOptions extends NotificationOptions {
  body?: string;
  icon?: string;
  tag?: string;
  requireInteraction?: boolean;
  onClick?: () => void;
}

interface ExtendedToastOptions extends HotToastOptions {
  action?: {
    label: string;
    onClick: () => void;
  };
}

declare global {
  interface Window {
    Notification: any;
  }
}

type NotificationType = 'info' | 'success' | 'warning' | 'error' | 'update' | 'reminder';

interface NotificationOptions {
  duration?: number;
  position?: ToastPosition;
  showProgress?: boolean;
  sound?: boolean;
  action?: {
    label: string;
    onClick: () => void;
  };
}

export const useNotification = () => {
  const { settings } = useSettings();
  const audioRefs = useRef<Record<string, HTMLAudioElement>>({});
  const notificationPermission = useRef<string>('default');
  
  // Efeito para configurar a posição padrão das notificações e carregar áudios
  useEffect(() => {
    // Carregar áudios
    ['info', 'success', 'warning', 'error', 'update', 'reminder'].forEach(type => {
      audioRefs.current[type] = new Audio(`/sounds/${type}.mp3`);
      audioRefs.current[type].load();
    });
    
    // Solicitar permissão para notificações
    if ('Notification' in window) {
      if (Notification.permission !== 'granted' && Notification.permission !== 'denied') {
        Notification.requestPermission().then(permission => {
          notificationPermission.current = permission;
        });
      } else {
        notificationPermission.current = Notification.permission;
      }
    }
    
    return () => {
      // Limpar áudios
      Object.values(audioRefs.current).forEach(audio => {
        audio.pause();
        audio.remove();
      });
    };
  }, []);

  const playSound = useCallback((type: NotificationType) => {
    if (!settings.notifications?.sounds) return;
    
    const audio = audioRefs.current[type] || audioRefs.current.info;
    if (audio) {
      audio.currentTime = 0; // Reiniciar o áudio se já estiver tocando
      audio.volume = 0.5;
      audio.play().catch(error => {
        console.warn('Erro ao reproduzir som:', error);
      });
    }
  }, [settings.notifications?.sounds]);
  
  const showNativeNotification = useCallback((title: string, options: CustomNotificationOptions) => {
    if (!('Notification' in window) || notificationPermission.current !== 'granted') {
      return;
    }
    
    // Verificar se o usuário está com a aba ativa
    const isWindowFocused = document.hasFocus();
    if (isWindowFocused && !settings.notifications.showWhenFocused) {
      return;
    }
    
    try {
      const notification = new Notification(title, options);
      
      notification.onclick = (event) => {
        event.preventDefault();
        window.focus();
        notification.close();
        
        if (options.onClick) {
          options.onClick();
        }
      };
      
      return notification;
    } catch (error) {
      console.error('Erro ao exibir notificação nativa:', error);
      return null;
    }
  }, [settings.notifications?.showWhenFocused]);

  const notify = useCallback(
    (
      message: string,
      type: NotificationType = 'info',
      options: NotificationOptions & {
        title?: string;
        native?: boolean;
        onClick?: () => void;
      } = {}
    ) => {
      // Verificar se as notificações estão habilitadas e se o tipo específico está ativo
      if (!settings.notifications?.enabled || !settings.notifications.types?.[type]) {
        return;
      }

      const {
        duration = settings.notifications.duration * 1000, // Converter para milissegundos
        position = settings.notifications.position as ToastPosition,
        showProgress = settings.notifications.showProgress,
        sound = settings.notifications.sounds,
        action,
        title = 'R5 Flowlight',
        native = settings.notifications.useNative,
        onClick,
        ...restOptions
      } = options;

      // Reproduzir som se estiver habilitado
      if (sound) {
        playSound(type);
      }

      // Exibir notificação nativa se estiver habilitado e suportado
      if (native && 'Notification' in window) {
        const notification = showNativeNotification(title, {
          body: message,
          icon: '/icons/icon-192x192.png',
          tag: `notification-${Date.now()}`,
          requireInteraction: type === 'reminder',
          ...restOptions,
          onClick
        });
        
        // Se a notificação nativa for exibida com sucesso, não exibir a notificação do toast
        if (notification) {
          return notification;
        }
      }

      // Configurações comuns para todos os tipos de notificação
      const toastOptions: ExtendedToastOptions = {
        duration,
        position,
        ...(showProgress && { 
          style: { 
            background: 'linear-gradient(to right, #6405d6, #f1011d)' 
          } 
        }),
      };

      // Adicionar ação se fornecida
      if (action) {
        toastOptions.action = {
          label: action.label,
          onClick: action.onClick,
        };
      }

      // Exibir notificação com base no tipo
      switch (type) {
        case 'success':
          return toast.success(message, toastOptions);
        case 'error':
          return toast.error(message, toastOptions);
        case 'warning':
          return toast(message, { ...toastOptions, icon: '⚠️' });
        case 'update':
          return toast(message, { ...toastOptions, icon: '🔄' });
        case 'reminder':
          return toast(message, { ...toastOptions, icon: '⏰' });
        case 'info':
        default:
          return toast(message, toastOptions);
      }
  }, [playSound, settings.notifications]);

  return { 
    notify,
    requestPermission: async () => {
      if ('Notification' in window) {
        const permission = await Notification.requestPermission();
        notificationPermission.current = permission;
        return permission;
      }
      return 'default';
    },
    hasPermission: () => {
      return notificationPermission.current === 'granted';
    }
  };
};
