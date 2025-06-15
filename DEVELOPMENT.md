# Guia de Desenvolvimento R5 Flowlight

Este documento fornece informações essenciais para configurar e trabalhar no ambiente de desenvolvimento do R5 Flowlight.

## 🛠️ Configuração do Ambiente

### Pré-requisitos

- [Rust](https://rustup.rs/) (1.70+)
- [Node.js](https://nodejs.org/) (18+)
- [Tauri CLI](https://tauri.app/v1/guides/getting-started/prerequisites)
- [Git](https://git-scm.com/)
- [GitHub CLI](https://cli.github.com/) (opcional, mas recomendado)

### Configuração Inicial

1. **Clonar o repositório**
   ```bash
   git clone https://github.com/r5hub/flowlight.git
   cd flowlight
   ```

2. **Configurar o ambiente de desenvolvimento**
   ```bash
   # Instalar dependências do Node.js
   npm install
   
   # Instalar dependências do Rust
   cargo build
   
   # Configurar o ambiente do VS Code
   chmod +x scripts/setup-dev-environment.sh
   ./scripts/setup-dev-environment.sh
   ```

3. **Configurar as proteções de branch** (apenas administradores)
   ```bash
   chmod +x scripts/setup-branch-protection.sh
   ./scripts/setup-branch-protection.sh
   ```

## 🏗️ Estrutura do Projeto

```
.
├── .github/               # Configurações do GitHub
│   ├── workflows/         # GitHub Actions
│   ├── CODEOWNERS         # Donos do código
│   └── branch-protection.yml # Configurações de proteção de branch
├── src/                   # Código-fonte do frontend (React/TypeScript)
├── src-tauri/            # Código-fonte do backend (Rust/Tauri)
│   ├── src/              # Código Rust
│   └── Cargo.toml         # Dependências Rust
├── scripts/               # Scripts úteis
├── .vscode/               # Configurações do VS Code
├── .editorconfig          # Configurações de estilo
├── package.json           # Dependências Node.js
└── README.md              # Documentação principal
```

## 🚀 Comandos Úteis

### Desenvolvimento

```bash
# Iniciar servidor de desenvolvimento
npm run tauri dev

# Construir para produção
npm run tauri build

# Executar testes
npm test

# Verificar formatação
npm run format:check

# Corrigir formatação
npm run format:fix

# Verificar linting
npm run lint
```

### Git Workflow

1. **Criar um novo branch**
   ```bash
   git checkout -b feature/nome-da-feature
   ```

2. **Fazer commit das alterações**
   ```bash
   git add .
   git commit -m "feat: adiciona nova funcionalidade"
   ```

3. **Enviar alterações**
   ```bash
   git push -u origin feature/nome-da-feature
   ```

4. **Criar um Pull Request**
   - Vá para o GitHub e crie um PR da sua branch para `develop`
   - Adicione revisores
   - Aguarde a aprovação e o CI passar
   - Faça o merge quando aprovado

## 🧪 Testes

### Rodando Testes

```bash
# Executar todos os testes
npm test

# Executar testes em modo watch
npm test -- --watch

# Executar testes de cobertura
npm run test:coverage
```

### Convenções de Teste

- Nomes de arquivos de teste: `*.test.tsx` ou `*.spec.tsx`
- Organização: `__tests__` em cada diretório ou arquivo `*.test.tsx` junto ao código
- Padrão: AAA (Arrange, Act, Assert)

## 🛡️ Segurança

- Nunca comitar credenciais ou chaves de API
- Usar variáveis de ambiente para configurações sensíveis
- Seguir as diretrizes de segurança do projeto
- Reportar vulnerabilidades de segurança para a equipe

## 🤝 Contribuição

1. Faça um fork do projeto
2. Crie um branch para sua feature (`git checkout -b feature/AmazingFeature`)
3. Faça commit das suas alterações (`git commit -m 'Add some AmazingFeature'`)
4. Faça push para o branch (`git push origin feature/AmazingFeature`)
5. Abra um Pull Request

## 📝 Licença

Este projeto está licenciado sob a licença MIT - veja o arquivo [LICENSE](LICENSE) para detalhes.

---

Atualizado em: Junho de 2025
