# Coleta de design: workspace do Kikito Terminal

## Identificação da referência

- Arquivo: `C:\Users\Marcos\Downloads\✅ Kikito Terminal.zip`
- SHA-256: `8CCDB225255F422E72B13915DE815911B62092345B73F31374B98BA6A8D09474`
- Inventário: 275 arquivos, 12.694.287 bytes descompactados.
- Arquivo inicial: `ui_kits/workspace/index.html`.
- Viewport declarado pelo manifesto: 1280 × 800 CSS px.
- Fonte do protótipo: React 18.3.1, ReactDOM 18.3.1, Babel Standalone 7.29.0 e pacote `WarpDesignSystem_e4c970`, carregados pelo HTML.

## Escopo

Esta coleta cobre o fluxo local de organizações, projetos, abas de terminal e Source Control visual. A ordem de implementação é deliberadamente serial: trilho de organizações, sidebar de projetos, barra de abas e terminal, Home, formulários e recorte de imagem, menus e estados de teclado, painel Source Control e revisão integrada. Cada superfície precisa passar pela comparação visual e funcional antes de iniciar a seguinte.

O arquivo `uploads/pasted-1792329106410-0.png` é uma referência complementar do Source Control do VS Code. Ele mostra a área lateral estreita com lista de repositórios, Changes, controles de commit, grafo de commits e refs coloridas. Não é uma captura do workspace inteiro do Kikito Terminal.

## O que a referência representa

O protótipo inicializa com três organizações demonstrativas (Keener, Pessoal e Freelas), sete projetos, terminais e sessões de agentes de exemplo. A composição padrão usa `tone: Jarvis`, `railNames: false` e `nestedTabs: false`. Há alternâncias no painel de edição para os tons Sóbria, Leve e Jarvis, nomes no trilho e abas aninhadas na sidebar.

O layout combina uma janela centralizada, trilho fixo de 64 px, sidebar fixa de 248 px quando uma organização está aberta, barra superior de 34 px, canvas expansível e painel Git opcional. A Home ocupa a área principal sem sidebar de projetos. A moldura demo tem controles de janela e painel de ajustes que não devem ser confundidos com elementos permanentes do produto.

## Fonte da verdade e limites

1. Para estrutura, tokens e medidas: `tokens/*.css`, `ui_kits/workspace/*.jsx` e comentários `@dsCard` do HTML.
2. Para componentes comuns: `components/**`, em especial `Button`, `TextInput`, `Menu`, `Dialog`, `MenuButton`, `Icon`, `KeyboardShortcut`, `Tooltip`, `ColorDot` e `IconWithStatus`.
3. Para aparência de estado: regras de estado e transições declaradas nos elementos React. O ZIP não inclui captura integral do workspace, portanto os valores medidos do código são evidência primária; não há screenshot de tela inteira para comparação por pixels nesta etapa.
4. Para Git real: o código Rust do Kikito Terminal. `GitData.jsx` e `GitPanel.jsx` contêm dados e operações simuladas; esses arquivos são somente referência visual e de interação demonstrativa.

A tentativa de captura estática por `file://` produziu uma página vazia porque o protótipo depende de scripts React/Babel externos e do carregamento correto por HTTP. Nenhuma captura da página vazia será tratada como referência visual. A etapa de QA deve servir uma cópia isolada do ZIP por HTTP local, testar carregamento de dependências e registrar console, viewport e screenshots antes de comparar com o app.

## Princípio de paridade

“98%” é uma porta de qualidade, não uma estimativa visual informal. Para cada etapa, comparar a referência e o build nativo no mesmo viewport, tema, escala, conteúdo e estado. O relatório da etapa precisa incluir diff visual, dimensões, contagem dos elementos, interações testadas e defeitos restantes. Só avançar à etapa seguinte quando a superfície atual alcançar pelo menos 98% na comparação visual e passar todos os fluxos da própria etapa.

Comparação de pixels idênticos não é adequada para conteúdo terminal dinâmico ou timestamps. Nesses casos, fixar fixtures e fontes, comparar a geometria e a aparência da casca separadamente, e mascarar apenas regiões de conteúdo variável declaradas previamente. Não mascarar espaçamento, texto estável, ícones, bordas ou estados interativos.

## Mapa desta coleta

- [Mapa do sistema visual](01-design-system-map.md)
- [Fluxo e composição](02-page-flow.md)
- [Inventário de componentes](03-components.md)
- [Interações e movimento](04-interactions-motion.md)
- [Ativos e licenças de uso](05-assets.md)
- [Tamanhos de tela](06-responsive.md)
- [Ordem de implementação e gates](07-implementation-plan.md)
- [Evidências e limitações](08-playwright-evidence.md)
- [Auditoria de diferenças no app atual](09-current-app-gap-audit.md)
- [Especificação de implementação](../../../impl/kikito-workspace-pixel-parity.md)
- [Plano de trabalho](../../superpowers/plans/2026-10-19-kikito-workspace-pixel-parity.md)
