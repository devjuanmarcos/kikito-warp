# Inventário de componentes

## Trilho de organizações — `OrgRail.jsx`

Responsabilidade: navegação primária por Início e organização, criação de organização e acesso às configurações. 64 px fixos; `neutral-1`, borda direita `border-outline`; cabeçalho demo de 34 px com três círculos de 12 px; coluna ocupa a altura restante. Itens ficam centralizados, com 8 px de gap e 8 px de padding superior.

`RailItem` oferece ativo, hover, tooltip, badge e modo opcional com nomes. Marcador lateral tem 3 px de largura e radius 0/3/3/0; ativo mede 28 px de altura e inicia 4 px abaixo do item; hover mede 12 px e inicia 12 px; inativo tem altura zero. A mudança usa `transition: all .12s`. Tooltip fica 10 px à direita, centralizado verticalmente e acima do conteúdo (`z-index:40`).

`OrgTile`: 36 × 36 px. Sem imagem, inicial maiúscula de 15 px/700, fundo `wsTint(color, 18%)` ou 32% ativo, contorno colorido de 30% e raio 8/10 px. Com imagem, crop ocupa todo o tile, opacidade 0.85 inativa/1 ativa e sombra de 1/1.5 px na cor ANSI. Organização com agente bloqueado mostra notification dot de 8 px com offset `[2,-2]`. Nomes opcionais usam 10 px, line-height 12 px, largura máxima 56 px e ellipsis. Home usa o PNG do ícone KT. Separador tem 24 × 1 px. `+` é borda tracejada 36 × 36; configurações usa gear 16 px no rodapé com padding 10/12 px.

Eventos: clique seleciona; clique direito do tile previne menu nativo e abre menu contextual. Início, `+` e configurações são ações independentes. Em touch/teclado, todos os alvos precisam continuar acessíveis sem hover.

## Sidebar de projetos — `ProjectSidebar.jsx`

Largura fixa 248 px, `flex:none`, `min-height:0`, `neutral-1`, contorno direito 1 px. Estrutura vertical:

1. Cabeçalho 34 px: ícone de organização 18 × 18 px (raio 4) ou ponto 8 × 8; nome 12 px/600 com ellipsis; botão naked small `plus` para criar projeto; padding `0 8px 0 16px`, gap 8.
2. Busca: padding `10px 8px 6px`; campo small com ícone search e placeholder `Buscar projetos...`.
3. Overline `PROJETOS` em 10 px e letter-spacing `.02em`, total de projetos à direita; padding `6px 16px 4px`.
4. Lista rolável: flex 1, gap 2, padding inferior 8 px. Sem projetos: empty state. Busca vazia: `Nenhum resultado encontrado`.
5. Rodapé fixo: padding 8 px e borda superior; botão `Novo projeto` small, ícone plus e largura total.

`ProjectRow`: margens horizontais 8 px; padding `7px 8px 7px 10px`; raio 4 px; cursor pointer. Ativo: fundo foreground a 10%, borda foreground a 15%, texto 12/600. Hover inativo: foreground a 5%; borda transparente. Título alinha ícone folder 14 px, gap 8; overflow em ellipsis. Hover troca count/status por botões new-tab e overflow, ambos small/naked. Caso contrário, mostrar o agente mais ativo/bloqueado se existir e total de abas em 10 px hint. Linha de branch: margem superior 2 px, padding-left 22 px, ícone 10 px, texto Hack 10 px com ellipsis. Abas aninhadas: apenas na variante; margem `2px 8px 4px 26px`, padding esquerdo 8 px, borda esquerda, linha com 4 px vertical/6 px horizontal e raio 4.

Eventos: clique na linha abre o projeto; clique nos controles de hover não deve propagar para abrir acidentalmente; menu de overflow fecha ao selecionar; nova aba e Source Control atuam sobre o projeto da própria linha.

## Barra de abas — `ProjectTabBar.jsx`

Altura 34 px incluindo borda inferior; background `neutral-1`; gap 8; padding horizontal 8. Sem projeto, título à esquerda. Com projeto, faixa de abas flexível (cada aba `flex:1 1 0`, min 72/max 220 px), mais botão `plus` small/naked com tooltip e shortcut `cmd+t`. Busca/launcher à direita: flex `0 1 240px`, min 120 px, altura 24, padding horizontal 10, ícone search 14 e truncamento. Source Control usa `git-branch-02` small/naked; fundo foreground a 10% enquanto aberto.

`ProjectTab`: altura 26, padding `0 24px 0 10px`, gap 6, raio 4. Ativa: fundo foreground 10%, borda 20%, texto principal; hover: 5%/15%, texto secundário. Largura mínima 72/máxima 220. Terminal usa glyph de terminal em 12 px; agente usa `IconWithStatus` em 14 px. Título 12 px, ellipsis. X de 12 px aparece apenas na ativa ou hover, alinhado a 4 px da direita; clique nele interrompe propagação e fecha somente aquela aba.

## Home — `HomeScreen.jsx`

Conteúdo rolável com coluna de máximo 960 px, centrada; padding `48px 40px 40px`; gap entre seções 32 px. Título de 23 px/600; subtítulo 14 px. Logo KT 44 × 44 com raio 10 aparece em Leve/Jarvis e some em Sóbria. Jarvis pode expor CTA de agente bloqueado.

`HomeAsk`: altura 44 px, padding horizontal 14, gap 10, raio 8; ícone agentmode 16; input 14; shortcut cmd+k 11. Borda neutral-4 normal e accent focada. Popup de resultados começa 50 px abaixo, superfície 2, contorno, radius 8, shadow-overlay, padding vertical 6; máximo seis linhas. Linhas 7 × 14 px, fundo accent-overlay-2 na seleção, resultados divididos por projeto/aba. Setas percorrem, Enter abre, Escape limpa busca; query normaliza prefixos de ação e filtra nome/subtexto.

Projetos recentes: até seis, ordenados por horário decrescente, grade `repeat(auto-fill,minmax(220px,1fr))`, gap 12. Card: 14 px padding, raio 8, gap interno 6, fundo foreground 5%, contorno 10%; hover fundo 10%, borda 20%. Exibe organização/ponto, atividade, projeto, caminho em Hack 12 hint, branch em Hack 12 e número de abas/tempo em 11 px.

Abas recentes: até sete, ordenadas pelo horário de uso; lista com borda, radius 8 e padding 4. Linha 40 px de altura, gap 12, padding horizontal 12, hover fg5%; inclui glyph/estado, título, nome do projeto/organização e tempo. Clique abre aquela aba no projeto correto.

## Diálogos e crop — `CreateDialog.jsx`, `ImageCropper.jsx`

O frame de backdrop cobre a área do workspace com `blur-backdrop`, alinha o diálogo ao centro e usa z-index 50. Diálogo tem 420 px; `surface-1`, border neutral-4 de 1 px, radius 8. Cabeçalho: 24 px laterais/topo, 12 px inferior; título 18/600. Corpo tem `DialogBody` com padding horizontal 24/inferior 24 e conteúdo em coluna, gap 14. Footer tem borda superior, padding 12/24 e ações à direita com gap 8. Input padrão 12 px; radius4, padding10, borda fg20% e focus accent.

Organização: nome; avatar/imagem; ação Escolher imagem/Trocar imagem; hint de formatos/drag-and-drop; ação Remover quando aplicável; paleta de seis cores ANSI (blue, green, magenta, yellow, cyan, red) em color dots selecionáveis. Projeto: nome e caminho/diretório. Criação não pode herdar o default de caminho de exemplo `~/slug` sem escolher e validar uma pasta real no app.

Recorte: viewport 240 × 240 px, raio 8; janela de saída 128 × 128 PNG; área selecionável com raio 22%; overlay preto 55% ao redor e borda branca 1 px; grades aparecem durante arraste. A imagem cobre a área mantendo proporção; pointer drag com capture; zoom 1×–4× centrado no ponteiro, botão ou slider, roda usa passo 1.08. Botões finais alinhados à direita. O app nativo já limita leituras, rasterização, dimensões e processamento; manter esses limites como comportamento de produção.

## Menus — `Menu.jsx`, `MenuButton.jsx`

Menu tem width padrão 186, surface-2, borda outline1, raio5, sombra overlay e padding externo vertical9. Linha: ícone14, gap8, padding `5px 14px`; rótulo 12 com truncamento; atalho naked 11; hover aplica accent sólido; item disabled não executa; item danger usa ANSI red. Separador 1 px com margem vertical4. Cabeçalho 10 px uppercase, letter-spacing `.04em`. MenuButton small posiciona o menu ao lado do botão e fecha em mousedown fora; direção esquerda no overflow do projeto.

## Painel Git — `GitPanel.jsx`, `GitGraph.jsx`

Painel lateral de largura `clamp(300px, 34%, 380px)`, superfície neutral-1, borda esquerda 1 px, altura flexível. Cabeçalho 34 px; seções colapsáveis 26 px; rows de arquivo 24 px; status bar inferior 28 px. Seção Changes agrupa staged/unstaged, ação por arquivo ao hover, status M/A/U/D e confirmação para descarte. Commit textarea e split button. Grafo usa linhas de 24 px, lane de 12 px e padding 8; refs em pill 16 px, estados HEAD/merge/color, clique expande metadados.

Este JSX é demonstração: usa `GIT_STORE`, fixtures fixas, commits sintéticos e timers para fetch/pull/push. Reutilizar apenas a organização visual, espaçamento, ícones, hierarquia e estados. O app deve usar o repositório selecionado e os dados/ações reais da camada `app/src/local_git/` e `app/src/workspace/view/local_workspace/source_control/`.
