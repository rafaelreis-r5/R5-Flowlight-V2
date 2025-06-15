import React, { useState, useCallback, useEffect } from 'react';
import { motion, AnimatePresence } from 'framer-motion';
import { X, ArrowLeft } from 'lucide-react';
import { useAuth } from '../../contexts/AuthContext';
import { z } from 'zod';
import { toast } from 'react-hot-toast';
import axios from 'axios';

// Componentes
import { ProgressDots } from './ProgressDots';
import { SuccessStep } from './SuccessStep';
import { NameStep } from './NameStep';
import { EmailStep } from './EmailStep';
import { PasswordStep } from './PasswordStep';

// Esquema de validação com Zod
const signUpSchema = z.object({
  name: z.string().min(3, { message: 'O nome deve ter pelo menos 3 caracteres' }),
  email: z.string().email({ message: 'Por favor, insira um e-mail válido' }),
  password: z.string()
    .min(6, { message: 'A senha deve ter pelo menos 6 caracteres' })
    .regex(/[A-Z]/, { message: 'A senha deve conter pelo menos uma letra maiúscula' })
    .regex(/[0-9]/, { message: 'A senha deve conter pelo menos um número' }),
  confirmPassword: z.string(),
}).refine((data) => data.password === data.confirmPassword, {
  message: 'As senhas não conferem',
  path: ['confirmPassword'],
});

type FormData = z.infer<typeof signUpSchema>;
type EmailStatus = 'checking' | 'available' | 'exists' | 'lead' | null;

interface NewSignUpFlowProps {
  onClose: () => void;
  onSwitchToLogin: (email?: string) => void;
  initialEmail?: string;
}

export const NewSignUpFlow: React.FC<NewSignUpFlowProps> = ({
  onClose,
  onSwitchToLogin,
  initialEmail = '',
}) => {
  const { signUp } = useAuth();
  const [currentStep, setCurrentStep] = useState<'name' | 'email' | 'password' | 'success'>('name');
  const [formData, setFormData] = useState<FormData>({
    name: '',
    email: initialEmail,
    password: '',
    confirmPassword: '',
  });
  const [errors, setErrors] = useState<Partial<Record<keyof FormData, string>>>({});
  const [isSubmitting, setIsSubmitting] = useState(false);
  const [isCheckingEmail, setIsCheckingEmail] = useState(false);
  const [emailStatus, setEmailStatus] = useState<EmailStatus>(null);
  const [showPassword, setShowPassword] = useState(false);
  const [showConfirmPassword, setShowConfirmPassword] = useState(false);

  // Efeito para verificar o e-mail quando ele for alterado
  useEffect(() => {
    const checkEmail = async () => {
      const email = formData.email.trim();
      
      if (!email || email === initialEmail) {
        setEmailStatus(null);
        return;
      }

      // Validação básica de e-mail
      if (!/^[^\s@]+@[^\s@]+\.[^\s@]+$/.test(email)) {
        setEmailStatus(null);
        return;
      }

      setEmailStatus('checking');
      setIsCheckingEmail(true);

      try {
        const response = await axios.post('/api/auth/check-email', { email });
        
        if (response.data.exists) {
          setEmailStatus(response.data.isLead ? 'lead' : 'exists');
        } else {
          setEmailStatus('available');
        }
      } catch (error) {
        console.error('Erro ao verificar e-mail:', error);
        setEmailStatus(null);
        toast.error('Não foi possível verificar o e-mail. Tente novamente.');
      } finally {
        setIsCheckingEmail(false);
      }
    };

    const timer = setTimeout(checkEmail, 500);
    return () => clearTimeout(timer);
  }, [formData.email, initialEmail]);

  // Manipulador de mudança nos campos do formulário
  const handleChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    const { name, value } = e.target;
    setFormData(prev => ({
      ...prev,
      [name]: value
    }));
    
    // Limpa o erro do campo quando o usuário começa a digitar
    if (errors[name as keyof FormData]) {
      setErrors(prev => ({
        ...prev,
        [name]: undefined,
      }));
    }
  };

  // Valida o passo atual do formulário
  const validateStep = (step: 'name' | 'email' | 'password'): boolean => {
    try {
      if (step === 'name') {
        signUpSchema.pick({ name: true }).parse({ name: formData.name });
        return true;
      } else if (step === 'email') {
        signUpSchema.pick({ email: true }).parse({ email: formData.email });
        
        if (emailStatus === 'exists') {
          setErrors(prev => ({
            ...prev,
            email: 'Este e-mail já está cadastrado',
          }));
          return false;
        }
        
        if (emailStatus === 'checking' || emailStatus === null) {
          setErrors(prev => ({
            ...prev,
            email: 'Verificando disponibilidade do e-mail...',
          }));
          return false;
        }
        
        return true;
      } else if (step === 'password') {
        signUpSchema.pick({ password: true, confirmPassword: true })
          .refine(data => data.password === data.confirmPassword, {
            message: 'As senhas não conferem',
            path: ['confirmPassword'],
          })
          .parse({
            password: formData.password,
            confirmPassword: formData.confirmPassword,
          });
        return true;
      }
      return false;
    } catch (error) {
      if (error instanceof z.ZodError) {
        const newErrors: Record<string, string> = {};
        error.errors.forEach(err => {
          const path = err.path[0] as string;
          if (path) {
            newErrors[path] = err.message;
          }
        });
        setErrors(prev => ({
          ...prev,
          ...newErrors,
        }));
      }
      return false;
    }
  };

  // Navega para o próximo passo
  const handleNext = () => {
    if (currentStep === 'email' && emailStatus === 'exists') {
      onSwitchToLogin(formData.email);
      return;
    }

    if (validateStep(currentStep)) {
      if (currentStep === 'name') {
        setCurrentStep('email');
      } else if (currentStep === 'email') {
        setCurrentStep('password');
      }
    }
  };

  // Volta para o passo anterior
  const handleBack = () => {
    if (currentStep === 'email') {
      setCurrentStep('name');
    } else if (currentStep === 'password') {
      setCurrentStep('email');
    }
  };

  // Submete o formulário
  const handleSubmit = async () => {
    if (!validateStep('password')) {
      return;
    }

    setIsSubmitting(true);

    try {
      await signUp({
        name: formData.name,
        email: formData.email,
        password: formData.password,
      });

      // Se chegou aqui, o cadastro foi bem-sucedido
      setCurrentStep('success');
      
      // Fecha o modal após 2 segundos
      setTimeout(() => {
        onClose();
      }, 2000);
      
    } catch (error) {
      console.error('Erro ao cadastrar:', error);
      toast.error('Ocorreu um erro ao fazer o cadastro. Tente novamente.');
    } finally {
      setIsSubmitting(false);
    }
  };

  // Índice do passo atual para a navegação
  const currentStepIndex = ['name', 'email', 'password'].indexOf(currentStep);
  const totalSteps = 3;

  // Se o cadastro foi concluído com sucesso
  if (currentStep === 'success') {
    return <SuccessStep />;
  }

  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center bg-black/70 backdrop-blur-sm p-4">
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
            onClick={currentStep === 'name' ? onClose : handleBack}
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
        <ProgressDots current={currentStepIndex} total={totalSteps} />

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
                  onChange={handleChange}
                  onNext={handleNext}
                />
              )}

              {/* Passo 2: E-mail */}
              {currentStep === 'email' && (
                <EmailStep 
                  email={formData.email}
                  error={errors.email}
                  emailStatus={emailStatus}
                  isCheckingEmail={isCheckingEmail}
                  isSubmitting={isSubmitting}
                  onChange={handleChange}
                  onNext={handleNext}
                  onSwitchToLogin={onSwitchToLogin}
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
                  showPassword={showPassword}
                  showConfirmPassword={showConfirmPassword}
                  setShowPassword={setShowPassword}
                  setShowConfirmPassword={setShowConfirmPassword}
                  onChange={handleChange}
                  onSubmit={handleSubmit}
                />
              )}
            </motion.div>
          </AnimatePresence>
        </div>

        {/* Rodapé com botão de próximo/enviar */}
        <div className="p-4 border-t border-white/10">
          <button
            onClick={currentStep === 'password' ? handleSubmit : handleNext}
            disabled={
              isSubmitting || 
              isCheckingEmail ||
              (currentStep === 'email' && emailStatus === 'checking')
            }
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

export default NewSignUpFlow;
