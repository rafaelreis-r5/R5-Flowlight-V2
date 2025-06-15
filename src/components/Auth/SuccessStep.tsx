import { motion } from 'framer-motion';
import { Check } from 'lucide-react';

export const SuccessStep = () => (
  <div className="fixed inset-0 z-50 flex items-center justify-center bg-black/70 backdrop-blur-sm p-4">
    <motion.div 
      className="relative bg-[#1a0a2e] rounded-2xl w-full max-w-md p-8 text-center"
      initial={{ opacity: 0, scale: 0.95 }}
      animate={{ opacity: 1, scale: 1 }}
    >
      <div className="w-16 h-16 bg-green-500/20 rounded-full flex items-center justify-center mx-auto mb-6">
        <Check className="w-8 h-8 text-green-400" />
      </div>
      <h2 className="text-2xl font-bold text-white mb-2">Cadastro concluído!</h2>
      <p className="text-gray-300 mb-6">Seu cadastro foi realizado com sucesso. Redirecionando...</p>
      <div className="h-1 w-full bg-white/10 rounded-full overflow-hidden">
        <motion.div 
          className="h-full bg-gradient-to-r from-primary-purple to-accent-red"
          initial={{ width: 0 }}
          animate={{ width: '100%' }}
          transition={{ duration: 2, ease: 'linear' }}
        />
      </div>
    </motion.div>
  </div>
);

export default SuccessStep;
