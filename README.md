# R5 Flowlight

**Centro de comando unificado para produtividade profissional**

R5 Flowlight é um utilitário de produtividade avançado para macOS e Windows, inspirado no Alfred e Spotlight, que combina busca ultra-rápida com módulos especializados alimentados por inteligência artificial.

## 🚀 Funcionalidades

### Módulos Especializados
- **💰 Finance**: Análise financeira e gestão de investimentos
- **⚕️ Health**: Medicina clínica e do trabalho  
- **🥗 Nutrition**: Planejamento nutricional e dietético
- **🎨 Creator**: Criação de conteúdo e marketing
- **📅 Daily**: Utilitários gerais do dia a dia

### Características Principais
- 🔍 **Busca instantânea** de arquivos e aplicativos
- 🤖 **IA contextual** integrada em todos os módulos
- ⚡ **Interface responsiva** com animações suaves
- ⌨️ **Atalhos globais** para acesso rápido
- 🎨 **Design minimalista** com tema escuro

## 🛠️ Stack Tecnológico

### Backend
- **Rust** com Tauri 2.0
- **Tantivy** para busca indexada
- **Tokio** para programação assíncrona

### Frontend  
- **React 18** + **TypeScript**
- **Tailwind CSS** para styling
- **Framer Motion** para animações
- **Zustand** para gerenciamento de estado

## 🏃‍♂️ Início Rápido

### Pré-requisitos
- [Rust](https://rustup.rs/) (1.70+)
- [Node.js](https://nodejs.org/) (18+)
- [Tauri CLI](https://tauri.app/v1/guides/getting-started/prerequisites)

### Instalação
```bash
# Clone o repositório
git clone https://github.com/r5hub/flowlight.git
cd flowlight

# Instale as dependências
npm install

# Execute em modo desenvolvimento
npm run dev
```

### Build para Produção
```bash
# Build da aplicação
npm run build

# Build dos instaladores
npm run tauri build
```

## ⌨️ Atalhos

### Atalhos Globais
- `⌘/Ctrl + Space`: Abrir/fechar aplicação
- `⌘/Ctrl + 1-5`: Ativar módulos específicos

### Atalhos na Interface
- `Tab`: Alternar modo IA
- `ESC`: Fechar aplicação
- `Enter`: Executar busca/consulta IA

## 🔧 Desenvolvimento

### Estrutura do Projeto
```
flowlight/
├── src-tauri/          # Backend Rust
│   ├── src/
│   │   ├── core/       # Motor de busca
│   │   ├── api/        # Endpoints Tauri
│   │   ├── modules/    # Módulos especializados
│   │   ├── ai/         # Integração IA
│   │   └── utils/      # Utilitários
├── src/                # Frontend React
│   ├── components/     # Componentes React
│   ├── hooks/          # Custom hooks
│   ├── services/       # Serviços API
│   └── types/          # Definições TypeScript
└── docs/               # Documentação
```

### Scripts Disponíveis
- `npm run dev`: Desenvolvimento com hot reload
- `npm run build`: Build de produção
- `npm run tauri dev`: Desenvolvimento Tauri
- `npm run tauri build`: Build completo com instaladores
- `npm run lint`: Verificação de código
- `npm run type-check`: Verificação de tipos

## 📦 Distribuição

### Formatos Suportados
- **macOS**: `.dmg` e `.pkg`
- **Windows**: `.msi` e `.exe`

### Auto-update
- Integração com Sparkle (macOS)
- Integração com Squirrel (Windows)

## 🤝 Contribuição

1. Fork o projeto
2. Crie uma branch para sua feature (`git checkout -b feature/AmazingFeature`)
3. Commit suas mudanças (`git commit -m 'Add some AmazingFeature'`)
4. Push para a branch (`git push origin feature/AmazingFeature`)
5. Abra um Pull Request

## 📄 Licença

Este projeto está licenciado sob a Licença MIT - veja o arquivo [LICENSE](LICENSE) para detalhes.

## 📞 Suporte

- **Email**: dev@r5hub.com.br
- **Website**: [r5hub.com.br/flowlight](https://r5hub.com.br/flowlight)
- **Documentação**: [docs/](docs/)

---

**R5 Flowlight** - Transformando produtividade em simplicidade.