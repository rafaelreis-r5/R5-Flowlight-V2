import React from 'react';
import { Tooltip } from '../../Common/Tooltip';
import { Info, AlertCircle, Check, FileText, Image, Music, Video, FileArchive, Code, FileSpreadsheet, File, Mail } from 'lucide-react';

export interface AdvancedIndexingSettingsProps {
  enabled: boolean;
  maxFileSize: number;
  excludeHiddenFiles: boolean;
  indexFileMetadata: boolean;
  indexFileContent: boolean;
  contentTypesToIndex: {
    documents: boolean;
    images: boolean;
    audio: boolean;
    video: boolean;
    archives: boolean;
    code: boolean;
    spreadsheets: boolean;
    pdfs: boolean;
    emails: boolean;
  };
  indexSchedule: 'realtime' | 'hourly' | 'daily' | 'weekly' | 'monthly';
  lastIndexed: string | null;
  onToggle: (key: keyof Omit<AdvancedIndexingSettingsProps, 'onChange' | 'onToggle'>, value: any) => void;
  onChange: (key: keyof Omit<AdvancedIndexingSettingsProps, 'onChange' | 'onToggle'>, value: any) => void;
}

const fileTypeGroups = [
  {
    id: 'documents',
    label: 'Documentos',
    icon: <FileText className="w-4 h-4" />,
    description: 'Documentos de texto, apresentações e outros formatos de documento',
    extensions: ['.doc', '.docx', '.odt', '.rtf', '.txt', '.md']
  },
  {
    id: 'images',
    label: 'Imagens',
    icon: <Image className="w-4 h-4" />,
    description: 'Fotos e imagens em formatos comuns',
    extensions: ['.jpg', '.jpeg', '.png', '.gif', '.webp', '.svg', '.bmp', '.tiff']
  },
  {
    id: 'audio',
    label: 'Áudio',
    icon: <Music className="w-4 h-4" />,
    description: 'Arquivos de áudio em diferentes formatos',
    extensions: ['.mp3', '.wav', '.ogg', '.m4a', '.flac', '.aac']
  },
  {
    id: 'video',
    label: 'Vídeos',
    icon: <Video className="w-4 h-4" />,
    description: 'Arquivos de vídeo em vários formatos',
    extensions: ['.mp4', '.avi', '.mov', '.wmv', '.mkv', '.flv', '.webm']
  },
  {
    id: 'archives',
    label: 'Arquivos Compactados',
    icon: <FileArchive className="w-4 h-4" />,
    description: 'Arquivos compactados e pastas zipadas',
    extensions: ['.zip', '.rar', '.7z', '.tar', '.gz', '.bz2']
  },
  {
    id: 'code',
    label: 'Código-fonte',
    icon: <Code className="w-4 h-4" />,
    description: 'Arquivos de código-fonte e desenvolvimento',
    extensions: ['.js', '.jsx', '.ts', '.tsx', '.py', '.java', '.c', '.cpp', '.h', '.hpp', '.cs', '.go', '.rs', '.rb', '.php', '.sh']
  },
  {
    id: 'spreadsheets',
    label: 'Planilhas',
    icon: <FileSpreadsheet className="w-4 h-4" />,
    description: 'Arquivos de planilha e dados tabulares',
    extensions: ['.xls', '.xlsx', '.ods', '.csv', '.tsv']
  },
  {
    id: 'pdfs',
    label: 'PDFs',
    icon: <File className="w-4 h-4" />,
    description: 'Documentos em formato PDF',
    extensions: ['.pdf']
  },
  {
    id: 'emails',
    label: 'E-mails',
    icon: <Mail className="w-4 h-4" />,
    description: 'Arquivos de e-mail e mensagens',
    extensions: ['.eml', '.msg', '.pst', '.ost']
  },
];

export const AdvancedIndexingSettings: React.FC<AdvancedIndexingSettingsProps> = ({
  enabled,
  maxFileSize,
  excludeHiddenFiles,
  indexFileMetadata,
  indexFileContent,
  contentTypesToIndex,
  indexSchedule,
  lastIndexed,
  onToggle,
  onChange,
}) => {
  const handleToggleContentType = (type: string) => {
    onToggle('contentTypesToIndex', {
      ...contentTypesToIndex,
      [type]: !contentTypesToIndex[type as keyof typeof contentTypesToIndex]
    });
  };

  const handleFileSizeChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    const value = parseInt(e.target.value);
    if (!isNaN(value) && value >= 0) {
      onChange('maxFileSize', value);
    }
  };

  const handleScheduleChange = (e: React.ChangeEvent<HTMLSelectElement>) => {
    onChange('indexSchedule', e.target.value as any);
  };

  const getEnabledContentTypesCount = () => {
    return Object.values(contentTypesToIndex).filter(Boolean).length;
  };

  const getLastIndexedText = () => {
    if (!lastIndexed) return 'Nunca';
    
    const lastIndexedDate = new Date(lastIndexed);
    return new Intl.DateTimeFormat('pt-BR', {
      day: '2-digit',
      month: '2-digit',
      year: 'numeric',
      hour: '2-digit',
      minute: '2-digit'
    }).format(lastIndexedDate);
  };

  return (
    <div className="space-y-8">
      <div className="bg-gray-900/50 p-6 rounded-xl border border-gray-700/50">
        <div className="flex items-center justify-between mb-6">
          <div className="flex items-center space-x-2">
            <h2 className="text-xl font-semibold text-white">Indexação Avançada</h2>
            <Tooltip content="Configure opções avançadas de indexação para melhorar os resultados da busca">
              <Info className="w-4 h-4 text-gray-400 hover:text-gray-300 cursor-help" />
            </Tooltip>
          </div>
          <label className="relative inline-flex items-center cursor-pointer">
            <input 
              type="checkbox" 
              className="sr-only peer" 
              checked={enabled}
              onChange={() => onToggle('enabled', !enabled)}
            />
            <div className="w-11 h-6 bg-gray-700 rounded-full peer peer-checked:after:translate-x-full after:content-[''] after:absolute after:top-0.5 after:left-[2px] after:bg-white after:border-gray-300 after:border after:rounded-full after:h-5 after:w-5 after:transition-all peer-checked:bg-gradient-to-r peer-checked:from-[#6405d6] peer-checked:to-[#f1011d]"></div>
          </label>
        </div>

        {!enabled && (
          <div className="bg-yellow-500/10 border border-yellow-500/20 text-yellow-300 p-4 rounded-lg mb-6 flex items-start space-x-3">
            <AlertCircle className="w-5 h-5 mt-0.5 flex-shrink-0" />
            <p className="text-sm">A indexação avançada está desativada. Ative para configurar opções adicionais e melhorar os resultados da busca.</p>
          </div>
        )}

        <div className={`space-y-6 ${!enabled ? 'opacity-50 pointer-events-none' : ''}`}>
          <div>
            <div className="flex items-center justify-between mb-2">
              <label htmlFor="maxFileSize" className="text-sm font-medium text-gray-200">
                Tamanho máximo de arquivo: {maxFileSize} MB
              </label>
              <Tooltip content="Arquivos maiores que este tamanho serão ignorados na indexação">
                <Info className="w-4 h-4 text-gray-400 hover:text-gray-300 cursor-help" />
              </Tooltip>
            </div>
            <input
              id="maxFileSize"
              type="range"
              min="1"
              max="100"
              value={maxFileSize}
              onChange={handleFileSizeChange}
              className="w-full h-2 bg-gray-700 rounded-lg appearance-none cursor-pointer"
            />
            <div className="flex justify-between text-xs text-gray-400 mt-1">
              <span>1 MB</span>
              <span>100 MB</span>
            </div>
          </div>

          <div className="flex items-center justify-between">
            <div>
              <p className="text-sm font-medium text-gray-200">Ignorar arquivos ocultos</p>
              <p className="text-xs text-gray-400">Arquivos e pastas que começam com ponto (.)</p>
            </div>
            <label className="relative inline-flex items-center cursor-pointer">
              <input 
                type="checkbox" 
                className="sr-only peer" 
                checked={excludeHiddenFiles}
                onChange={() => onToggle('excludeHiddenFiles', !excludeHiddenFiles)}
              />
              <div className="w-11 h-6 bg-gray-700 rounded-full peer peer-checked:after:translate-x-full after:content-[''] after:absolute after:top-0.5 after:left-[2px] after:bg-white after:border-gray-300 after:border after:rounded-full after:h-5 after:w-5 after:transition-all peer-checked:bg-gradient-to-r peer-checked:from-[#6405d6] peer-checked:to-[#f1011d]"></div>
            </label>
          </div>

          <div className="flex items-center justify-between">
            <div>
              <p className="text-sm font-medium text-gray-200">Indexar metadados</p>
              <p className="text-xs text-gray-400">Extrair e indexar metadados dos arquivos</p>
            </div>
            <label className="relative inline-flex items-center cursor-pointer">
              <input 
                type="checkbox" 
                className="sr-only peer" 
                checked={indexFileMetadata}
                onChange={() => onToggle('indexFileMetadata', !indexFileMetadata)}
              />
              <div className="w-11 h-6 bg-gray-700 rounded-full peer peer-checked:after:translate-x-full after:content-[''] after:absolute after:top-0.5 after:left-[2px] after:bg-white after:border-gray-300 after:border after:rounded-full after:h-5 after:w-5 after:transition-all peer-checked:bg-gradient-to-r peer-checked:from-[#6405d6] peer-checked:to-[#f1011d]"></div>
            </label>
          </div>

          <div className="flex items-center justify-between">
            <div>
              <p className="text-sm font-medium text-gray-200">Indexar conteúdo dos arquivos</p>
              <p className="text-xs text-gray-400">Extrair e indexar o conteúdo de arquivos suportados</p>
            </div>
            <label className="relative inline-flex items-center cursor-pointer">
              <input 
                type="checkbox" 
                className="sr-only peer" 
                checked={indexFileContent}
                onChange={() => onToggle('indexFileContent', !indexFileContent)}
              />
              <div className="w-11 h-6 bg-gray-700 rounded-full peer peer-checked:after:translate-x-full after:content-[''] after:absolute after:top-0.5 after:left-[2px] after:bg-white after:border-gray-300 after:border after:rounded-full after:h-5 after:w-5 after:transition-all peer-checked:bg-gradient-to-r peer-checked:from-[#6405d6] peer-checked:to-[#f1011d]"></div>
            </label>
          </div>

          <div className="pt-4 border-t border-gray-700/50">
            <div className="flex items-center justify-between mb-4">
              <h3 className="text-sm font-medium text-white">Agendamento de Indexação</h3>
              <Tooltip content="Frequência com que o sistema irá atualizar o índice de busca">
                <Info className="w-4 h-4 text-gray-400 hover:text-gray-300 cursor-help" />
              </Tooltip>
            </div>
            <select
              value={indexSchedule}
              onChange={handleScheduleChange}
              className="w-full bg-gray-800 border border-gray-700 text-white text-sm rounded-lg focus:ring-2 focus:ring-indigo-500 focus:border-indigo-500 block p-2.5"
            >
              <option value="realtime">Tempo Real</option>
              <option value="hourly">A cada Hora</option>
              <option value="daily">Diariamente</option>
              <option value="weekly">Semanalmente</option>
              <option value="monthly">Mensalmente</option>
            </select>
            <p className="text-xs text-gray-400 mt-2">
              Última indexação: <span className="text-white">{getLastIndexedText()}</span>
            </p>
          </div>

          <div className="pt-4 border-t border-gray-700/50">
            <div className="flex items-center justify-between mb-4">
              <div>
                <h3 className="text-sm font-medium text-white">Tipos de Arquivo</h3>
                <p className="text-xs text-gray-400">
                  {getEnabledContentTypesCount()} de {Object.keys(contentTypesToIndex).length} tipos ativos
                </p>
              </div>
              <Tooltip content="Selecione os tipos de arquivo que devem ser incluídos na indexação">
                <Info className="w-4 h-4 text-gray-400 hover:text-gray-300 cursor-help" />
              </Tooltip>
            </div>
            
            <div className="grid grid-cols-1 md:grid-cols-2 gap-3">
              {fileTypeGroups.map(({ id, label, icon, description, extensions }) => (
                <Tooltip key={id} content={description} position="right">
                  <div 
                    className={`flex items-center p-3 rounded-lg cursor-pointer transition-colors ${
                      contentTypesToIndex[id as keyof typeof contentTypesToIndex]
                        ? 'bg-indigo-500/10 border border-indigo-500/20 hover:bg-indigo-500/15'
                        : 'bg-gray-800/50 hover:bg-gray-700/50'
                    }`}
                    onClick={() => handleToggleContentType(id)}
                  >
                    <div className="flex items-center space-x-3 flex-1">
                      <div 
                        className={`p-1.5 rounded-md ${
                          contentTypesToIndex[id as keyof typeof contentTypesToIndex]
                            ? 'bg-indigo-500/20 text-indigo-400'
                            : 'bg-gray-700 text-gray-400'
                        }`}
                      >
                        {icon}
                      </div>
                      <div>
                        <p className={`text-sm font-medium ${
                          contentTypesToIndex[id as keyof typeof contentTypesToIndex] 
                            ? 'text-indigo-100' 
                            : 'text-white'
                        }`}>
                          {label}
                        </p>
                        <p className="text-xs text-gray-400 truncate max-w-[180px]">
                          {extensions.join(', ')}
                        </p>
                      </div>
                    </div>
                    {contentTypesToIndex[id as keyof typeof contentTypesToIndex] && (
                      <div className="p-1 rounded-full bg-indigo-500 text-white">
                        <Check className="w-3.5 h-3.5" />
                      </div>
                    )}
                  </div>
                </Tooltip>
              ))}
            </div>
          </div>
        </div>
      </div>
    </div>
  );
};
