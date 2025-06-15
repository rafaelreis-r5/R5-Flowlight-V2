import { useState, useCallback } from 'react';
import { z } from 'zod';
import { useAuth } from '../../../contexts/AuthContext';
import { toast } from 'react-hot-toast';

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

type FormData = z.infer<typeof baseSignUpSchema> & {
  confirmPassword: string;
};

type SignUpStep = 'name' | 'email' | 'password' | 'success';

export const useSignUpForm = (initialEmail = '') => {
  const { completeFirstAccess } = useAuth();
  const [currentStep, setCurrentStep] = useState<SignUpStep>('name');
  const [formData, setFormData] = useState<FormData>({
    name: '',
    email: initialEmail,
    password: '',
    confirmPassword: '',
  });
  const [errors, setErrors] = useState<Partial<Record<keyof FormData, string>>>({});
  const [isSubmitting, setIsSubmitting] = useState(false);
  // Removidas as verificações de e-mail que não são mais necessárias
  const [showPassword, setShowPassword] = useState(false);
  const [showConfirmPassword, setShowConfirmPassword] = useState(false);

    // Validação básica de e-mail
  const validateEmail = useCallback((email: string): boolean => {
    return /^[^\s@]+@[^\s@]+\.[^\s@]+$/.test(email);
  }, []);

  // Atualiza o formulário e limpa erros
  const updateFormData = useCallback((field: keyof FormData, value: string) => {
    setFormData(prev => ({
      ...prev,
      [field]: value
    }));

    // Limpa o erro do campo quando o usuário começa a digitar
    if (errors[field]) {
      setErrors(prev => ({
        ...prev,
        [field]: undefined,
      }));
    }
  }, [errors]);

  // Valida o passo atual
  const validateStep = useCallback((step: SignUpStep): step is Exclude<SignUpStep, 'success'> => {
    try {
      if (step === 'name') {
        baseSignUpSchema.pick({ name: true }).parse({ name: formData.name });
        return true;
      } 
      
      if (step === 'email') {
        // Validação básica de e-mail
        baseSignUpSchema.pick({ email: true }).parse({ email: formData.email });
        
        // Verifica se o e-mail é válido
        if (!validateEmail(formData.email)) {
          throw new Error('Por favor, insira um e-mail válido');
        }
        
        return true;
      }
      
      if (step === 'password') {
        // Valida a senha
        baseSignUpSchema.pick({ password: true }).parse({ password: formData.password });
        
        // Valida a confirmação de senha
        if (formData.password !== formData.confirmPassword) {
          throw new Error('As senhas não conferem');
        }
        
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
      } else if (error instanceof Error) {
        setErrors(prev => ({
          ...prev,
          [step]: error.message,
        }));
      }
      return false;
    }
  }, [formData, validateEmail]);

  // Avança para o próximo passo
  const nextStep = useCallback(() => {
    if (currentStep === 'success') {
      return true;
    }
    
    if (validateStep(currentStep)) {
      if (currentStep === 'name') {
        setCurrentStep('email');
      } else if (currentStep === 'email') {
        setCurrentStep('password');
      }
      return true;
    }
    return false;
  }, [currentStep, validateStep]);

  // Volta para o passo anterior
  const prevStep = useCallback(() => {
    if (currentStep === 'email') {
      setCurrentStep('name');
    } else if (currentStep === 'password') {
      setCurrentStep('email');
    }
  }, [currentStep]);

  // Submete o formulário
  const submitForm = useCallback(async () => {
    if (!validateStep('password')) {
      return false;
    }

    setIsSubmitting(true);

    try {
      const result = await completeFirstAccess({
        name: formData.name.trim(),
        email: formData.email.trim().toLowerCase(),
        password: formData.password,
      });
      
      if (result?.success) {
        setCurrentStep('success');
        return true;
      } else {
        throw new Error(result?.error || 'Falha ao completar o cadastro');
      }
    } catch (error) {
      console.error('Erro ao completar o cadastro:', error);
      toast.error(error instanceof Error ? error.message : 'Ocorreu um erro ao completar o cadastro. Tente novamente.');
      return false;
    } finally {
      setIsSubmitting(false);
    }
  }, [formData, completeFirstAccess, validateStep]);

  return {
    // Estado
    currentStep,
    formData,
    errors,
    isSubmitting,
    isCheckingEmail: false, // Não estamos mais verificando e-mail
    emailStatus: 'available', // Define como disponível, pois o Supabase irá tratar duplicatas
    showPassword,
    showConfirmPassword,
    
    // Ações
    updateFormData,
    nextStep,
    prevStep,
    submitForm,
    setShowPassword,
    setShowConfirmPassword,
  };
};
