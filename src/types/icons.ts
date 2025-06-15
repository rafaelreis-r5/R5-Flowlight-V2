export interface IconResult {
  success: boolean;
  data?: string;
  error?: string;
  size: number;
  path: string;
}

export interface BatchIconResult {
  icons: IconResult[];
  total_requested: number;
  successful: number;
  failed: number;
}

export interface UseFileIconOptions {
  size?: number;
  fallbackIcon?: string;
  enabled?: boolean;
  debounceMs?: number;
}

export interface CacheStats {
  frontend: number;
  backend: {
    size: number;
    keys: string[];
  } | null;
}