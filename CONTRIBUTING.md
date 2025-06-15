# Guia de Contribuição para o R5 Flowlight

Obrigado por considerar contribuir para o R5 Flowlight! Este guia irá ajudá-lo a configurar o ambiente de desenvolvimento e entender como contribuir para o projeto.

## 📋 Pré-requisitos

Antes de começar, certifique-se de ter instalado:

- [Git](https://git-scm.com/)
- [Node.js](https://nodejs.org/) (versão 18 ou superior)
- [Rust](https://www.rust-lang.org/tools/install) (versão 1.70 ou superior)
- [Tauri CLI](https://tauri.app/v1/guides/getting-started/prerequisites)
- [GitHub CLI](https://cli.github.com/) (opcional, mas recomendado)

## 🚀 Configuração do Ambiente

1. **Faça um Fork do repositório**
   - Clique no botão "Fork" no canto superior direito da [página do repositório](https://github.com/r5hub/flowlight)
   - Clone o repositório forkado para sua máquina local:
     ```bash
     git clone https://github.com/seu-usuario/flowlight.git
     cd flowlight
     ```

2. **Configure o repositório remoto**
   ```bash
   git remote add upstream https://github.com/r5hub/flowlight.git
   ```

3. **Instale as dependências**
   ```bash
   # Instalar dependências do Node.js
   npm install
   
   # Instalar dependências do Rust
   cargo build
   ```

4. **Configure o ambiente de desenvolvimento**
   ```bash
   # Instalar extensões recomendadas do VS Code
   chmod +x scripts/setup-dev-environment.sh
   ./scripts/setup-dev-environment.sh
   ```

## 🔄 Fluxo de Trabalho

1. **Atualize seu fork**
   ```bash
   git fetch upstream
   git checkout main
   git merge upstream/main
   git push origin main
   ```

2. **Crie um branch para sua feature**
   ```bash
   git checkout -b feature/nome-da-feature
   ```

3. **Faça suas alterações**
   - Siga as convenções de código do projeto
   - Adicione testes para novas funcionalidades
   - Atualize a documentação conforme necessário

4. **Execute os testes**
   ```bash
   # Executar testes
   npm test
   
   # Verificar formatação
   npm run format:check
   
   # Verificar linting
   npm run lint
   ```

5. **Faça o commit das alterações**
   ```bash
   # Adicione os arquivos alterados
   git add .
   
   # Faça o commit com uma mensagem descritiva
   git commit -m "tipo(escopo): descrição concisa das alterações"
   ```
   
   **Tipos de commit**:
   - `feat`: Nova funcionalidade
   - `fix`: Correção de bug
   - `docs`: Alterações na documentação
   - `style`: Formatação, ponto e vírgula, etc. (não altera o código)
   - `refactor`: Refatoração de código
   - `test`: Adição ou modificação de testes
   - `chore`: Atualização de tarefas, configurações, etc.

6. **Envie as alterações**
   ```bash
   git push origin feature/nome-da-feature
   ```

7. **Abra um Pull Request**
   - Vá até a [página de Pull Requests](https://github.com/r5hub/flowlight/pulls)
   - Clique em "New Pull Request"
   - Selecione seu fork e branch
   - Preencha o template do PR
   - Adicione revisores se necessário
   - Clique em "Create Pull Request"

## 🧪 Testes

### Executando Testes

```bash
# Executar todos os testes
npm test

# Executar testes em modo watch
npm test -- --watch

# Executar testes de cobertura
npm run test:coverage
```

### Convenções de Teste

- Nomes de arquivos de teste: `*.test.ts` ou `*.test.tsx`
- Organização: `__tests__` em cada diretório ou arquivo `*.test.ts` junto ao código
- Padrão: AAA (Arrange, Act, Assert)

## 📝 Convenções de Código

### Estilo de Código

- Siga o [JavaScript Standard Style](https://standardjs.com/)
- Use Prettier para formatação
- Mantenha as linhas com no máximo 100 caracteres

### Nomenclatura

- **Componentes**: PascalCase (`MeuComponente.tsx`)
- **Arquivos**: kebab-case (`meu-arquivo.ts`)
- **Variáveis**: camelCase (`minhaVariavel`)
- **Constantes**: UPPER_SNAKE_CASE (`MINHA_CONSTANTE`)
- **Tipos/Interfaces**: PascalCase (`MinhaInterface`)
- **Enums**: PascalCase (`MeuEnum`)

## 🤝 Código de Conduta

Este projeto segue o [Código de Conduta do Contribuidor](CODE_OF_CONDUCT.md). Ao participar, você concorda em cumprir seus termos.

## 📄 Licença

Ao contribuir, você concorda que suas contribuições serão licenciadas sob a [Licença MIT](LICENSE).

## 🙋‍♂️ Preciso de ajuda?

Se tiver dúvidas ou precisar de ajuda, sinta-se à vontade para:

1. Abrir uma [issue](https://github.com/r5hub/flowlight/issues)
2. Entrar em contato com a equipe de desenvolvimento
3. Verificar a documentação do projeto

Obrigado por contribuir para o R5 Flowlight! Sua ajuda é muito valiosa para nós. 🚀
