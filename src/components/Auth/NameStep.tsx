import React from 'react';
import { User, AlertCircle } from 'lucide-react';

interface NameStepProps {
  name: string;
  error?: string;
  isSubmitting: boolean;
  onChange: (e: React.ChangeEvent<HTMLInputElement>) => void;
  onNext: () => void;
}

export const NameStep: React.FC<NameStepProps> = ({
  name,
  error,
  isSubmitting,
  onChange,
  onNext,
}) => {
  const handleKeyDown = (e: React.KeyboardEvent) => {
    if (e.key === 'Enter') {
      onNext();
    }
  };

  return (
    <div className="space-y-6">
      <div className="relative">
        <div className="absolute inset-y-0 left-0 pl-3 flex items-center pointer-events-none">
          <User className={`h-5 w-5 ${error ? 'text-red-500' : 'text-gray-400'}`} />
        </div>
        <input
          type="text"
          name="name"
          value={name}
          onChange={onChange}
          onKeyDown={handleKeyDown}
          className={`block w-full pl-10 pr-10 py-3 bg-white/5 border ${
            error ? 'border-red-500' : 'border-white/10'
          } rounded-lg text-white placeholder-gray-500 focus:outline-none focus:ring-2 focus:ring-primary-purple/50 focus:border-transparent transition-colors`}
          placeholder="Seu nome completo"
          autoComplete="name"
          autoFocus
          disabled={isSubmitting}
        />
      </div>
      {error && (
        <p className="text-red-400 text-sm flex items-center">
          <AlertCircle className="w-4 h-4 mr-1" />
          {error}
        </p>
      )}
      <p className="text-sm text-gray-400">
        Como você gostaria de ser chamado(a)?
      </p>
    </div>
  );
};

export default NameStep;
