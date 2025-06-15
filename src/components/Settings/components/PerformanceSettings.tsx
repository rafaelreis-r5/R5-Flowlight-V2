import React, { useState } from 'react';

export interface PerformanceSettingsType {
  hardwareAcceleration: boolean;
  backgroundThrottling: boolean;
  maxConcurrent: number;
  cacheSize: number;
}

interface PerformanceSettingsProps {
  settings: PerformanceSettingsType;
  onChange: <K extends keyof PerformanceSettingsType>(
    key: K, 
    value: PerformanceSettingsType[K]
  ) => void;
}

export const PerformanceSettings: React.FC<PerformanceSettingsProps> = ({ settings, onChange }) => {
  const [showAdvanced, setShowAdvanced] = useState(false);
  const [isOptimizing, setIsOptimizing] = useState(false);
  const [optimizationProgress, setOptimizationProgress] = useState(0);

  const handleToggle = (key: keyof PerformanceSettingsType, value: boolean) => {
    onChange(key as any, value);
  };

  const handleSliderChange = (key: keyof PerformanceSettingsType, value: number) => {
    onChange(key as any, value);
  };

  const startOptimization = () => {
    setIsOptimizing(true);
    setOptimizationProgress(0);
    
    // Simular processo de otimização
    const interval = setInterval(() => {
      setOptimizationProgress(prev => {
        const newProgress = prev + Math.floor(Math.random() * 10) + 5;
        if (newProgress >= 100) {
          clearInterval(interval);
          setTimeout(() => {
            setIsOptimizing(false);
            setOptimizationProgress(0);
          }, 500);
          return 100;
        }
        return newProgress;
      });
    }, 200);
  };

  // Usar as propriedades do settings
  const { 
    hardwareAcceleration,
    backgroundThrottling,
    maxConcurrent,
    cacheSize
  } = settings;

  return (
    <div className="space-y-6">
      <div>
        <h3 className="text-lg font-semibold text-white mb-4">Otimização de Desempenho</h3>
        <p className="text-sm text-gray-400 mb-6">
          Ajuste as configurações de desempenho para otimizar o uso de recursos do sistema.
        </p>
        <div className="space-y-6">
          <div className="bg-[#1a0a2e] p-4 rounded-lg border border-[#2a1a3a]">
            <h4 className="text-md font-medium text-gray-200 mb-4">Otimização do Sistema</h4>
            
            <div className="space-y-4">
              <div>
                <div className="flex justify-between mb-1">
                  <span className="text-sm font-medium text-gray-200">Uso de Memória</span>
                  <span className="text-xs text-gray-400">Moderado</span>
                </div>
                <div className="w-full bg-gray-700 rounded-full h-2">
                  <div 
                    className="bg-gradient-to-r from-[#6405d6] to-[#f1011d] h-2 rounded-full" 
                    style={{ width: '65%' }}
                  ></div>
                </div>
              </div>

              <div>
                <div className="flex justify-between mb-1">
                  <span className="text-sm font-medium text-gray-200">Uso de CPU</span>
                  <span className="text-xs text-gray-400">Baixo</span>
                </div>
                <div className="w-full bg-gray-700 rounded-full h-2">
                  <div 
                    className="bg-gradient-to-r from-[#6405d6] to-[#f1011d] h-2 rounded-full" 
                    style={{ width: '25%' }}
                  ></div>
                </div>
              </div>

              <div className="pt-2">
                <button
                  onClick={startOptimization}
                  disabled={isOptimizing}
                  className={`w-full py-2 px-4 rounded-md text-sm font-medium text-white ${
                    isOptimizing 
                      ? 'bg-gray-600 cursor-not-allowed' 
                      : 'bg-gradient-to-r from-[#6405d6] to-[#f1011d] hover:opacity-90'
                  }`}
                >
                  {isOptimizing ? (
                    <div className="flex items-center justify-center">
                      <svg className="animate-spin -ml-1 mr-2 h-4 w-4 text-white" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
                        <circle className="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" strokeWidth="4"></circle>
                        <path className="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
                      </svg>
                      {optimizationProgress > 0 ? `Otimizando... ${optimizationProgress}%` : 'Otimizando...'}
                    </div>
                  ) : 'Otimizar Agora'}
                </button>
                
                {isOptimizing && optimizationProgress > 0 && (
                  <div className="mt-2">
                    <div className="w-full bg-gray-700 rounded-full h-1.5 mt-2">
                      <div 
                        className="bg-blue-500 h-1.5 rounded-full transition-all duration-300 ease-out" 
                        style={{ width: `${optimizationProgress}%` }}
                      ></div>
                    </div>
                  </div>
                )}
              </div>
            </div>
          </div>

          <div className="space-y-4">
            <h4 className="text-md font-medium text-gray-200">Configurações de Desempenho</h4>
            
            <div className="space-y-4">
              <div className="flex items-start">
                <div className="flex items-center h-5">
                  <input
                    id="hardware-acceleration"
                    name="hardware-acceleration"
                    type="checkbox"
                    className="focus:ring-[#6405d6] h-4 w-4 text-[#6405d6] border-[#2a1a3a] rounded"
                    checked={hardwareAcceleration}
                    onChange={(e) => handleToggle('hardwareAcceleration', e.target.checked)}
                  />
                </div>
                <div className="ml-3 text-sm">
                  <label htmlFor="hardware-acceleration" className="font-medium text-gray-200">
                    Aceleração de hardware
                  </label>
                  <p className="text-gray-400">
                    Melhora o desempenho de renderização usando a GPU
                  </p>
                </div>
              </div>

              <div className="flex items-start">
                <div className="flex items-center h-5">
                  <input
                    id="background-throttling"
                    name="background-throttling"
                    type="checkbox"
                    className="focus:ring-[#6405d6] h-4 w-4 text-[#6405d6] border-[#2a1a3a] rounded"
                    checked={backgroundThrottling}
                    onChange={(e) => handleToggle('backgroundThrottling', e.target.checked)}
                  />
                </div>
                <div className="ml-3 text-sm">
                  <label htmlFor="background-throttling" className="font-medium text-gray-200">
                    Limitar quando em segundo plano
                  </label>
                  <p className="text-gray-400">
                    Reduz o uso de recursos quando a janela não está em foco
                  </p>
                </div>
              </div>

              <button 
                onClick={() => setShowAdvanced(!showAdvanced)}
                className="text-sm text-[#6405d6] hover:text-[#7a1ae6] flex items-center"
              >
                {showAdvanced ? 'Ocultar opções avançadas' : 'Mostrar opções avançadas'}
                <svg 
                  className={`ml-1 h-4 w-4 transform transition-transform ${showAdvanced ? 'rotate-180' : ''}`} 
                  fill="none" 
                  viewBox="0 0 24 24" 
                  stroke="currentColor"
                >
                  <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M19 9l-7 7-7-7" />
                </svg>
              </button>

              {showAdvanced && (
                <div className="bg-[#1a0a2e] p-4 rounded-lg border border-[#2a1a3a] space-y-4">
                  <div>
                    <label htmlFor="max-concurrent" className="block text-sm font-medium text-gray-200 mb-1">
                      Máximo de processos simultâneos: <span className="text-[#6405d6]">4</span>
                    </label>
                    <input
                      type="range"
                      id="max-concurrent"
                      min="1"
                      max="8"
                      step="1"
                      value={maxConcurrent}
                      onChange={(e) => handleSliderChange('maxConcurrent', parseInt(e.target.value, 10))}
                      className="w-full h-2 bg-gray-700 rounded-lg appearance-none cursor-pointer"
                    />
                    <div className="flex justify-between text-xs text-gray-400 mt-1">
                      <span>1</span>
                      <span>2</span>
                      <span>3</span>
                      <span>4</span>
                      <span>5</span>
                      <span>6</span>
                      <span>7</span>
                      <span>8</span>
                    </div>
                  </div>

                  <div>
                    <label htmlFor="cache-size" className="block text-sm font-medium text-gray-200 mb-1">
                      Tamanho máximo do cache: <span className="text-[#6405d6]">500 MB</span>
                    </label>
                    <input
                      type="range"
                      id="cache-size"
                      min="100"
                      max="2000"
                      step="100"
                      value={cacheSize}
                      onChange={(e) => handleSliderChange('cacheSize', parseInt(e.target.value, 10))}
                      className="w-full h-2 bg-gray-700 rounded-lg appearance-none cursor-pointer"
                    />
                    <div className="flex justify-between text-xs text-gray-400 mt-1">
                      <span>100MB</span>
                      <span>500MB</span>
                      <span>1GB</span>
                      <span>1.5GB</span>
                      <span>2GB</span>
                    </div>
                  </div>

                  <div className="pt-2">
                    <button 
                      onClick={() => {
                        // Lógica para aplicar configurações avançadas
                        console.log('Configurações aplicadas:', { 
                          maxConcurrent, 
                          cacheSize, 
                          hardwareAcceleration 
                        });
                      }}
                      className="w-full py-2 px-4 bg-[#6405d6] hover:bg-[#7a1ae6] text-white text-sm font-medium rounded-md transition-colors"
                    >
                      Aplicar Configurações Avançadas
                    </button>
                  </div>
                </div>
              )}
            </div>
          </div>
        </div>
      </div>

      <div className="bg-[#1a0a2e] border-l-4 border-yellow-500 p-4 rounded-md">
        <div className="flex">
          <div className="flex-shrink-0">
            <svg className="h-5 w-5 text-yellow-400" viewBox="0 0 20 20" fill="currentColor">
              <path fillRule="evenodd" d="M8.257 3.099c.765-1.36 2.722-1.36 3.486 0l5.58 9.92c.75 1.334-.213 2.98-1.742 2.98H4.42c-1.53 0-2.493-1.646-1.743-2.98l5.58-9.92zM11 13a1 1 0 11-2 0 1 1 0 012 0zm-1-8a1 1 0 00-1 1v3a1 1 0 002 0V6a1 1 0 00-1-1z" clipRule="evenodd" />
            </svg>
          </div>
          <div className="ml-3">
            <h3 className="text-sm font-medium text-yellow-200">Aviso de Desempenho</h3>
            <div className="mt-2 text-sm text-yellow-100">
              <p>
                Alterações nas configurações avançadas podem afetar significativamente o desempenho e a estabilidade do aplicativo. 
                Modifique apenas se souber o que está fazendo.
              </p>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
};
