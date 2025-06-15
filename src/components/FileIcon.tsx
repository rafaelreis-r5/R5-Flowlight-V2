import React from 'react';
import { useFileIcon } from '../hooks/useFileIcon';

interface FileIconProps {
  filePath: string;
  size?: number;
  className?: string;
  fallbackIcon?: string;
  showLoadingState?: boolean;
  onClick?: () => void;
}

export const FileIcon: React.FC<FileIconProps> = ({
  filePath,
  size = 32,
  className = '',
  fallbackIcon,
  showLoadingState = true,
  onClick
}) => {
  const { iconSrc, isLoading, error } = useFileIcon(filePath, { 
    size, 
    fallbackIcon 
  });

  const handleClick = () => {
    if (onClick) {
      onClick();
    }
  };

  return (
    <div 
      className={`inline-flex items-center justify-center ${className} ${onClick ? 'cursor-pointer' : ''}`}
      onClick={handleClick}
      title={error ? `Error loading icon: ${error}` : filePath}
    >
      {isLoading && showLoadingState ? (
        <div 
          className="animate-pulse bg-gray-200 rounded flex items-center justify-center"
          style={{ width: size, height: size }}
        >
          <div className="w-1/2 h-1/2 bg-gray-300 rounded"></div>
        </div>
      ) : (
        <img
          src={iconSrc}
          alt="File icon"
          className="object-contain"
          style={{ width: size, height: size }}
          loading="lazy"
          onError={(e) => {
            // Fallback final se tudo falhar
            const target = e.target as HTMLImageElement;
            if (target.src !== (fallbackIcon || '/assets/default-file-icon.svg')) {
              target.src = fallbackIcon || '/assets/default-file-icon.svg';
            }
          }}
        />
      )}
    </div>
  );
};