with open('README.md', 'r', encoding='utf-8') as file:
    content = file.read()

# Atualiza a primeira ocorrência
content = content.replace(
    "- [ ] Ícones de Pastas\n  - [ ] Ícones personalizados (planejado)\n  - [ ] Tamanho dos ícones (planejado)",
    "- [x] Ícones de Pastas\n  - [x] Ícones personalizados (implementado)\n  - [x] Tamanho dos ícones (pequeno, médio, grande)\n  - [x] Temas pré-definidos (padrão, colorido, mínimo)\n  - [x] Controle de visibilidade (barra lateral e explorador de arquivos)",
    1
)

# Atualiza a segunda ocorrência
content = content.replace(
    "- [ ] Ícones de Pastas\n  - [ ] Ícones personalizados (planejado)\n  - [ ] Tamanho dos ícones (planejado)",
    "- [x] Ícones de Pastas\n  - [x] Ícones personalizados (implementado)\n  - [x] Tamanho dos ícones (pequeno, médio, grande)\n  - [x] Temas pré-definidos (padrão, colorido, mínimo)",
    1
)

with open('README.md', 'w', encoding='utf-8') as file:
    file.write(content)
