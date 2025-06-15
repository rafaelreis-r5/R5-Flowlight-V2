import React from 'react';
import { Mail, AlertCircle } from 'lucide-react';

interface EmailStepProps {
  email: string;
  error?: string;
  isSubmitting: boolean;
  onChange: (e: React.ChangeEvent<HTMLInputElement>) => void;
  onNext: () => void;
}

export const EmailStep: React.FC<EmailStepProps> = ({
  email,
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
    <div className="space-y-4">
      <div className="relative">
        <div className="absolute inset-y-0 left-0 pl-3 flex items-center pointer-events-none">
          <Mail className={`h-5 w-5 ${error ? 'text-red-500' : 'text-gray-400'}`} />
        </div>
        <input
          type="email"
          name="email"
          value={email}
          onChange={onChange}
          onKeyDown={handleKeyDown}
          className={`block w-full pl-10 pr-10 py-3 bg-white/5 border ${
            error ? 'border-red-500' : 'border-white/10'
          } rounded-lg text-white placeholder-gray-500 focus:outline-none focus:ring-2 focus:ring-primary-purple/50 focus:border-transparent transition-colors`}
          placeholder="seu@email.com"
          autoComplete="email"
          autoFocus
          disabled={isSubmitting}
        />
        {error && (
          <div className="absolute inset-y-0 right-0 pr-3 flex items-center">
            <AlertCircle className="h-5 w-5 text-red-500" />
          </div>
        )}
      </div>
      
      {error ? (
        <p className="text-red-400 text-sm flex items-center">
          <AlertCircle className="w-4 h-4 mr-1" />
          {error}
        </p>
      ) : (
        <p className="text-sm text-gray-400">
          Usaremos este e-mail para acessar sua conta e enviar notificações importantes.
        </p>
      )}
    </div>
  );
};

export default EmailStep;
