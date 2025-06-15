import { useState, useEffect, useCallback } from 'react';
import { invoke } from '@tauri-apps/api/core';
import type { BatchIconResult, UseFileIconOptions } from '../types/icons';

export const useFileIcons = (
  filePaths: string[], 
  options: UseFileIconOptions = {}
) => {
  const { 
    size = 32, 
    fallbackIcon = '/assets/default-file-icon.svg', 
    enabled = true 
  } = options;
  
  const [icons, setIcons] = useState<Record<string, string>>({});
  const [isLoading, setIsLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [stats, setStats] = useState({ successful: 0, failed: 0, total: 0 });

  const fetchIcons = useCallback(async () => {
    if (!enabled || filePaths.length === 0) {
      setIcons({});
      setStats({ successful: 0, failed: 0, total: 0 });
      return;
    }

    setIsLoading(true);
    setError(null);

    try {
      const result = await invoke<BatchIconResult>('get_file_icons_batch', {
        paths: filePaths,
        size,
      });

      const iconsMap: Record<string, string> = {};
      
      result.icons.forEach(({ path, success, data }) => {
        iconsMap[path] = success && data 
          ? `data:image/png;base64,${data}`
          : fallbackIcon;
      });

      setIcons(iconsMap);
      setStats({
        successful: result.successful,
        failed: result.failed,
        total: result.total_requested
      });

    } catch (err) {
      console.error('Error fetching batch icons:', err);
      setError(err instanceof Error ? err.message : 'Unknown error');
      
      // Fallback: todos com ícone padrão
      const fallbackIcons = filePaths.reduce((acc, path) => {
        acc[path] = fallbackIcon;
        return acc;
      }, {} as Record<string, string>);
      
      setIcons(fallbackIcons);
    } finally {
      setIsLoading(false);
    }
  }, [filePaths, size, fallbackIcon, enabled]);

  useEffect(() => {
    fetchIcons();
  }, [fetchIcons]);

  return {
    icons,
    isLoading,
    error,
    stats,
    refresh: fetchIcons
  };
};