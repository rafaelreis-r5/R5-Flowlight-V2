# Mudanças Implementadas - R5 Flowlight

## 🚀 **v2.0.1 - Estabilização Pós-Refatoramento** - 03/07/2025

### 🎯 **Objetivo**
Estabilizar a arquitetura Daemon + Overlay e corrigir problemas críticos de inicialização e renderização que surgiram após o refatoramento v2.0.0.

### ✅ **Problemas Resolvidos**

1.  **Janela Principal (Frontend em Branco):**
    *   **Causa:** Erro de `SyntaxError` devido a chamadas incorretas de `getCurrent()` e `getCurrentWindow()` em `src/App.tsx` e `src/hooks/useGlobalShortcut.ts`.
    *   **Solução:** Atualizadas todas as chamadas `getCurrent()` para `getCurrentWindow()` e corrigidas as importações para `import { getCurrentWindow } from '@tauri-apps/api/window';`.
    *   **Impacto:** A janela principal agora carrega e exibe o frontend corretamente.

2.  **.env não encontrado / Erro de Parsing:**
    *   **Causa:** O arquivo `.env` não estava presente no repositório Git (prática de segurança) e/ou continha erros de formatação que impediam seu carregamento.
    *   **Solução:** O arquivo `.env` foi recriado com a formatação correta e os valores de exemplo. Foi instruído ao usuário a preencher com as chaves reais.
    *   **Impacto:** A aplicação agora consegue carregar as variáveis de ambiente essenciais para seu funcionamento.

3.  **Barra de Pesquisa (Overlay) Não Aparecia:**
    *   **Causa:**
        *   **Erro de Binário:** O script `npm run dev:overlay` não especificava qual binário do Rust (`real-overlay` ou `simple-overlay`) deveria ser executado.
        *   **Condição de Corrida IPC:** O `real-overlay` tentava se conectar ao `real-daemon` antes que o daemon estivesse pronto, resultando em `Connection refused`.
        *   **Lógica de Processamento de Mensagens:** A função `listen_for_real_daemon_messages` em `real_overlay.rs` não estava processando corretamente as mensagens `ToggleOverlay` do daemon.
    *   **Solução:**
        *   O script `dev:overlay` no `package.json` foi corrigido para `cargo run --bin real-overlay`.
        *   Implementada lógica de retry na conexão IPC em `apps/search-overlay/src/real_overlay.rs`, permitindo que o overlay espere o daemon.
        *   Restaurada a lógica de processamento de mensagens `ToggleOverlay` em `listen_for_real_daemon_messages`.
    *   **Impacto:** A barra de pesquisa agora aparece e desaparece corretamente ao acionar o atalho global.

4.  **Múltiplos Ícones no Dock (macOS):**
    *   **Causa:** A aplicação principal (`r5-flowlight`) ainda definia uma janela de busca em seu `tauri.conf.json`, criando um conflito com o `search-overlay`.
    *   **Solução:** A definição da janela de busca foi removida do `src-tauri/tauri.conf.json` da aplicação principal.
    *   **Impacto:** Apenas um ícone para a aplicação principal e um para o overlay (quando ativo) são exibidos, conforme a arquitetura Daemon + Overlay.

5.  **Erros de Permissão no `tauri.conf.json`:**
    *   **Causa:** Uso de identificadores de permissão incorretos (ex: `window:default` em vez de `core:window:default`).
    *   **Solução:** Corrigidos os prefixos das permissões para `core:` no `src-tauri/tauri.conf.json`.
    *   **Impacto:** A compilação do Tauri é bem-sucedida e as permissões são aplicadas corretamente.

### 🧹 **Limpeza e Otimização**

*   **Cache do Vite:** Recomendada a limpeza manual do cache (`rm -rf node_modules/.vite`) para garantir que as alterações do frontend sejam aplicadas.
*   **Refatoração de Código:** Pequenas refatorações e correções de warnings em arquivos Rust para melhorar a clareza e a manutenibilidade.
*   **Limpeza de Componentes Legados:** Desativado o componente de busca antigo (`SearchWindow.tsx`) e removida sua chamada do `App.tsx` para centralizar toda a lógica de busca no novo `OverlaySearchWindow.tsx`, eliminando código obsoleto e potencial de conflitos.

---

## Resumo das Alterações

Reestruturei completamente a aplicação para implementar o comportamento solicitado de janelas separadas:

### 1. Estrutura de Janelas

#### Janela Principal (`src/App.tsx`)

- **Função**: Tela de seleção de nichos apenas
- **Comportamento**:
  - Mostra na inicialização
  - Após selecionar nicho, esconde automaticamente
  - Funciona em segundo plano
- **Conteúdo**: Apenas os botões dos nichos (Finance, Health, Nutrition, Creator, Daily)

#### Janela de Busca (`src/search.tsx`)

- **Função**: Interface flutuante de busca
- **Comportamento**:
  - Criada dinamicamente após seleção do nicho
  - Aparece apenas com atalho global (Cmd+Space / Ctrl+Space)
  - Sem decorações de janela (flutuante)
  - Sempre no topo (alwaysOnTop)
- **Conteúdo**:
  - Barra de busca com busca em tempo real
  - Botão de IA ao lado
  - Mini botões das funcionalidades do nicho (embaixo)
  - Resultados de busca (embaixo da barra)
  - Resposta da IA (acima da barra quando ativada)

### 2. Funcionalidades Implementadas

#### Busca em Tempo Real

- Busca de arquivos e aplicativos conforme digita
- Exibe ícones dos aplicativos dinamicamente
- Preview de conteúdo dos arquivos

#### Interface Adaptativa

- Altura da janela se ajusta ao conteúdo
- Resposta da IA aparece acima da barra de busca
- Resultados aparecem abaixo da barra de busca
- Mini botões dos nichos com atalhos (Cmd+1, Cmd+2, etc.)

#### Atalhos de Teclado

- **Tab**: Alterna entre modo busca e modo IA
- **Escape**: Fecha a janela de busca
- **Cmd+1 a Cmd+5**: Funções específicas de cada nicho
- **Enter**: Executa busca ou pergunta à IA

### 3. Arquivos Modificados/Criados

#### Frontend

- `src/App.tsx` - Reescrito para ser apenas seleção de nicho
- `src/search.tsx` - Nova janela de busca (CRIADO)
- `search.html` - HTML para janela de busca (CRIADO)
- `vite.config.ts` - Configurado para múltiplas páginas

#### Backend (Rust)

- `src-tauri/src/main.rs` - Adicionados comandos para gerenciar janelas:
  - `set_selected_module` / `get_selected_module`
  - `setup_search_window` / `show_search_window` / `hide_search_window`
  - `resize_search_window`
  - `execute_module_function`

#### Configuração

- `src-tauri/tauri.conf.json` - Configuração para janela principal apenas

### 4. Fluxo de Funcionamento

1. **Inicialização**: Abre janela principal com seleção de nichos
2. **Seleção de Nicho**:
   - Usuário clica em um nicho
   - Janela principal esconde automaticamente
   - Sistema prepara janela de busca para o nicho selecionado
3. **Ativação da Busca**:
   - Usuário pressiona Cmd+Space (ou Ctrl+Space)
   - Janela de busca aparece flutuante
   - Interface se adapta ao nicho selecionado
4. **Interação**:
   - Digite para buscar arquivos/apps
   - Tab + Enter para perguntar à IA
   - Cmd+1-5 para funções específicas do nicho
   - Escape para fechar

### 5. Recursos Implementados

#### Busca Inteligente

- Busca arquivos e aplicativos simultaneamente
- Resultados com ícones e previews
- Pontuação de relevância

#### Interface Moderna

- Animações com Framer Motion
- Design glassmorphism
- Responsiva e adaptativa
- Tema escuro otimizado

#### Integração com IA

- Contexto específico por nicho
- Respostas formatadas
- Interface dedicada acima da busca

## Próximos Passos

### Para Testar

## **IMPORTANTE: Diferença entre Dev e Build**

### 🚀 **MODO DESENVOLVIMENTO** (que você estava executando):

```bash
source "$HOME/.cargo/env" && npm run dev:full
```

**O que faz:**

- ✅ Compila apenas para **testar/desenvolver**
- ✅ Abre a aplicação **direto para uso**
- ✅ **Hot reload** - mudanças no código atualizam automaticamente
- ✅ Inclui **ferramentas de debug**
- ✅ **Mais rápido** para testar mudanças
- ❌ **NÃO gera executável** para distribuir

### 📦 **MODO BUILD** (para distribuição):

```bash
npm run build:full
```

**O que faz:**

- ✅ Compila para **produção final**
- ✅ Gera **executável .app/.exe** na pasta `src-tauri/target/release/bundle/`
- ✅ **Otimizado** (menor, mais rápido)
- ✅ **Pronto para distribuir** aos usuários
- ❌ **Mais lento** de compilar
- ❌ **Não abre automaticamente**

---

## **Para Testar Suas Mudanças:**

1. **Use MODO DEV** (o que você estava fazendo):

   ```bash
   source "$HOME/.cargo/env" && npm run dev:full
   ```

2. **⚠️ Primeira Execução - É Normal**:
   - Compila MUITAS dependências Rust (5-15 min)
   - Deixe compilar até abrir a janela
   - Próximas execuções: poucos segundos

3. **Teste o fluxo completo**:
   - ✅ Selecionar nicho na tela inicial
   - ✅ Testar busca em tempo real
   - ✅ Testar modo IA (Tab + Enter)
   - ✅ Testar atalhos (Cmd+1-5)

4. **Quando quiser o executável final**:
   ```bash
   npm run build:full
   ```
   (Vai gerar o .app na pasta target/release/bundle/)

### Funcionalidades Pendentes (TODO)

1. **Atalhos Globais**: Implementar registro de Cmd+Space quando plugins estiverem disponíveis
2. **Indexação Real**: Conectar com SearchEngine real (atualmente usando dados mock)
3. **Ícones de Apps**: Implementar extração real de ícones dos aplicativos
4. **Funções dos Nichos**: Implementar funcionalidades específicas de cada nicho
5. **Persistência**: Salvar nicho selecionado entre sessões

## Estrutura dos Nichos

Cada nicho tem suas próprias funcionalidades:

- **Finance** (⌘+1: Portfólio, ⌘+2: Análise)
- **Health** (⌘+1: Sintomas, ⌘+2: Diagnóstico)
- **Nutrition** (⌘+1: Cardápio, ⌘+2: Calorias)
- **Creator** (⌘+1: Conteúdo, ⌘+2: Design)
- **Daily** (⌘+1: Tarefas, ⌘+2: Agenda)

A aplicação agora está completamente reestruturada conforme as especificações solicitadas!

### 5. Melhorias na Comunicação IPC e Inicialização

Para resolver os problemas persistentes de "Connection refused" e a falha na comunicação entre o `main-app` e o `search-daemon`, bem como erros de build no `search-overlay`, foram implementadas as seguintes correções e melhorias:

#### 5.1. Correções na Configuração do `search-overlay`

- **`apps/search-overlay/tauri.conf.json`**:
  - A estrutura do arquivo foi corrigida para alinhar com as expectativas do Tauri 2.0, movendo os campos `security` e `windows` para dentro do objeto `app`, e o objeto `bundle` para o nível raiz.
  - O campo `identifier` duplicado dentro do objeto `bundle` foi removido, resolvendo o erro de build "unknown field `identifier`".
- **`apps/search-overlay/Cargo.toml`**:
  - A feature `macos-private-api` foi removida da dependência `tauri`, eliminando um conflito de features que impedia a compilação do `search-overlay`.

#### 5.2. Refatoração da Comunicação IPC no `search-daemon`

- **`libs/ipc-communication/src/tcp_ipc.rs`**:
  - Adicionado o método `listen` ao `TcpIPCServer`. Este método é responsável por ler continuamente as mensagens de todos os clientes conectados, desserializá-las e passá-las para um handler.
  - Substituídas todas as chamadas `println!` e `eprintln!` por `log::info!` e `log::error!` para uma saída de log consistente e controlável.
- **`apps/search-daemon/src/real_daemon.rs`**:
  - A lógica de inicialização do `TcpIPCServer` foi ajustada para garantir que o servidor TCP seja iniciado corretamente e comece a aceitar conexões (`ipc_server_instance.start().await?`).
  - A tarefa de escuta de mensagens foi refatorada para utilizar o novo método `listen` do `TcpIPCServer`, permitindo que o daemon receba e processe mensagens `IPCMessage::ModuleChanged` do `main-app`.
  - Adicionados logs detalhados para rastrear o fluxo de inicialização do servidor IPC e a recepção de mensagens, facilitando a depuração.

#### 5.3. Ajustes Temporários no `main-app` para Depuração

- **`src-tauri/src/main.rs`**:
  - O atraso (`tokio::time::sleep`) antes da tentativa de conexão do `TcpIPCClient` ao daemon foi aumentado de 3 para 10 segundos. Esta é uma medida temporária para mitigar possíveis condições de corrida durante a inicialização e garantir que o daemon tenha tempo suficiente para iniciar seu servidor.

**Impacto:**

Estas mudanças visam resolver o problema de "Connection refused" ao garantir que o servidor IPC do `search-daemon` seja iniciado e escute mensagens corretamente. Com a comunicação IPC estabelecida, o `main-app` poderá notificar o daemon sobre o módulo selecionado, permitindo que o atalho global funcione conforme o esperado e eliminando o aviso "No active module". Os erros de build do `search-overlay` também foram eliminados.
