// Tipos compartilhados relacionados a conteúdo
import React from 'react';
import { File, Folder, Package } from 'lucide-react';

export type ContentType = 'apps' | 'files' | 'system';

export const ALL_CONTENT_TYPES: ContentType[] = ['apps', 'files', 'system'];

export const CONTENT_TYPE_LABELS: Record<ContentType, string> = {
  apps: 'Aplicativos',
  files: 'Arquivos',
  system: 'Pastas do Sistema',
};

export const CONTENT_TYPE_DESCRIPTIONS: Record<ContentType, string> = {
  apps: 'Buscar aplicativos instalados no sistema',
  files: 'Buscar arquivos pessoais e documentos',
  system: 'Incluir pastas do sistema nos resultados',
};

export const CONTENT_TYPE_ICONS: Record<ContentType, React.ReactNode> = {
  apps: React.createElement(Package, { className: "w-4 h-4" }),
  files: React.createElement(File, { className: "w-4 h-4" }),
  system: React.createElement(Folder, { className: "w-4 h-4" }),
};
