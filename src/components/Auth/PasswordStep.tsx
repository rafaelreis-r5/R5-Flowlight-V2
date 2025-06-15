import React, { useState } from 'react';
import { Eye, EyeOff, Lock, AlertCircle, Check } from 'lucide-react';
import { motion } from 'framer-motion';

interface PasswordStepProps {
  formData: {
    password: string;
    confirmPassword: string;
  };
  errors: {
    password?: string;
    confirmPassword?: string;
  };
  handleChange: (e: React.ChangeEvent<HTMLInputElement>) => void;
  handleSubmit: () => void;
  isSubmitting: boolean;
}

export const PasswordStep: React.FC<PasswordStepProps> = ({
  formData,
  errors,
  handleChange,
  handleSubmit,
  isSubmitting,
}) => {
  const [showPassword, setShowPassword] = useState(false);
  const [showConfirmPassword, setShowConfirmPassword] = useState(false);

  const handleKeyDown = (e: React.KeyboardEvent) => {
    if (e.key === 'Enter') {
      handleSubmit();
    }
  };

  return (
    <div className="space-y-6">
      <div className="space-y-4">
        <div className="relative">
          <div className="absolute inset-y-0 left-0 pl-3 flex items-center pointer-events-none">
            <Lock className={`h-5 w-5 ${errors.password ? 'text-red-500' : 'text-gray-400'}`} />
          </div>
          <input
            type={showPassword ? 'text' : 'password'}
            name="password"
            value={formData.password}
            onChange={handleChange}
            onKeyDown={handleKeyDown}
            className={`block w-full pl-10 pr-10 py-3 bg-white/5 border ${
              errors.password ? 'border-red-500' : 'border-white/10'
            } rounded-lg text-white placeholder-gray-500 focus:outline-none focus:ring-2 focus:ring-primary-purple/50 focus:border-transparent transition-colors`}
            placeholder="Sua senha"
            autoComplete="new-password"
            autoFocus
            disabled={isSubmitting}
          />
          <button
            type="button"
            onClick={() => setShowPassword(!showPassword)}
            className="absolute inset-y-0 right-0 pr-3 flex items-center text-gray-400 hover:text-white transition-colors"
            aria-label={showPassword ? 'Ocultar senha' : 'Mostrar senha'}
          >
            {showPassword ? (
              <EyeOff className="h-5 w-5" />
            ) : (
              <Eye className="h-5 w-5" />
            )}
          </button>
        </div>
        
        <div className="relative">
          <div className="absolute inset-y-0 left-0 pl-3 flex items-center pointer-events-none">
            <Lock className={`h-5 w-5 ${errors.confirmPassword ? 'text-red-500' : 'text-gray-400'}`} />
          </div>
          <input
            type={showConfirmPassword ? 'text' : 'password'}
            name="confirmPassword"
            value={formData.confirmPassword}
            onChange={handleChange}
            onKeyDown={handleKeyDown}
            className={`block w-full pl-10 pr-10 py-3 bg-white/5 border ${
              errors.confirmPassword ? 'border-red-500' : 'border-white/10'
            } rounded-lg text-white placeholder-gray-500 focus:outline-none focus:ring-2 focus:ring-primary-purple/50 focus:border-transparent transition-colors`}
            placeholder="Confirme sua senha"
            autoComplete="new-password"
            disabled={isSubmitting}
          />
          <button
            type="button"
            onClick={() => setShowConfirmPassword(!showConfirmPassword)}
            className="absolute inset-y-0 right-0 pr-3 flex items-center text-gray-400 hover:text-white transition-colors"
            aria-label={showConfirmPassword ? 'Ocultar confirmação de senha' : 'Mostrar confirmação de senha'}
          >
            {showConfirmPassword ? (
              <EyeOff className="h-5 w-5" />
            ) : (
              <Eye className="h-5 w-5" />
            )}
          </button>
        </div>
        
        {(errors.password || errors.confirmPassword) && (
          <div className="text-red-400 text-sm flex items-start">
            <AlertCircle className="w-4 h-4 mr-1 mt-0.5 flex-shrink-0" />
            <span>{errors.password || errors.confirmPassword}</span>
          </div>
        )}
      </div>
      
      <div className="bg-white/5 p-4 rounded-lg border border-white/10">
        <p className="text-sm font-medium text-gray-300 mb-2">Sua senha deve conter:</p>
        <ul className="text-xs text-gray-400 space-y-1">
          <li className={`flex items-center ${formData.password.length >= 6 ? 'text-green-400' : ''}`}>
            {formData.password.length >= 6 ? (
              <Check className="w-3.5 h-3.5 mr-1.5 flex-shrink-0" />
            ) : (
              <span className="w-3.5 h-3.5 mr-1.5 flex items-center justify-center">•</span>
            )}
            Pelo menos 6 caracteres
          </li>
          <li className={`flex items-center ${/[A-Z]/.test(formData.password) ? 'text-green-400' : ''}`}>
            {/[A-Z]/.test(formData.password) ? (
              <Check className="w-3.5 h-3.5 mr-1.5 flex-shrink-0" />
            ) : (
              <span className="w-3.5 h-3.5 mr-1.5 flex items-center justify-center">•</span>
            )}
            Pelo menos 1 letra maiúscula
          </li>
          <li className={`flex items-center ${/[0-9]/.test(formData.password) ? 'text-green-400' : ''}`}>
            {/[0-9]/.test(formData.password) ? (
              <Check className="w-3.5 h-3.5 mr-1.5 flex-shrink-0" />
            ) : (
              <span className="w-3.5 h-3.5 mr-1.5 flex items-center justify-center">•</span>
            )}
            Pelo menos 1 número
          </li>
        </ul>
      </div>
    </div>
  );
};

export default PasswordStep;
