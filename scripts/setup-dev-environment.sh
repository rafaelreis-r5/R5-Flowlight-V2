#!/bin/bash

# Script para configurar o ambiente de desenvolvimento do R5 Flowlight
# Este script instala as extensões recomendadas para o VS Code e configura o ambiente

# Cores para saída formatada
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo -e "${YELLOW}🚀 Configurando ambiente de desenvolvimento R5 Flowlight...${NC}
"
# Verifica se o VS Code está instalado
if ! command -v code &> /dev/null; then
    echo -e "${YELLOW}⚠️  VS Code não encontrado no PATH. Por favor, instale o VS Code primeiro.${NC}"
    echo "Baixe em: https://code.visualstudio.com/"
    exit 1
fi

echo -e "${GREEN}✅ VS Code encontrado${NC}"

# Lista de extensões recomendadas
extensions=(
    # Rust
    "rust-lang.rust-analyzer"
    "tamasfe.even-better-toml"
    "serayuzgur.crates"
    
    # Tauri
    "tauri-apps.tauri-vscode"
    "tauri-apps.tauri"
    "tauri-apps.tauri-vscode"
    
    # React/TypeScript
    "dbaeumer.vscode-eslint"
    "esbenp.prettier-vscode"
    "bradlc.vscode-tailwindcss"
    "streetsidesoftware.code-spell-checker"
    "streetsidesoftware.code-spell-checker-portuguese-brazilian"
    
    # Ferramentas de desenvolvimento
    "editorconfig.editorconfig"
    "github.copilot"
    "github.vscode-github-actions"
    "eamodio.gitlens"
    "wix.vscode-import-cost"
    "usernamehw.errorlens"
    "gruntfuggly.todo-tree"
)

echo -e "\n${YELLOW}📦 Instalando extensões do VS Code...${NC}"

# Instala cada extensão
for extension in "${extensions[@]}"; do
    echo -n "Instalando $extension... "
    code --install-extension "$extension" --force &> /dev/null
    if [ $? -eq 0 ]; then
        echo -e "${GREEN}✅${NC}"
    else
        echo -e "${YELLOW}⚠️  Falha ao instalar${NC}"
    fi
done

# Cria a pasta .vscode se não existir
mkdir -p .vscode

# Cria/atualiza o arquivo de configurações do VS Code
cat > .vscode/settings.json << 'EOL'
{
    "editor.defaultFormatter": "esbenp.prettier-vscode",
    "editor.formatOnSave": true,
    "editor.codeActionsOnSave": {
        "source.fixAll.eslint": "explicit",
        "source.organizeImports": true
    },
    "editor.tabSize": 2,
    "editor.insertSpaces": true,
    "files.trimTrailingWhitespace": true,
    "files.insertFinalNewline": true,
    "files.trimFinalNewlines": true,
    "eslint.validate": ["javascript", "javascriptreact", "typescript", "typescriptreact"],
    "typescript.tsdk": "node_modules/typescript/lib",
    "typescript.preferences.importModuleSpecifier": "relative",
    "javascript.updateImportsOnFileMove.enabled": "always",
    "typescript.updateImportsOnFileMove.enabled": "always",
    "cSpell.words": [
        "Flowlight",
        "Rustup",
        "Tauri",
        "Tantivy",
        "Tokio",
        "Zustand"
    ],
    "rust-analyzer.checkOnSave.command": "clippy",
    "rust-analyzer.cargo.features": "all",
    "rust-analyzer.procMacro.enable": true,
    "tauri.webview.devPath": "http://localhost:1420"
}
EOL

echo -e "\n${GREEN}🎉 Configuração do ambiente concluída com sucesso!${NC}"
echo -e "\nPróximos passos:"
echo -e "1. Reinicie o VS Code para aplicar todas as configurações"
echo -e "2. Execute 'npm install' para instalar as dependências do projeto"
echo -e "3. Execute 'cargo build' para compilar as dependências Rust"

echo -e "\n${YELLOW}🚀 Ambiente pronto para desenvolvimento!${NC}"
