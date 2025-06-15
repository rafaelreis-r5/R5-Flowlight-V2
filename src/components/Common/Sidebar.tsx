import React, { useState } from 'react';
import { ChevronLeft, ChevronRight, Settings, Zap, Info, Mail } from 'lucide-react';
import { motion } from 'framer-motion';

interface SidebarItem {
  id: string;
  label: string;
  icon: React.ReactNode;
  onClick: () => void;
}

interface SocialIcon {
  id: string;
  icon: React.ReactNode;
  url: string;
  tooltip: string;
}

export const Sidebar = () => {
  const [isExpanded, setIsExpanded] = useState(true);

  const menuItems: SidebarItem[] = [
    {
      id: 'features',
      label: 'Funcionalidades',
      icon: <Zap className="w-5 h-5" />,
      onClick: () => console.log('Funcionalidades clicado')
    },
    {
      id: 'settings',
      label: 'Configurações',
      icon: <Settings className="w-5 h-5" />,
      onClick: () => {
        const settingsButton = document.querySelector('button[title="Configurações"]') as HTMLButtonElement;
        if (settingsButton) settingsButton.click();
      }
    },
    {
      id: 'about',
      label: 'Quem Somos',
      icon: <Info className="w-5 h-5" />,
      onClick: () => console.log('Quem Somos clicado')
    },
    {
      id: 'contact',
      label: 'Contato',
      icon: <Mail className="w-5 h-5" />,
      onClick: () => console.log('Contato clicado')
    }
  ];

  const socialIcons: SocialIcon[] = [
    {
      id: 'whatsapp',
      icon: <img src="/icons/whatsapp-icon.svg" alt="WhatsApp" className="w-6 h-6" />,
      url: 'https://wa.me/message/ASD3JJZCA7GSP1',
      tooltip: 'Fale conosco no WhatsApp'
    },
    {
      id: 'instagram',
      icon: <img src="/icons/instagram-icon.svg" alt="Instagram" className="w-6 h-6" />,
      url: 'https://www.instagram.com/r5hub',
      tooltip: 'Siga-nos no Instagram'
    },
    {
      id: 'website',
      icon: <img src="/icons/website-icon.svg" alt="Site" className="w-6 h-6" />,
      url: 'https://www.r5hub.com.br',
      tooltip: 'Visite nosso site'
    }
  ];

  return (
    <motion.div 
      className={`h-screen bg-[#1a0a2e] flex flex-col border-l border-gray-800 ${isExpanded ? 'w-64' : 'w-16'}`}
      initial={{ x: 300, opacity: 0 }}
      animate={{ x: 0, opacity: 1 }}
      transition={{ duration: 0.3 }}
    >
      {/* Botão de recolher/expandir */}
      <button 
        onClick={() => setIsExpanded(!isExpanded)}
        className="p-3 text-gray-400 hover:text-white transition-colors self-end"
        aria-label={isExpanded ? 'Recolher menu' : 'Expandir menu'}
      >
        {isExpanded ? <ChevronRight /> : <ChevronLeft />}
      </button>

      {/* Itens do menu */}
      <nav className="flex-1 px-2 space-y-1">
        {menuItems.map((item) => (
          <button
            key={item.id}
            onClick={item.onClick}
            className={`w-full flex items-center p-3 rounded-lg transition-colors text-gray-300 hover:bg-[#2a1a3a] hover:text-white group`}
          >
            <div className="flex items-center justify-center w-8">
              {item.icon}
            </div>
            {isExpanded && (
              <motion.span 
                className="ml-3 text-sm font-medium"
                initial={{ opacity: 0, x: -10 }}
                animate={{ opacity: 1, x: 0 }}
                exit={{ opacity: 0, x: -10 }}
                transition={{ duration: 0.2 }}
              >
                {item.label}
              </motion.span>
            )}
          </button>
        ))}
      </nav>

      {/* Ícones sociais */}
      <div className="p-4 border-t border-gray-800">
        <div className={`flex ${isExpanded ? 'justify-between' : 'flex-col items-center space-y-4'}`}>
          {socialIcons.map((social) => (
            <a 
              key={social.id}
              href={social.url}
              target="_blank"
              rel="noopener noreferrer"
              className="text-gray-400 hover:text-white transition-colors"
              title={isExpanded ? '' : social.tooltip}
            >
              {social.icon}
              {isExpanded && (
                <span className="ml-2 text-xs">
                  {social.tooltip}
                </span>
              )}
            </a>
          ))}
        </div>
      </div>
    </motion.div>
  );
};
