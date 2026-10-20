# Mapa do sistema visual

## Regras gerais

- Interface densa, escura e derivada do tema ativo; não aplicar amarelo da marca às barras do workspace.
- Chrome usa superfícies sólidas e linhas de 1 px. Não adicionar gradientes, sombras de card ou animações inventadas.
- Roboto é a fonte de interface; Hack é a fonte monoespaçada para caminhos, branches, hashes e terminal.
- Raio padrão 4 px; 8 px em grupos e diálogos; círculos para pontos e seletores.
- Acento do tema sinaliza foco, seleção e ação primária. A identidade amarela KT fica em ativos de marca, como o ícone.

## Tokens brutos do ZIP

| Categoria | Token ou regra | Valor |
|---|---|---:|
| Viewport | workspace do manifesto | 1280 × 800 px |
| Layout | trilho de organizações | 64 px |
| Layout | sidebar de projetos | 248 px |
| Layout | barra superior de abas | 34 px |
| Layout | largura máxima do conteúdo Home | 960 px |
| Layout | largura mínima de card Home | 220 px |
| Tipografia | interface | Roboto |
| Tipografia | terminal/caminhos/branches | Hack |
| Tipografia | base de chrome | 12 px |
| Tipografia | overline | 10 px |
| Tipografia | botão padrão/pequeno | 14 / 12 px |
| Tipografia | título de diálogo | 18 px, peso 600 |
| Tipografia | título Home | 23 px, peso 600 |
| Tipografia | terminal | 13 px, line-height 1.4 |
| Espaçamento | escala usada | 2, 3, 4, 5, 6, 7, 8, 10, 12, 15, 16, 20, 24, 28, 48 px |
| Controles | botão padrão/pequeno | 32 / 24 px de altura |
| Controles | padding horizontal padrão/pequeno | 12 / 8 px |
| Controles | ícone padrão/pequeno | 16 / 14 px |
| Controles | item do trilho | 36 × 36 px |
| Controles | avatar de organização no cabeçalho | 18 × 18 px |
| Controles | imagem de organização no diálogo | 56 × 56 px |
| Controles | ícone de estado na lista | 12–16 px conforme contexto |
| Diálogo | largura de criação/edição no JSX | 420 px |
| Diálogo | padding do cabeçalho | 24 px laterais, 24 px superior, 12 px inferior |
| Diálogo | corpo | 24 px laterais e 24 px inferior |
| Menu | largura padrão | 186 px |
| Menu | padding vertical externo | 9 px |
| Menu | linha | 5 px vertical e 14 px horizontal |
| Menu | separador | 1 px de altura e 4 px de margem vertical |
| Menu | raio e sombra | 5 px; `0 8px 24px rgba(0,0,0,.4)` |
| Bordas | contorno padrão | 1 px, foreground a 10% |
| Cards | card recente | 14 px interno, raio 8 px, sem sombra |

## Cores por tema

Os valores são os tokens exportados pelo ZIP; manter os tokens semânticos para temas dinâmicos.

| Token | Phenomenon (`:root`) | Escuro (`data-theme=dark`) | Claro (`data-theme=light`) |
|---|---|---|---|
| `--warp-background` | `#121212` | `#050505` | `#FFFFFF` |
| `--warp-foreground` | `#FAF9F6` | `#FFFFFF` | `#111111` |
| `--warp-accent` | `#2E5D9E` | `#19AAD8` | `#00C2FF` |
| `--warp-accent-bright` | `#3780E9` | `#19AAD8` | `#008EC4` |
| `--neutral-1` | `#1E1E1D` | `#121212` | `#F3F3F3` |
| `--neutral-2` | `#292929` | `#1E1E1E` | `#E7E7E7` |
| `--neutral-3` | `#353534` | `#2B2B2B` | `#DBDBDB` |
| `--neutral-4` | `#404040` | `#373737` | `#CFCFCF` |
| `--text-main/sub/hint` | foreground 90/60/40% | foreground 90/60/40% | foreground 90/60/40% |
| `--border-outline` | foreground 10% | foreground 10% | foreground 10% |
| `--border-split-pane` | foreground 15% | foreground 15% | foreground 15% |
| `--accent-overlay-2` | accent 25% | accent 25% | accent 25% |
| `--fg-overlay-1/2/3` | foreground 5/10/15% | foreground 5/10/15% | foreground 5/10/15% |

Status fixo: warning `#C28000`, error `#BC362A`, yellow `#E5A01A`, green `#1CA05A`, success `#008E41`. ANSI status dos agentes vem do token `--ansi-*`, não dos hex da marca.

## Mapeamento para WarpUI

O protótipo usa CSS variables, CSS `color-mix`, transparências e eventos DOM. A implementação nativa deve mapear essas operações para `Appearance::theme()` e componentes nativos, calculando a mesma composição de alpha sobre o fundo correto. Não converter `fg-overlay-*` em cores absolutas que quebrem temas criados pelo usuário.

Usar os componentes de `warpui::ui_components`/`app/src/ui_components` quando a aparência e estados correspondem. Quando não houver equivalente fiel, criar uma peça local pequena e específica em `app/src/workspace/view/local_workspace/`. Não alterar globalmente fontes, animações, botões, hovers ou dimensões dos terminais para corrigir somente o workspace.

## Diferenças de identidade a preservar

- `--kt-yellow`, `--kt-yellow-bright`, `--kt-ink`, `--kt-panel`, `--kt-steel`, `--kt-coral` e `--kt-mist` pertencem a superfícies de marca, fora do chrome cotidiano.
- Logo `kikito-terminal-icon.png` é raster. Não redesenhar ou substituir por um ícone aproximado.
- Ícones do workspace vêm do conjunto mono SVG do ZIP e são tintados pelo contexto. Evitar glifos substitutos como `⋯` quando existe o ícone indicado (`dots-horizontal`, `plus`, `gear`, `folder`, `git-branch-02`, `x-close`).
