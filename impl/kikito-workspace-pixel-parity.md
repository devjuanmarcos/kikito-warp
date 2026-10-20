# Kikito Terminal Workspace: paridade visual e interacional

## Objective

Refazer a casca visual de organizações, projetos, abas, Home, diálogos, menus e painel Source Control do Kikito Terminal a partir do ZIP de design, corrigir os cliques inoperantes e obter pelo menos 98% de paridade visual em cada superfície antes de iniciar a seguinte.

## Business context

O usuário organiza terminais por organização e projetos. Cada projeto referencia um diretório local e contém abas/sessões de terminal. A implementação atual não responde adequadamente a vários cliques e diverge do design recebido. O fluxo precisa continuar local, persistente e compatível com as sessões abertas. Os nomes, caminhos, branches e conteúdo real de terminal continuam sendo dados do usuário; nenhuma fixture do protótipo pode aparecer como dado de produção.

## Repository context relevant to the feature

O app é Rust/WarpUI. A montagem está em `app/src/workspace/view.rs` e `app/src/workspace/view/local_workspace/`. Organizações, projetos, seleção e tabs usam `app/src/local_workspace/model.rs`; persistência local usa `crates/persistence/`. As rotas e ações ficam em `app/src/workspace/action.rs`. Git local existente fica em `app/src/local_git/` e a interface em `app/src/workspace/view/local_workspace/source_control/`.

O protótipo fornecido é `C:\Users\Marcos\Downloads\✅ Kikito Terminal.zip` (SHA-256 `8CCDB225255F422E72B13915DE815911B62092345B73F31374B98BA6A8D09474`). `ui_kits/workspace/index.html` declara 1280×800. A especificação visual derivada está em `docs/design-harvest/kikito-workspace/`.

### Reuse candidates

- `WorkspaceModel`, IDs, persistência e roteamento de seleção existentes.
- `Modal<CreateDialog>`, seletor de pasta, cropper e validação nativa já implementados.
- `Appearance::theme()` e funções de tokens do tema; manter cores derivadas do tema.
- `warpui::ui_components` e `app/src/ui_components/` para botão, input, tooltip, menu, dialog, color dot, icon-with-status e shortcut.
- `app/src/local_git/` e `source_control/` para operações reais. React `GitData.jsx`/`GitPanel.jsx` são demonstrações com fixtures e temporizadores, não serviços a portar.

## Scope

- Diagnóstico reprodutível do hit testing e da camada que hoje bloqueia cliques.
- Refazer a apresentação do trilho de organizações, sidebar de projetos, barra de abas, Home, formulários de organização/projeto, recorte de imagem, menus e painel Source Control.
- Preservar o modelo local, as relações de tabs, processos/sessões, diretórios e operações Git existentes.
- Manter todos os textos novos em pt-BR e alinhar ícones, fontes, cores, dimensões, espaçamentos, hover, foco, seleção, vazio, loading, erro e scroll com o ZIP.
- Trabalhar serialmente: cada superfície deve alcançar 98% e passar sua bateria de interações antes da seguinte.

## Out of scope

- Migrar o app desktop para React ou introduzir webview.
- Alterar o desenho global de terminal, blocos, fontes e animações fora do workspace.
- Implementar backend Kikito, login, telemetria, crash reporting, sync ou atualizador.
- Portar `localStorage`, conteúdo demo, branches fictícias, commits sintéticos ou timers do protótipo.
- Avançar em funcionalidades Git além das já aprovadas/implementadas enquanto os fluxos de organização e projeto não passarem os gates visuais.
- Reinterpretar os registros históricos da tabela `projects`.

## Business rules

1. Organizações agrupam projetos; projeto pertence a uma organização e aponta para um diretório local.
2. Remover/editar metadados nunca move nem apaga diretórios e nunca encerra sessões de terminal.
3. Cada aba pertence a no máximo um projeto; selecionar projeto restaura o conjunto e a aba ativa persistidos.
4. Remover a seleção atual escolhe uma organização válida ou Home e não deixa project/tab selection órfã.
5. Criar projeto valida caminho existente e diretório. Não criar caminho de exemplo `~/slug` implicitamente no app real.
6. Cliques de controles aninhados não podem acionar a linha de fundo; menus/diálogos precisam ter hit regions e z-order corretos.
7. A aparência é derivada do tema. Amarelo KT só aparece nos ativos/superfícies de marca previstos.
8. Nenhum item Git de fixture é apresentado como resultado real.
9. Operações remotas só executam após ação explícita e destino correto visível.

## User flows

1. Abrir Home; pesquisar e abrir projeto recente ou aba recente.
2. Selecionar/criar organização; editar/remover via clique direito ou menu explícito.
3. Buscar/criar/selecionar projeto; editar/remover projeto; criar nova aba pelo row ou pela barra.
4. Alternar e fechar abas; restaurar aba ativa por projeto; criar terminal para projeto vazio.
5. Selecionar imagem, recortar/zoom/pan, aplicar ou cancelar; persistir só resultado válido.
6. Abrir/fechar menu por mouse e teclado, inclusive fora da tela/área rolada.
7. Abrir Source Control do projeto ativo, revisar status Git real e executar somente ações disponíveis no app.

## UI/screens involved

1. Organization rail (64 px): Home, tiles, divisor, criação, settings, marcação ativa/hover, imagem/inicial, badge, tooltip.
2. Project sidebar (248 px): header da organização, busca, contagem, rows, branch, ações no hover, nested tabs opcional, estados empty/no results, footer Novo projeto.
3. Project tab bar (34 px): tab 72–220 px, ícone/estado, fechamento, adicionar, launcher, Git.
4. Home: coluna max 960 px, busca 44 px, até 6 cards e 7 linhas de tabs recentes.
5. Project canvas: sessão ativa e estado sem tabs.
6. Create/edit/remove dialogs de organização/projeto e image cropper.
7. Context menus de organização/projeto e teclado.
8. Source Control panel e grafo de commits.

Valores exatos, cores, componentes e estados por superfície estão em `docs/design-harvest/kikito-workspace/03-components.md`.

## Frontend implementation notes

Fazer o layout primeiro e isolar renderers por superfície para permitir estados e testes separados. Manter a casca em `Flex`/`Container` existentes sem estender o hit region do canvas sobre rail/sidebar. Conferir ordem de dispatch, z-index, hit testing, dimensões após layout e `StopPropagation` dos filhos interativos. Não adicionar handler global que consuma mouse-up para a janela inteira.

Reusar componentes compartilhados quando a geometria e estados forem equivalentes. Criar primitives locais pequenas quando o componente não der a forma exata. Não modificar tokens globais para ajustar somente esta feature. O frame de 1400×900/controles de janela pertence à demo; não duplicar o chrome nativo. Respeitar o viewport principal 1280×800 e o uso real da janela.

Registrar screenshots pareados por etapa e manter um `parity-report.md` com bounding boxes, fontes carregadas, image diff, estados, máscaras e score. A comparação exige mesma resolução, tema, UI scale, conteúdo controlado e cursor fora de elementos, exceto na captura explícita de hover.

## Backend/API needs

Nenhuma API ou serviço remoto. Usar ações/métodos existentes do modelo local e comandos Git locais. Não adicionar rede, login, telemetria, analytics ou update check.

## Data model impact

Nenhuma alteração de schema é esperada para a refação visual. Reutilizar as migrations e a relação de projeto/aba já presentes. Uma mudança de modelo só entra se o diagnóstico demonstrar estado necessário ausente; deve preservar os registros existentes e ganhar teste de persistência/restauração.

## State management impact

Selection e dados persistentes permanecem em `WorkspaceModel`. Hover, popover, expansão de seções, busca temporária, imagem em processamento e confirmação ficam em estado de view apropriado. Tarefas async devem ter generation/cancelamento para descartar resultados após fechar diálogo ou trocar alvo. Não duplicar seleção persistente em estado visual local.

## Validations

- Nome de organização/projeto não vazio e trim conforme modelo existente.
- Diretório selecionado precisa existir e ser diretório; erros de permissão e caminho ausente distintos.
- Imagem: PNG/JPG/SVG, limite de leitura, decode/raster limit, cancelamento e crop final 128×128 PNG.
- Ação de menu verifica que target ainda existe no model quando ativada.
- Git valida que o caminho do repositório ainda é o projeto selecionado antes de mutar dados.

## Permissions and access rules

Todos os dados são locais ao usuário. CRUD de metadados não altera permissões nem arquivos do projeto. Diálogos de confirmação são obrigatórios para remover organizações/projetos e descartar conteúdo Git.

## Edge cases

- Nenhuma organização; organização sem projeto; projeto sem terminal; query vazia/sem resultados.
- Muitos tiles/rows; busca preserva seleção; scroll com menu/contexto aberto.
- Nomes, branches, caminhos e títulos longos; espaços, acentos e Unicode.
- Organização sem imagem/com imagem inválida; imagem grande; SVG inválido; cancelamento no meio do processamento.
- Projeto removido ou diretório desaparece enquanto selecionado; falha de persistência.
- Duplo clique, mousedown/up em alvos diferentes, controle aninhado, menu perto da borda, janela inativa e escala >100%.
- Agente em running/blocked/success, sem status e sem agentes.
- Repositório Git ausente ou indisponível; conflito; branch longa; status async obsoleto.

## Error states and fallback behavior

Mostrar erro localizado junto ao formulário ou superfície que falhou; manter valores digitados e seleção quando recuperável. Picker cancelado não é erro. Persistência que falha não deve atualizar estado local como se tivesse sucesso. Path checker apresenta Checking/Available/Missing/NotDirectory/Inaccessible. Git indisponível mantém o terminal utilizável e mostra status específico; não exibir snapshot velho como se pertencesse ao projeto atual.

## Analytics/events tracking needs

Nenhum evento. O produto permanece sem telemetria.

## Accessibility requirements

- Nome acessível em todos os botões apenas com ícone; estado selecionado exposto no tree e foco visível.
- Ordem de Tab coerente do rail ao canvas e às ações; Home/menus/dialog têm atalhos explicados.
- Menus suportam Up/Down/Enter/Space/Escape e clique fora; foco volta ao acionador.
- Diálogos prendem foco, começam no campo principal e descrevem confirmação/erro.
- Não depender só de cor para active/status; oferecer label/tooltip.
- Os controles visíveis apenas no hover também aparecem com teclado e touch.
- Honrar movimento reduzido sem alterar animações globais.

## Acceptance criteria

- [ ] O bug de cliques é reproduzido e sua causa é identificada antes do refactor.
- [ ] Rail tem score visual ≥98% e todos os controles passam mouse/teclado antes do início da sidebar.
- [ ] Project sidebar tem score ≥98% e seleção/busca/menus/CRUD passam antes da barra de abas.
- [ ] Tab bar/canvas, Home, diálogos/crop e menus alcançam score ≥98% em sequência, cada uma fechando seu próprio gate.
- [ ] Source Control visual alcança score ≥98% sem fixtures; status e ações vêm do Git real.
- [ ] Comparações usam 1280×800, DPR/escala, tema, fonte, conteúdo e estado equivalentes. Métrica visual registra changed-pixel ratio com limiar de canal e limitações; bounding boxes críticos diferem no máximo 1 px.
- [ ] Score não mascara nenhum texto estável, ícone, borda, espaçamento, botão, hover ou estado selecionado; regiões dinâmicas são declaradas previamente.
- [ ] Criar/editar/remover não apaga/move diretórios e não perde sessões/tabs.
- [ ] Não há alvo inoperante, hit area sobreposta, menu preso nem camada invisível interceptando cliques.
- [ ] Nenhuma mudança altera visual global do terminal, suas fontes ou animações.

## Implementation checklist

### Frontend
- [ ] Reproduzir e registrar hit test, z-order e ação dispatch antes de mudanças.
- [ ] Refazer rail e fechar gate ≥98%.
- [ ] Refazer project sidebar e fechar gate ≥98%.
- [ ] Refazer tab bar/canvas e fechar gate ≥98%.
- [ ] Refazer Home/launcher e fechar gate ≥98%.
- [ ] Refazer dialogs/crop/menu e fechar seus gates ≥98%.
- [ ] Ajustar painel e grafo Git ao source, sem dados simulados.

### Backend
- [ ] Nenhuma API nova; manter execução no processo local.

### Data / Contracts
- [ ] Reutilizar IDs, estado SQLite e relações existentes.
- [ ] Adicionar alteração de modelo somente com prova da necessidade e migração backward-safe.

### Permissions
- [ ] Confirmar antes de remover metadados ou conteúdo Git destrutivo.
- [ ] Não apagar diretório do usuário durante CRUD.

### Analytics
- [ ] Não adicionar telemetria.

### Tests
- [ ] Testar cada ação de clique com evento e resultado observável.
- [ ] Testar mouse, teclado, foco, hover, click-outside e scroll dos menus/dialogs.
- [ ] Testar persistência, reabertura e isolamento de tabs por projeto.
- [ ] Testar imagens, validação de path, falhas async e cancelamento.
- [ ] Testar Source Control contra repositórios temporários reais.
- [ ] Capturar e comparar screenshots por estado; guardar score/dimensões.

## Testing checklist

- [ ] Unit tests para seleção, filtragem, estado de menu, recent sorting, crop geometry e rotas.
- [ ] GUI integration test para hit targets, clique em rail/sidebar/tab/dialog/menu e event propagation.
- [ ] Smoke visual do app a 1280×800 em tema escuro e de conferência no claro.
- [ ] Validar create/edit/remove e que nenhum diretório/sessão é apagado.
- [ ] Testar imagens válidas/inválidas/limite e crop.
- [ ] Testar estado vazio, carregando, erro e conteúdo para cada superfície.
- [ ] Testar acessibilidade com teclado/foco/labels.
- [ ] Registrar diferença visual e revisar cada região fora do limite.

## Risks/dependencies

- A demonstração depende de CDN e não inclui captura integral renderizada; a sessão de comparação precisa carregar React/Babel, fonts e assets antes de emitir qualquer score.
- WarpUI tem regras próprias de hit testing e texto; comportamento em release pode diferir de debug. Os testes precisam usar eventos GUI reais e configuração equivalente à build entregue.
- Antialiasing do texto pode afetar diff entre browser React e renderer GPU. Registrar limiar e combinar pixel diff com limites geométricos; não mascarar fontes erradas.
- O app instalado pode estar em versão anterior ao checkout. Capturar identificação/build antes de atribuir defeito à fonte atual.
- `app/src/workspace/view/local_workspace/mod.rs` e arquivos Git estão modificados no workspace antes desta tarefa; preservar essas mudanças e revisar diffs antes de substituir módulos.

## Open Questions

- Nenhuma decisão bloqueante para começar. Pormenores de mobile não existem no ZIP; esta entrega usa desktop 1280×800 como referência e documenta outros tamanhos como QA de integração.

## Assumptions

- 98% será medido com captura pareada e métricas registradas; toda superfície tem gate independente.
- O chrome real da janela do app substitui o cabeçalho demo de tráfego da OrgRail quando houver sobreposição.
- O usuário autorizou refazer as telas de organização/projeto e atuar na branch atual sem esperar aprovação intermediária.
