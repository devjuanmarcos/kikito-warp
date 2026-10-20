# Fluxo e composição do workspace

## Hierarquia da janela

```text
WorkspaceFrame (centralizado, max-width 1400, max-height 900)
├── OrganizationRail (64 px, sempre visível)
├── ProjectSidebar (248 px, somente em view=project)
└── MainColumn (flexível, min-width 0)
    ├── ProjectTabBar (34 px)
    └── MainContent
        ├── HomeScreen (view=home)
        ├── SettingsScreen (view=settings; componente herdado do kit)
        └── ProjectTerminalCanvas (view=project)
            ├── TerminalScreen ativo
            └── GitPanel opcional (largura clamp(300px, 34%, 380px))
└── Menus/diálogos sobrepostos ao frame
```

O protótipo centraliza o frame com 24 px de margem, raio externo 10 px, contorno de 1 px, overflow escondido e sombra de janela. A largura é 100% até 1400 px e a altura 100% até 900 px; a referência de captura principal é 1280 × 800 px. Essas dimensões pertencem à moldura de apresentação do protótipo. No app nativo, a área do workspace começa dentro do chrome real da janela; não duplicar controles de janela nem a margem da moldura sem comprovação na captura original.

## Estado inicial demonstrativo

`WorkspaceData.jsx` define organizações e projetos de fixture. `index.html` inicia em `view=home` por `wsLoad()` sem estado salvo, usa `orgId=keener` para seleção implícita, guarda `lastProject` por organização e `activeTab` por projeto. `WS_TWEAKS` inicia em Jarvis, sem nomes no trilho e sem abas aninhadas. `localStorage` (`kt-workspace-v1`) guarda o estado da demonstração.

Esses valores orientam densidade e variedade visual. Não persistir os IDs demonstrativos nem copiar o armazenamento em memória para o app nativo; manter o domínio SQLite e os IDs estáveis de organização/projeto/aba já existentes.

## Fluxos

### Navegação

1. `Início` no topo do trilho muda para Home e mantém organizações/projetos locais.
2. Selecionar uma organização muda para `project`, abre sua sidebar e restaura o último projeto selecionado, se existente.
3. Selecionar um projeto mantém a organização e mostra as abas do projeto na faixa superior.
4. Selecionar uma aba troca a sessão ativa sem transferir a aba a outro projeto.
5. Fechar a última aba deixa o canvas vazio com ação para criar nova aba.
6. Criar nova organização abre formulário, salva localmente e entra no estado da organização recém-criada, com projeto vazio.
7. Criar novo projeto pertence à organização atual; confirmar salva nome e diretório e abre o projeto.

### Home

Home mostra título/subtítulo conforme o tom, launcher de projetos/abas, até seis projetos recentes e até sete abas recentes. Busca filtrada retorna até seis opções, navega com setas e Enter, fecha sugestões com Escape e permite abrir projeto ou aba. Estados vazios variam entre Home, organização sem projetos e projeto sem abas.

### Organizações e projetos

- Clique no tile seleciona; clique direito ou menu explícito abre ações da organização.
- Ações da organização: editar, remover e criar projeto. Remover exige confirmação clara de que metadados e relações locais serão retirados sem apagar pastas do disco ou encerrar sessões.
- Projeto selecionado abre o canvas; hover expõe nova aba e overflow; o menu inclui nova aba, Source Control e remoção conforme a referência.
- Search filtra projetos por nome e caminho, sem mudar a seleção por conta própria. A contagem no cabeçalho é o total da organização.
- O controle `nestedTabs` é uma variante do protótipo; o default não aninha abas na sidebar.

### Diálogos

- Organização: nome, imagem opcional, cor de destaque e recorte da imagem.
- Projeto: nome e diretório. A referência de demonstração sugere `~/<slug>` quando o caminho está vazio; a implementação nativa precisa validar diretório real e manter o fluxo do seletor de pastas existente.
- Edição preenche os dados atuais e muda o rótulo final para Salvar.
- Cancelar, Escape, clique fora e X fecham ou voltam do recorte sem salvar mudanças incompletas.
- Remoção é diálogo de confirmação, não uma submissão implícita.

### Source Control

O botão Git na barra do projeto abre/fecha painel à direita; a ação no menu do projeto também abre o painel. O painel visual exibe alterações, commit e grafo. O painel do protótipo simula Fetch/Pull/Push, stage, commit, descarte e histórico; para produção, cada ação visual deve apontar às operações reais e locais disponíveis na camada Git. Operações com rede seguem apenas por ação explícita do usuário, conforme as regras já aprovadas para o app.

## Máquina de estados mínima

```text
Home ⇄ Organização/Projeto ⇄ Projeto com Aba ⇄ Projeto com Git aberto
                         ├── Diálogo criar/editar/remover
                         ├── Menu de organização/projeto
                         └── Diálogo de recorte de imagem
```

Manter a seleção válida após remoção ou erro de persistência. Todo estado aberto deve ter caminho claro para fechar com mouse e teclado; nenhuma camada invisível pode bloquear o restante do workspace.
