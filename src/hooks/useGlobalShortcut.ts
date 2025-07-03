import { useEffect, useCallback } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { getCurrent } from '@tauri-apps/api/window';

interface UseGlobalShortcutOptions {
  autoHide?: boolean;
  hideDelay?: number;
}

export interface WindowState {
  visible: boolean;
  focused: boolean;
  always_on_top: boolean;
}

export const useGlobalShortcut = (options: UseGlobalShortcutOptions = {}) => {
  const { autoHide = true, hideDelay = 150 } = options;

  const toggleWindow = useCallback(async (): Promise<WindowState | null> => {
    try {
      const result = await invoke<WindowState>('toggle_search_window');
      return result;
    } catch (error) {
      console.error('Failed to toggle search window:', error);
      return null;
    }
  }, []);

  const showWindow = useCallback(async (): Promise<WindowState | null> => {
    try {
      const result = await invoke<WindowState>('show_search_window');
      return result;
    } catch (error) {
      console.error('Failed to show search window:', error);
      return null;
    }
  }, []);

  const hideWindow = useCallback(async (): Promise<WindowState | null> => {
    try {
      const result = await invoke<WindowState>('hide_search_window');
      return result;
    } catch (error) {
      console.error('Failed to hide search window:', error);
      return null;
    }
  }, []);

  const isWindowVisible = useCallback(async (): Promise<boolean> => {
    try {
      return await invoke<boolean>('is_search_window_visible');
    } catch (error) {
      console.error('Failed to check window visibility:', error);
      return false;
    }
  }, []);

  // Configurar eventos de janela para auto-hide APENAS NA JANELA DE BUSCA
  useEffect(() => {
    const currentWindow = getCurrentWindow();
    
    // Listener para quando a janela perde o foco
    let unlistenBlur: (() => void) | undefined;
    
    // ✅ CORREÇÃO: Só aplicar auto-hide se for a janela de busca
    if (autoHide && currentWindow.label === 'search') {
      currentWindow.onFocusChanged(({ payload: focused }) => {
        if (!focused) {
          // Auto-hide com delay configurável
          setTimeout(() => {
            hideWindow();
          }, hideDelay);
        }
      }).then(unlisten => {
        unlistenBlur = unlisten;
      }).catch(error => {
        console.error('Failed to setup focus listener:', error);
      });
    }

    // Cleanup
    return () => {
      if (unlistenBlur) {
        unlistenBlur();
      }
    };
  }, [autoHide, hideDelay, hideWindow]);

  // Listener para tecla ESC para esconder a janela APENAS NA JANELA DE BUSCA
  useEffect(() => {
    const currentWindow = getCurrentWindow();
    
    const handleKeyDown = (event: KeyboardEvent) => {
      // ✅ CORREÇÃO: ESC só funciona na janela de busca
      if (event.key === 'Escape' && currentWindow.label === 'search') {
        event.preventDefault();
        hideWindow();
      }
    };

    // Adiciona listener no documento
    document.addEventListener('keydown', handleKeyDown);
    
    return () => {
      document.removeEventListener('keydown', handleKeyDown);
    };
  }, [hideWindow]);

  // Listener para clique fora da janela APENAS NA JANELA DE BUSCA
  useEffect(() => {
    const currentWindow = getCurrentWindow();
    
    // ✅ CORREÇÃO: Click outside só funciona na janela de busca
    if (!autoHide || currentWindow.label !== 'search') return;

    const handleClickOutside = (event: MouseEvent) => {
      const target = event.target as Element;
      
      // Verifica se o clique foi fora do container principal da busca
      const searchContainer = document.querySelector('[data-search-container]');
      if (searchContainer && !searchContainer.contains(target)) {
        hideWindow();
      }
    };

    // Adiciona listener para cliques
    document.addEventListener('mousedown', handleClickOutside);
    
    return () => {
      document.removeEventListener('mousedown', handleClickOutside);
    };
  }, [autoHide, hideWindow]);

  return {
    toggleWindow,
    showWindow,
    hideWindow,
    isWindowVisible
  };
};