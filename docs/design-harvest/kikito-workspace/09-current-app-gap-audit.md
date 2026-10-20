# Auditoria do workspace atual

## Escopo e certeza

Esta auditoria compara o protótipo do ZIP com os módulos Rust/WarpUI presentes no checkout em 19/10/2026. É leitura estática do código: registra diferenças provadas pelo código e não atribui a elas os relatos de clique sem reprodução em janela real. Os arquivos Git já modificados no checkout foram preservados.

## Divergências observadas

| Área | Referência do ZIP | Checkout atual | Implicação |
| --- | --- | --- | --- |
| Home e sidebar | Home ocupa a área principal sem a sidebar de projetos. | `local_workspace::render` sempre adiciona `org_rail::render` e `project_sidebar::render` antes de compor o conteúdo principal. | O modo Home mantém uma coluna lateral que não existe no protótipo. |
| Trilho | 64 px, 36 px por tile, marcador de seleção à esquerda, divider, `+` junto das organizações e engrenagem no rodapé. | Largura nominal 64 px e tiles de 36 px; `+` e engrenagem ficam ambos no footer; indicador ativo é borda/preenchimento do tile, sem marcador lateral. | Posição e estado visual precisam ser reconstruídos conforme a ordem da referência. |
| Cabeçalho do rail | O mock tem três círculos de janela em 34 px. A captura do protótipo inclui esse chrome. | `org_rail::render` não desenha a linha de círculos. O app tem chrome nativo próprio. | Verificar se o chrome nativo é o equivalente correto antes de reproduzir os círculos e criar duplicação. |
| Organização | Tiles com imagem ou inicial, cor de acento, badge para agente bloqueado e hover que expõe menu. | Tiles usam imagem ou inicial e acento; há hover e menu contextual. O rail mostrado não compõe badge de agente bloqueado. | Badge/status e hit areas precisam ser incluídos e verificados. |
| Sidebar de projetos | Existe só com organização selecionada; cabeçalho da organização, busca, seção, cards, paths/branch, ações no hover e footer fixo. | A sidebar é sempre montada; em Home lista projetos globais. Busca por nome/caminho existe, mas não cobre o conjunto de estados do protótipo. | Separar visibilidade de Home e organização e portar os estados restantes da busca. |
| Navegação de projeto/tab | Clique em projeto controla a área principal e o vínculo das tabs; tabs ficam no header do projeto. | A navegação por ação está presente, mas tab bar e seleção ainda residem em `local_workspace/mod.rs`; a barra atual usa nomes genéricos `Terminal N` e controles diferentes do mock. | Refatorar a apresentação após fechar gates do rail e da sidebar sem perder identidade de sessão. |
| Menus | `Menu` do kit, largura 186 px, itens e hover próprios; botão de menu e clique direito. | `context_menu.rs` cria linhas próprias em `Flex`, sem largura fixa explícita, e só o menu em si trata Escape. | Menu precisa de geometria/posicionamento e foco cobertos por teste antes de dizer que todos os itens clicam. |

## Rastreio de eventos encontrado

- Os tiles do rail e as linhas de projeto criam `EventHandler` local com `on_left_mouse_up`, despacham ações de workspace e retornam `StopPropagation`.
- O botão de menu do rail e o menu contextual também têm handlers locais.
- `view.rs` instala um handler no stack de workspace; quando um menu está aberto, ele fecha no `left_mouse_up` e usa `StopPropagation`, além de tratar Escape e setas.
- A composição de UI por si só não demonstra qual handler recebe primeiro o evento em execução. A captura automatizada do protótipo via CSS locator também ficou sem resultado utilizável; ela não foi usada para inferir uma causa para os cliques relatados.

## Próxima evidência necessária

Antes de corrigir o problema de interação, capturar o app nativo com perfil isolado e registrar, para rail Home, tile de organização, botão de menu e projeto: bounds do elemento, posição do ponteiro, elemento sob o ponteiro, evento recebido, ação despachada e estado do modelo depois do clique. A hipótese deve ser testada com um único handler/overlay por vez. Reutilizar o harness de GUI existente e não abrir nem escrever no perfil do app instalado.

## Ordem de execução

1. Reproduzir e localizar a falha de hit testing; preservar a sessão/abas atuais.
2. Trilho de organizações: geometria, imagem/inicial, badge, menu e hit targets; atingir o gate do plano.
3. Sidebar de projetos; só começar depois de rail passar seu gate.
4. Barra de tabs e canvas; só começar depois de sidebar passar seu gate.
5. Home; depois formulários/cropper, menus/overlays e Git Source Control.

As dimensões e critérios completos permanecem na [especificação](../../../impl/kikito-workspace-pixel-parity.md) e no [plano](../../superpowers/plans/2026-10-19-kikito-workspace-pixel-parity.md). Esta auditoria não afirma que os bugs de interação foram corrigidos nem que a implementação já atingiu 98%.
