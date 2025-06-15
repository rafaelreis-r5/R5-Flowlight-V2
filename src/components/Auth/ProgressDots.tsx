import React from 'react';
import { motion } from 'framer-motion';

interface ProgressDotsProps {
  atual: number;
  total: number;
  className?: string;
}

export const ProgressDots: React.FC<ProgressDotsProps> = ({ atual, total, className = '' }) => {
  return (
    <div className={`flex items-center justify-center space-x-2 py-4 ${className}`}>
      {Array.from({ length: total }).map((_, index) => (
        <motion.div
          key={index}
          className={`h-2 w-2 rounded-full ${
            index <= atual ? 'bg-primary-purple' : 'bg-white/20'
          }`}
          initial={{ scale: 0.8 }}
          animate={{ scale: index === atual ? 1.2 : 1 }}
          transition={{ type: 'spring', stiffness: 500, damping: 30 }}
        />
      ))}
    </div>
  );
};

export default ProgressDots;
