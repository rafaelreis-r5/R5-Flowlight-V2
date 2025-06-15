import React, { useCallback, useEffect } from 'react';
import { motion, AnimatePresence } from 'framer-motion';
import { X, ArrowLeft, Loader2 } from 'lucide-react';
import { useSignUpForm } from './useSignUpForm';
import { ProgressDots as ProgressDotsComponent } from '../ProgressDots';
import { NameStep } from '../NameStep';
import { EmailStep } from '../EmailStep';
import { PasswordStep } from '../PasswordStep';
import { SuccessStep } from '../SuccessStep';

interface SignUpFlowProps {
  onClose: () => void;
  initialEmail?: string;
}

export const SignUpFlow: React.FC<SignUpFlowProps> = ({
  onClose,
  initialEmail = '',
}) => {
  const {
    // Estado
    currentStep,
    formData,
    errors,
    isSubmitting,
    
    // Ações
    updateFormData,
    nextStep,
    prevStep,
    submitForm,
  } = useSignUpForm(initialEmail);
  
  const currentStepIndex = ['name', 'email', 'password'].indexOf(currentStep);
  const totalSteps = 3;

  // Manipulador de teclado para navegação
  const handleKeyDown = useCallback((e: React.KeyboardEvent) => {
    if (e.key === 'Escape') {
      onClose();
    } else if (e.key === 'Enter' && currentStep !== 'password') {
      e.preventDefault();
      nextStep();
    }
  }, [currentStep, nextStep, onClose]);

  // Efeito para fechar o modal ao pressionar ESC
  useEffect(() => {
    const handleEsc = (e: KeyboardEvent) => {
      if (e.key === 'Escape') {
        onClose();
      }
    };

    window.addEventListener('keydown', handleEsc);
    return () => window.removeEventListener('keydown', handleEsc);
  }, [onClose]);

  // Manipulador de envio do formulário
  const handleSubmit = useCallback(async () => {
    const success = await submitForm();
    if (success) {
      // Fecha o modal após 2 segundos
      setTimeout(() => {
        onClose();
      }, 2000);
    }
  }, [submitForm, onClose]);

  // Se o cadastro foi concluído com sucesso
  if (currentStep === 'success') {
    return <SuccessStep />;
  }

  return (
    <div 
      className="fixed inset-0 z-50 flex items-center justify-center bg-black/70 backdrop-blur-sm p-4"
      role="dialog"
      aria-modal="true"
      onKeyDown={handleKeyDown}
      tabIndex={-1}
    >
      <motion.div 
        className="relative bg-[#1a0a2e] rounded-2xl w-full max-w-md border border-white/10 overflow-hidden shadow-2xl"
        initial={{ opacity: 0, scale: 0.95, y: 20 }}
        animate={{ opacity: 1, scale: 1, y: 0 }}
        exit={{ opacity: 0, scale: 0.95, y: 20 }}
        transition={{ type: 'spring', damping: 25, stiffness: 300 }}
      >
        {/* Cabeçalho */}
        <div className="p-4 border-b border-white/10 flex items-center justify-between">
          <button 
            onClick={currentStep === 'name' ? onClose : prevStep}
            className="p-1.5 rounded-full hover:bg-white/10 text-gray-400 hover:text-white transition-colors"
            aria-label={currentStep === 'name' ? 'Fechar' : 'Voltar'}
            disabled={isSubmitting}
          >
            {currentStep === 'name' ? <X className="w-5 h-5" /> : <ArrowLeft className="w-5 h-5" />}
          </button>
          
          <h2 className="text-lg font-medium text-white">
            {currentStep === 'name' ? 'Qual é o seu nome?' : 
             currentStep === 'email' ? 'Qual é o seu e-mail?' : 
             'Crie uma senha segura'}
          </h2>
          
          <div className="w-8"></div> {/* Espaçador para centralizar o título */}
        </div>

        {/* Indicador de progresso */}
        <ProgressDotsComponent atual={currentStepIndex} total={totalSteps} />

        {/* Conteúdo do formulário */}
        <div className="p-6">
          <AnimatePresence mode="wait">
            <motion.div
              key={currentStep}
              initial={{ opacity: 0, x: 20 }}
              animate={{ opacity: 1, x: 0 }}
              exit={{ opacity: 0, x: -20 }}
              transition={{ duration: 0.2 }}
              className="space-y-6"
            >
              {/* Passo 1: Nome */}
              {currentStep === 'name' && (
                <NameStep 
                  name={formData.name}
                  error={errors.name}
                  isSubmitting={isSubmitting}
                  onChange={(e) => updateFormData('name', e.target.value)}
                  onNext={nextStep}
                />
              )}

              {/* Passo 2: E-mail */}
              {currentStep === 'email' && (
                <EmailStep 
                  email={formData.email}
                  error={errors.email}
                  isSubmitting={isSubmitting}
                  onChange={(e) => updateFormData('email', e.target.value)}
                  onNext={nextStep}
                />
              )}

              {/* Passo 3: Senha */}
              {currentStep === 'password' && (
                <PasswordStep 
                  formData={{
                    password: formData.password,
                    confirmPassword: formData.confirmPassword
                  }}
                  errors={{
                    password: errors.password,
                    confirmPassword: errors.confirmPassword
                  }}
                  isSubmitting={isSubmitting}
                  handleChange={(e) => {
                    const field = e.target.name as 'password' | 'confirmPassword';
                    updateFormData(field, e.target.value);
                  }}
                  handleSubmit={handleSubmit}
                />
              )}
            </motion.div>
          </AnimatePresence>
        </div>

        {/* Rodapé com botão de próximo/enviar */}
        <div className="p-4 border-t border-white/10">
          <button
            onClick={currentStep === 'password' ? handleSubmit : nextStep}
            disabled={isSubmitting}
            className="w-full py-3 px-4 bg-gradient-to-r from-primary-purple to-accent-red text-white font-medium rounded-lg hover:opacity-90 disabled:opacity-50 disabled:cursor-not-allowed transition-opacity flex items-center justify-center"
          >
            {isSubmitting ? (
              <>
                <Loader2 className="w-4 h-4 mr-2 animate-spin" />
                {currentStep === 'password' ? 'Cadastrando...' : 'Carregando...'}
              </>
            ) : currentStep === 'password' ? (
              'Finalizar cadastro'
            ) : (
              'Continuar'
            )}
          </button>
        </div>
      </motion.div>
    </div>
  );
};

export default SignUpFlow;
