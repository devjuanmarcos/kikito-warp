# Ativos e uso

## Inventário do ZIP

O ZIP contém 121 SVG, 11 PNG, 5 JPG, 10 arquivos TTF, 40 JSX, 27 HTML, 27 Markdown, 24 declarações TypeScript, 6 CSS, 2 JSON e um bundle JavaScript, além de `.thumbnail`. Ele inclui o design system e outros kits do Kikito Terminal; somente subconjuntos abaixo pertencem ao workspace.

| Ativo | Uso no desenho | Tratamento no app |
|---|---|---|
| `assets/logo/kikito-terminal-icon.png` | Home do rail e ilustração Home | Asset de marca original; não redesenhar |
| `assets/logo/kikito-app-icon-512.png` | referência do ícone do app | asset de marca, não cor de superfície de sidebar |
| `assets/icons/*.svg` | ações, menu, Git, busca, terminal, status | mapear para `Icon`; tintar com cor contextual |
| `fonts/Roboto-*.ttf` | todos os textos de chrome | comparar com fontes incluídas no app; não usar fallback se a fonte equivalente estiver disponível |
| `fonts/Hack-*.ttf` | caminho, branch, hash e conteúdo terminal | preservar a fonte monoespaçada atual do app |
| `uploads/pasted-1792329106410-0.png` | referência visual complementar do Source Control VS Code | manter como prova de referência, não como UI/asset do produto |
| `assets/backgrounds/*.jpg` | opções de tema pré-existentes no kit global | não introduzir no chrome do workspace |
| `assets/illustrations/*.png` | onboarding e telas do kit global | fora da navegação de organizações/projetos |
| `assets/logo/legacy-warp/*` | identificação antiga do fork | referência histórica apenas; não renderizar |
| `assets/logo/warp.svg` | integração/zero-state Warp | fora desta feature e sujeito à política de marca já aplicada no app |

## Ícones usados diretamente nas telas

`agentmode`, `plus`, `gear`, `folder`, `git-branch-02`, `terminal`, `x-close`, `search`, `pencil-02`, `trash-02`, `dots-horizontal`, `upload-01`, `image-01`, `minus`, `check`, `refresh`, `download-01`, `chevron-down`, `chevron-right`, `cloud-01`, `bookmark`, `file`, `retry`. Status de agente utiliza o componente `IconWithStatus`, não emoji ou glyph improvisado.

No código Rust, validar os nomes com `warp_core::ui::Icon`. Se um SVG não existir no enum atual, confirmar um símbolo próprio equivalente nos assets antes de substituir; registrar a correspondência. Evitar texto `⋯` como substituto do botão dots-horizontal.

## Imagens de organização

O protótipo aceita file input `image/*` e drag-and-drop, mas não implementa validação completa. A app precisa manter o fluxo nativo seguro: PNG/JPG/SVG, limite de tamanho, leitura limitada mesmo se o arquivo crescer, decode/raster limitados, recorte de saída PNG 128×128, processamento fora da thread de UI, erros e cancelamento visíveis. Persistir apenas a imagem final/crop e metadados já suportados pela migration local. O original não deve ser usado como estado confiável depois do encerramento do diálogo.

## Licença e cópia

Os ativos foram fornecidos no ZIP da identidade do Kikito Terminal. A implementação deve reutilizar os arquivos de marca licenciados pelo usuário e os SVG já existentes no app sempre que forem equivalentes. Esta documentação não autoriza copiar fontes ou conteúdo de terceiros fora do arquivo. Manter atribuições/licenças já inclusas no repositório.
