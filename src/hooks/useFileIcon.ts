import { useState, useEffect, useCallback, useMemo } from 'react';
import { invoke } from '@tauri-apps/api/core';
import type { IconResult, UseFileIconOptions } from '../types/icons';

// Cache no lado do React
const iconCache = new Map<string, Promise<string>>();

// Debounce helper
function useDebounce<T>(value: T, delay: number): T {
  const [debouncedValue, setDebouncedValue] = useState<T>(value);

  useEffect(() => {
    const handler = setTimeout(() => {
      setDebouncedValue(value);
    }, delay);

    return () => {
      clearTimeout(handler);
    };
  }, [value, delay]);

  return debouncedValue;
}

export const useFileIcon = (
  filePath: string | null, 
  options: UseFileIconOptions = {}
) => {
  const { 
    size = 32, 
    fallbackIcon = '/assets/default-file-icon.svg', 
    enabled = true,
    debounceMs = 100
  } = options;
  
  const [iconSrc, setIconSrc] = useState<string>(fallbackIcon);
  const [isLoading, setIsLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  
  // Debounce do filePath para evitar muitas chamadas
  const debouncedFilePath = useDebounce(filePath, debounceMs);
  
  const cacheKey = useMemo(() => 
    debouncedFilePath ? `${debouncedFilePath}_${size}` : null, 
    [debouncedFilePath, size]
  );

  const fetchIcon = useCallback(async (path: string): Promise<string> => {
    if (!cacheKey) return fallbackIcon;

    // Verifica cache primeiro
    if (iconCache.has(cacheKey)) {
      return iconCache.get(cacheKey)!;
    }

    // Cria promise e adiciona ao cache
    const iconPromise = (async () => {
      try {
        const result = await invoke<IconResult>('get_file_icon', {
          path,
          size,
        });

        if (result.success && result.data) {
          return `data:image/png;base64,${result.data}`;
        } else {
          console.warn(`Failed to get icon for ${path}:`, result.error);
          return fallbackIcon;
        }
      } catch (err) {
        console.error(`Error fetching icon for ${path}:`, err);
        return fallbackIcon;
      }
    })();

    iconCache.set(cacheKey, iconPromise);
    return iconPromise;
  }, [cacheKey, size, fallbackIcon]);

  useEffect(() => {
    if (!debouncedFilePath || !enabled) {
      setIconSrc(fallbackIcon);
      setIsLoading(false);
      setError(null);
      return;
    }

    let cancelled = false;
    setIsLoading(true);
    setError(null);

    fetchIcon(debouncedFilePath)
      .then((icon) => {
        if (!cancelled) {
          setIconSrc(icon);
        }
      })
      .catch((err) => {
        if (!cancelled) {
          setError(err.message);
          setIconSrc(fallbackIcon);
        }
      })
      .finally(() => {
        if (!cancelled) {
          setIsLoading(false);
        }
      });

    return () => {
      cancelled = true;
    };
  }, [debouncedFilePath, fetchIcon, fallbackIcon, enabled]);

  const refresh = useCallback(() => {
    if (cacheKey) {
      iconCache.delete(cacheKey);
    }
    if (debouncedFilePath && enabled) {
      setIsLoading(true);
      fetchIcon(debouncedFilePath);
    }
  }, [cacheKey, debouncedFilePath, enabled, fetchIcon]);

  return {
    iconSrc,
    isLoading,
    error,
    refresh
  };
};