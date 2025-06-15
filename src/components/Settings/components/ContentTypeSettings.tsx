import React from 'react';
import { Check, AlertCircle } from 'lucide-react';
import { Tooltip } from '../../Common';
import { 
  ContentType, 
  CONTENT_TYPE_LABELS, 
  CONTENT_TYPE_DESCRIPTIONS, 
  CONTENT_TYPE_ICONS,
  ALL_CONTENT_TYPES 
} from '../../../types/content';

interface ContentTypeSettingsProps {
  enabledTypes: ContentType[];
  onChange: (enabledTypes: ContentType[]) => void;
}

const contentTypes = ALL_CONTENT_TYPES.map(id => ({
  id,
  label: CONTENT_TYPE_LABELS[id],
  icon: CONTENT_TYPE_ICONS[id],
  description: CONTENT_TYPE_DESCRIPTIONS[id]
}));

export const ContentTypeSettings: React.FC<ContentTypeSettingsProps> = ({
  enabledTypes = [],
  onChange,
}) => {
  const toggleContentType = (type: ContentType) => {
    const newEnabledTypes = enabledTypes.includes(type)
      ? enabledTypes.filter(t => t !== type)
      : [...enabledTypes, type];
    
    onChange(newEnabledTypes);
  };

  const allDisabled = enabledTypes.length === 0;

  return (
    <div className="space-y-4">
      <div>
        <div className="flex items-center justify-between">
          <h3 className="text-lg font-medium text-white">Tipos de Conteúdo</h3>
          {allDisabled && (
            <span className="inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium bg-yellow-100 text-yellow-800">
              <AlertCircle className="w-3 h-3 mr-1" />
              Selecione ao menos um tipo
            </span>
          )}
        </div>
        <p className="text-sm text-gray-400 mt-1 mb-4">
          Selecione os tipos de conteúdo que devem ser incluídos nos resultados de busca
        </p>
      </div>

      <div className="space-y-3">
        {contentTypes.map((type) => (
          <Tooltip key={type.id} content={type.description} position="right">
            <div 
              className={`flex items-center justify-between p-3 rounded-lg transition-colors cursor-pointer ${
                enabledTypes.includes(type.id)
                  ? 'bg-indigo-500/10 border border-indigo-500/20 hover:bg-indigo-500/15'
                  : 'bg-gray-800/50 hover:bg-gray-700/50'
              }`}
              onClick={() => toggleContentType(type.id)}
            >
              <div className="flex items-center space-x-3">
                <div 
                  className={`p-1.5 rounded-md ${
                    enabledTypes.includes(type.id)
                      ? 'bg-indigo-500/20 text-indigo-400'
                      : 'bg-gray-700 text-gray-400'
                  }`}
                >
                  {type.icon}
                </div>
                <span className={enabledTypes.includes(type.id) ? 'text-indigo-100' : 'text-white'}>
                  {type.label}
                </span>
              </div>
              {enabledTypes.includes(type.id) ? (
                <div className="p-1 rounded-full bg-indigo-500 text-white">
                  <Check className="w-3.5 h-3.5" />
                </div>
              ) : (
                <div className="w-5 h-5 rounded-full border-2 border-gray-600" />
              )}
            </div>
          </Tooltip>
        ))}
      </div>

      {enabledTypes.length === 0 && (
        <p className="text-sm text-yellow-400 mt-2">
          Selecione pelo menos um tipo de conteúdo para habilitar a busca
        </p>
      )}
    </div>
  );
};
