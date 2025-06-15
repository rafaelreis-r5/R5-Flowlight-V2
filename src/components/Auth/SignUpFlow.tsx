import React, { useState, useCallback } from 'react';
import { motion, AnimatePresence } from 'framer-motion';
import { X, ArrowLeft, User, Mail, Lock, Check, Eye, EyeOff, Loader2, AlertCircle } from 'lucide-react';
import { useAuth } from '../../contexts/AuthContext';
import { toast } from 'react-hot-toast';
import { z } from 'zod';

// Esquema base sem a validação de confirmação de senha
const baseSignUpSchema = z.object({
  name: z.string()
    .min(3, { message: 'O nome deve ter pelo menos 3 caracteres' })
    .max(100, { message: 'O nome deve ter no máximo 100 caracteres' })
    .regex(/^[\p{L}\s-]+$/u, { message: 'O nome deve conter apenas letras e espaços' }),
  email: z.string()
    .email({ message: 'Por favor, insira um e-mail válido' })
    .max(100, { message: 'O e-mail deve ter no máximo 100 caracteres' }),
  password: z.string()
    .min(6, { message: 'A senha deve ter pelo menos 6 caracteres' })
    .max(50, { message: 'A senha deve ter no máximo 50 caracteres' })
    .regex(/[A-Z]/, { message: 'A senha deve conter pelo menos uma letra maiúscula' })
    .regex(/[a-z]/, { message: 'A senha deve conter pelo menos uma letra minúscula' })
    .regex(/[0-9]/, { message: 'A senha deve conter pelo menos um número' }),
  confirmPassword: z.string()
});

// Esquema completo com validação de confirmação de senha
const signUpSchema = baseSignUpSchema.refine(
  (data) => data.password === data.confirmPassword,
  {
    message: 'As senhas não conferem',
    path: ['confirmPassword']
  }
);

type FormData = z.infer<typeof signUpSchema>;

interface SignUpFlowProps {
  onClose: () => void;
  onSwitchToLogin: (email?: string) => void;
  onSuccess: () => void;
}

const STEPS = ['name', 'email', 'password'] as const;
type Step = typeof STEPS[number];

// Componente de indicador de progresso
interface ProgressDotsProps {
  current: number;
  total: number;
}

const ProgressDots = ({ current, total }: ProgressDotsProps) => {
  return (
    <div className="flex justify-center mt-2 space-x-2">
      {Array.from({ length: total }).map((_, index) => (
        <div
          key={index}
          className={`h-1.5 rounded-full transition-all duration-300 ${
            index <= current
              ? 'bg-gradient-to-r from-primary-purple to-accent-red w-4'
              : 'bg-white/20 w-1.5'
          }`}
          aria-hidden="true"
        />
      ))}
    </div>
  );
};

export function SignUpFlow({ onClose, onSwitchToLogin, onSuccess }: SignUpFlowProps) {
  const { completeFirstAccess } = useAuth();
  const [currentStep, setCurrentStep] = useState<Step>('name');
  const [formData, setFormData] = useState<FormData>({
    name: '',
    email: '',
    password: '',
    confirmPassword: ''
  });
  const [errors, setErrors] = useState<Partial<Record<keyof FormData, string>>>({});
  const [isSubmitting, setIsSubmitting] = useState(false);
  const [showPassword, setShowPassword] = useState<boolean>(false);
  const [showConfirmPassword, setShowConfirmPassword] = useState<boolean>(false);
  const [signUpSuccess, setSignUpSuccess] = useState(false);

  // Valida o passo atual
  const validateCurrentStep = useCallback(() => {
    try {
      if (currentStep === 'name') {
        baseSignUpSchema.pick({ name: true }).parse({ name: formData.name });
      } else if (currentStep === 'email') {
        baseSignUpSchema.pick({ email: true }).parse({ email: formData.email });
      } else if (currentStep === 'password') {
        signUpSchema.pick({ password: true, confirmPassword: true }).parse({
          password: formData.password,
          confirmPassword: formData.confirmPassword
        });
      }
      return true;
    } catch (error) {
      if (error instanceof z.ZodError) {
        const newErrors: Partial<Record<keyof FormData, string>> = {};
        error.errors.forEach((err) => {
          const path = err.path[0] as keyof FormData;
          newErrors[path] = err.message;
        });
        setErrors(newErrors);
      }
      return false;
    }
  }, [currentStep, formData]);

  // Navega para um passo específico
  const goToStep = useCallback((step: Step) => {
    setCurrentStep(step);
  }, []);

  // Avança para o próximo passo
  const handleNext = useCallback(() => {
    if (validateCurrentStep()) {
      if (currentStep === 'name') {
        goToStep('email');
      } else if (currentStep === 'email') {
        goToStep('password');
      }
    }
  }, [currentStep, goToStep, validateCurrentStep]);

  // Volta para o passo anterior
  const handleBack = useCallback(() => {
    if (currentStep === 'email') {
      goToStep('name');
    } else if (currentStep === 'password') {
      goToStep('email');
    }
  }, [currentStep, goToStep]);

  // Manipulador de mudança nos campos do formulário
  const handleChange = useCallback((e: React.ChangeEvent<HTMLInputElement>) => {
    const { name, value } = e.target;
    setFormData(prev => ({
      ...prev,
      [name]: value
    }));
    // Limpa o erro do campo quando o usuário começa a digitar
    if (errors[name as keyof FormData]) {
      setErrors(prev => ({
        ...prev,
        [name]: undefined
      }));
    }
  }, [errors]);

  // Envia o formulário
  const handleSubmit = useCallback(async () => {
    console.log('Iniciando submissão do formulário...');
    if (!validateCurrentStep()) {
      console.log('Validação do formulário falhou');
      return;
    }
    
    try {
      setIsSubmitting(true);
      
      // Exibe feedback visual para o usuário
      toast.loading('Finalizando seu cadastro...', { 
        id: 'signup',
        style: {
          background: '#2d0a42',
          color: '#fff',
          border: '1px solid #6d28d9',
          padding: '12px 16px',
          borderRadius: '8px',
          maxWidth: '90vw',
          textAlign: 'center'
        }
      });
      
      console.log('Chamando completeFirstAccess com:', { 
        email: formData.email,
        name: formData.name,
        password: '***' // Não logar a senha real
      });
      
      const result = await completeFirstAccess({
        name: formData.name.trim(),
        email: formData.email.trim().toLowerCase(),
        password: formData.password
      });
      
      console.log('Resposta do completeFirstAccess:', result);
      
      if (result.success) {
        // Feedback de sucesso
        console.log('Cadastro concluído com sucesso!');
        toast.success('Cadastro concluído com sucesso! Faça login para continuar.', { 
          id: 'signup',
          style: {
            background: '#166534',
            color: '#fff',
            border: '1px solid #22c55e',
            padding: '12px 16px',
            borderRadius: '8px',
            maxWidth: '90vw',
            textAlign: 'center'
          }
        });
        
        setSignUpSuccess(true);
        
        // Fecha o modal após 1.5 segundos
        setTimeout(() => {
          onSuccess();
          onClose();
          
          // Reseta o formulário após o fechamento
          setTimeout(() => {
            setFormData({
              name: '',
              email: '',
              password: '',
              confirmPassword: ''
            });
            setCurrentStep('name');
            setSignUpSuccess(false);
            setErrors({});
          }, 300);
        }, 1500);
      } else {
        // Trata erros específicos da API
        console.error('Erro ao completar cadastro:', result.error);
        let errorMessage = result.error || 'Erro ao completar o cadastro. Tente novamente.';
        
        // Traduz mensagens comuns de erro
        if (errorMessage.includes('Usuário não encontrado') || errorMessage.includes('não encontrado')) {
          errorMessage = 'E-mail não encontrado. Verifique se digitou corretamente ou entre em contato com o suporte.';
          // Vai para o passo do e-mail
          goToStep('email');
        } else if (errorMessage.includes('password') || errorMessage.includes('senha')) {
          errorMessage = 'A senha fornecida não atende aos requisitos mínimos.';
          goToStep('password');
        } else if (errorMessage.includes('name') || errorMessage.includes('nome')) {
          errorMessage = 'O nome fornecido é inválido.';
          goToStep('name');
        } else if (errorMessage.includes('email') || errorMessage.includes('e-mail')) {
          errorMessage = 'Por favor, insira um endereço de e-mail válido.';
          goToStep('email');
        }
        
        // Exibe a mensagem de erro estilizada
        toast.error(errorMessage, { 
          id: 'signup',
          style: {
            background: '#2d0a42',
            color: '#fff',
            border: '1px solid #6d28d9',
            padding: '12px 16px',
            borderRadius: '8px',
            maxWidth: '90vw',
            textAlign: 'center'
          }
        });
      }
    } catch (error) {
      console.error('Erro ao cadastrar:', error);
      
      let errorMessage = 'Erro inesperado ao processar seu cadastro';
      
      if (error instanceof Error) {
        if (error.name === 'AbortError' || error.message.includes('timeout')) {
          errorMessage = 'Tempo de conexão esgotado. Verifique sua conexão com a internet.';
        } else if (error.message.includes('Failed to fetch') || error.message.includes('NetworkError')) {
          errorMessage = 'Não foi possível conectar ao servidor. Verifique sua conexão com a internet.';
        } else if (error.message) {
          errorMessage = error.message;
        }
      }
      
      toast.error(errorMessage, { 
        id: 'signup',
        style: {
          background: '#2d0a42',
          color: '#fff',
          border: '1px solid #6d28d9',
          padding: '12px 16px',
          borderRadius: '8px',
          maxWidth: '90vw',
          textAlign: 'center'
        }
      });
    } finally {
      setIsSubmitting(false);
    }
  }, [completeFirstAccess, formData, goToStep, onClose, onSuccess, validateCurrentStep]);

  // Índice do passo atual para a navegação
  const stepIndex = STEPS.indexOf(currentStep);
  const totalSteps = STEPS.length;

  return (
    <div 
      className="fixed inset-0 z-50 flex items-center justify-center bg-black/70 backdrop-blur-sm p-4 overflow-y-auto"
      role="dialog"
      aria-modal="true"
      onClick={(e) => e.target === e.currentTarget && onClose()}
    >
      <motion.div 
        className="relative bg-[#1a0a2e] rounded-2xl w-full max-w-md border border-white/10 overflow-hidden shadow-2xl"
        initial={{ opacity: 0, scale: 0.95, y: 20 }}
        animate={{ opacity: 1, scale: 1, y: 0 }}
        exit={{ opacity: 0, scale: 0.95, y: 20 }}
        transition={{ duration: 0.2, ease: 'easeOut' }}
      >
        {/* Cabeçalho */}
        <div className="p-4 border-b border-white/10 flex items-center justify-between">
          <button 
            onClick={currentStep === 'name' ? onClose : handleBack}
            className="p-1.5 rounded-full hover:bg-white/10 transition-colors focus:outline-none focus-visible:ring-2 focus-visible:ring-white/50"
            aria-label={currentStep === 'name' ? 'Fechar' : 'Voltar'}
          >
            {currentStep === 'name' ? (
              <X className="h-5 w-5 text-gray-400" />
            ) : (
              <ArrowLeft className="h-5 w-5 text-gray-400" />
            )}
          </button>
          
          <h2 className="text-lg font-semibold text-white">
            {currentStep === 'name' && 'Seu Nome'}
            {currentStep === 'email' && 'Seu E-mail'}
            {currentStep === 'password' && 'Criar Senha'}
          </h2>
          
          <div className="w-8">
            {/* Espaçador para alinhar o título ao centro */}
          </div>
        </div>
        
        <div className="p-6">
          <AnimatePresence mode="wait" initial={false}>
            {signUpSuccess ? (
              <motion.div 
                key="success"
                initial={{ opacity: 0, y: 10 }}
                animate={{ opacity: 1, y: 0 }}
                exit={{ opacity: 0, y: -10 }}
                className="text-center py-8"
              >
                <div className="mx-auto flex items-center justify-center h-16 w-16 rounded-full bg-green-100 mb-4">
                  <Check className="h-8 w-8 text-green-600" />
                </div>
                <h3 className="text-xl font-semibold text-white mb-2">Tudo certo!</h3>
                <p className="text-gray-300">Seu cadastro foi concluído com sucesso.</p>
              </motion.div>
            ) : (
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
                  <div className="space-y-6">
                    <div className="relative">
                      <div className="absolute inset-y-0 left-0 pl-3 flex items-center pointer-events-none">
                        <User className={`h-5 w-5 ${errors.name ? 'text-red-500' : 'text-gray-400'}`} />
                      </div>
                      <input
                        type="text"
                        name="name"
                        value={formData.name}
                        onChange={handleChange}
                        className={`block w-full pl-10 pr-3 py-3 bg-white/5 border ${errors.name ? 'border-red-500' : 'border-white/10'} rounded-lg text-white placeholder-gray-400 focus:outline-none focus:ring-2 focus:ring-primary-purple focus:border-transparent`}
                        placeholder="Nome completo"
                        autoComplete="name"
                        aria-invalid={!!errors.name}
                        aria-describedby={errors.name ? 'name-error' : undefined}
                      />
                    </div>
                    {errors.name && (
                      <p id="name-error" className="mt-1 text-sm text-red-400">
                        {errors.name}
                      </p>
                    )}
                    <button
                      onClick={handleNext}
                      disabled={!formData.name.trim()}
                      className="w-full bg-gradient-to-r from-primary-purple to-accent-red text-white font-medium py-3 px-4 rounded-lg hover:opacity-90 transition-opacity disabled:opacity-50 disabled:cursor-not-allowed focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-primary-purple focus:ring-offset-[#1a0a2e]"
                    >
                      Continuar
                    </button>
                  </div>
                )}

                {/* Passo 2: E-mail */}
                {currentStep === 'email' && (
                  <div className="space-y-4 w-full">
                    <div className="relative">
                      <div className="absolute inset-y-0 left-0 pl-3 flex items-center pointer-events-none">
                        <Mail className={`h-5 w-5 ${errors.email ? 'text-red-500' : 'text-gray-400'}`} />
                      </div>
                      <input
                        type="email"
                        name="email"
                        value={formData.email}
                        onChange={handleChange}
                        className={`block w-full pl-10 pr-3 py-3 bg-white/5 border ${errors.email ? 'border-red-500' : 'border-white/10'} rounded-lg text-white placeholder-gray-400 focus:outline-none focus:ring-2 focus:ring-primary-purple focus:border-transparent`}
                        placeholder="seu@email.com"
                        autoComplete="email"
                        aria-invalid={!!errors.email}
                        aria-describedby={errors.email ? 'email-error' : undefined}
                      />
                    </div>
                    {errors.email && (
                      <p id="email-error" className="mt-1 text-sm text-red-400">
                        {errors.email}
                      </p>
                    )}
                    <div className="flex space-x-3 pt-2">
                      <button
                        onClick={handleBack}
                        className="flex-1 bg-white/5 hover:bg-white/10 text-white font-medium py-3 px-4 rounded-lg transition-colors focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-white/20 focus:ring-offset-[#1a0a2e]"
                      >
                        Voltar
                      </button>
                      <button
                        onClick={handleNext}
                        disabled={!formData.email || !!errors.email}
                        className="flex-1 bg-gradient-to-r from-primary-purple to-accent-red text-white font-medium py-3 px-4 rounded-lg hover:opacity-90 transition-opacity disabled:opacity-50 disabled:cursor-not-allowed focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-primary-purple focus:ring-offset-[#1a0a2e]"
                      >
                        Continuar
                      </button>
                    </div>
                  </div>
                )}

                {/* Passo 3: Senha */}
                {currentStep === 'password' && (
                  <div className="space-y-6">
                    <div className="space-y-4">
                      <div className="relative">
                        <div className="absolute inset-y-0 left-0 pl-3 flex items-center pointer-events-none">
                          <Lock className="h-5 w-5 text-gray-500" />
                        </div>
                        <input
                          type={showPassword ? 'text' : 'password'}
                          name="password"
                          value={formData.password}
                          onChange={handleChange}
                          className="block w-full pl-10 pr-10 py-3 bg-white/5 border border-white/10 rounded-lg text-white placeholder-gray-400 focus:outline-none focus:ring-2 focus:ring-primary-purple focus:border-transparent"
                          placeholder="Crie uma senha"
                          autoComplete="new-password"
                          aria-invalid={!!errors.password}
                          aria-describedby={errors.password ? 'password-error' : undefined}
                        />
                        <button
                          type="button"
                          onClick={() => setShowPassword(!showPassword)}
                          className="absolute inset-y-0 right-0 pr-3 flex items-center text-gray-400 hover:text-white focus:outline-none"
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
                          <Lock className="h-5 w-5 text-gray-500" />
                        </div>
                        <input
                          type={showConfirmPassword ? 'text' : 'password'}
                          name="confirmPassword"
                          value={formData.confirmPassword}
                          onChange={handleChange}
                          className="block w-full pl-10 pr-10 py-3 bg-white/5 border border-white/10 rounded-lg text-white placeholder-gray-400 focus:outline-none focus:ring-2 focus:ring-primary-purple focus:border-transparent"
                          placeholder="Confirme sua senha"
                          autoComplete="new-password"
                          aria-invalid={!!errors.confirmPassword}
                          aria-describedby={errors.confirmPassword ? 'confirm-password-error' : undefined}
                        />
                        <button
                          type="button"
                          onClick={() => setShowConfirmPassword(!showConfirmPassword)}
                          className="absolute inset-y-0 right-0 pr-3 flex items-center text-gray-400 hover:text-white focus:outline-none"
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
                        <div 
                          id={errors.password ? 'password-error' : 'confirm-password-error'} 
                          className="flex items-start text-red-400 text-sm -mt-2"
                        >
                          <AlertCircle className="h-4 w-4 mr-1.5 mt-0.5 flex-shrink-0" />
                          <span>{errors.password || errors.confirmPassword}</span>
                        </div>
                      )}
                    </div>
                    
                    <div className="bg-white/5 p-3 rounded-lg border border-white/10">
                      <p className="text-xs text-gray-300 font-medium mb-2">Sua senha deve conter:</p>
                      <ul className="text-xs text-gray-400 space-y-1">
                        <li className="flex items-center">
                          <Check className={`h-3 w-3 mr-1.5 ${formData.password.length >= 6 ? 'text-green-400' : 'text-gray-600'}`} />
                          <span className={formData.password.length >= 6 ? 'text-green-400' : ''}>
                            Pelo menos 6 caracteres
                          </span>
                        </li>
                        <li className="flex items-center">
                          <Check className={`h-3 w-3 mr-1.5 ${/[A-Z]/.test(formData.password) ? 'text-green-400' : 'text-gray-600'}`} />
                          <span className={/[A-Z]/.test(formData.password) ? 'text-green-400' : ''}>
                            Pelo menos uma letra maiúscula
                          </span>
                        </li>
                        <li className="flex items-center">
                          <Check className={`h-3 w-3 mr-1.5 ${/[a-z]/.test(formData.password) ? 'text-green-400' : 'text-gray-600'}`} />
                          <span className={/[a-z]/.test(formData.password) ? 'text-green-400' : ''}>
                            Pelo menos uma letra minúscula
                          </span>
                        </li>
                        <li className="flex items-center">
                          <Check className={`h-3 w-3 mr-1.5 ${/[0-9]/.test(formData.password) ? 'text-green-400' : 'text-gray-600'}`} />
                          <span className={/[0-9]/.test(formData.password) ? 'text-green-400' : ''}>
                            Pelo menos um número
                          </span>
                        </li>
                        <li className="flex items-center">
                          <Check className={`h-3 w-3 mr-1.5 ${formData.password === formData.confirmPassword && formData.password ? 'text-green-400' : 'text-gray-600'}`} />
                          <span className={formData.password === formData.confirmPassword && formData.password ? 'text-green-400' : ''}>
                            As senhas devem ser iguais
                          </span>
                        </li>
                      </ul>
                    </div>
                    
                    <div className="flex space-x-3 pt-2">
                      <button
                        onClick={handleBack}
                        className="flex-1 bg-white/5 hover:bg-white/10 text-white font-medium py-3 px-4 rounded-lg transition-colors focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-white/20 focus:ring-offset-[#1a0a2e]"
                      >
                        Voltar
                      </button>
                      <button
                        onClick={handleSubmit}
                        disabled={isSubmitting || !formData.password || !formData.confirmPassword || !!errors.password || !!errors.confirmPassword}
                        className="flex-1 bg-gradient-to-r from-primary-purple to-accent-red text-white font-medium py-3 px-4 rounded-lg hover:opacity-90 transition-opacity disabled:opacity-50 disabled:cursor-not-allowed focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-primary-purple focus:ring-offset-[#1a0a2e] flex items-center justify-center"
                      >
                        {isSubmitting ? (
                          <>
                            <Loader2 className="animate-spin h-5 w-5 mr-2" />
                            Finalizando...
                          </>
                        ) : (
                          'Finalizar Cadastro'
                        )}
                      </button>
                    </div>
                  </div>
                )}
              </motion.div>
            )}
          </AnimatePresence>
        </div>
        
        {/* Rodapé */}
        <div className="p-4 border-t border-white/10 text-center">
          <p className="text-sm text-gray-400">
            Já tem uma conta?{' '}
            <button
              onClick={() => onSwitchToLogin(formData.email)}
              className="text-primary-purple hover:text-primary-purple/80 font-medium focus:outline-none focus:underline"
            >
              Fazer login
            </button>
          </p>
        </div>
      </motion.div>
    </div>
  );
}
