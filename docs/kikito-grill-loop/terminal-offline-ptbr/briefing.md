# Terminal independente e em português

## Objetivo

Evoluir o Kikito Terminal para uma experiência independente dos serviços da Warp, utilizável sem conexão externa, sem exigir login Warp e com toda a interface em português brasileiro.

## Resultado esperado

- O terminal local abre e permite usar shells, sessões, abas, configurações e restauração local sem rede ou conta.
- Nenhum fluxo de inicialização, login, logout ou uso básico abre páginas ou faz chamadas a servidores da Warp.
- Recursos dependentes de serviço remoto são removidos do modo offline ou substituídos por serviços próprios quando essa opção for definida.
- A interface visível ao usuário, inclusive mensagens, menus, configurações e acessibilidade, fica em pt-BR.

## Usuário e problema

Juan quer desenvolver e usar uma distribuição própria do terminal sem depender da identidade, autenticação ou infraestrutura de terceiros.

## Escopo

- Identificar e separar as dependências de autenticação, API, GraphQL, sincronização, colaboração, IA remota, atualizações, telemetria e navegação externa.
- Manter o terminal local e os dados armazenados localmente.
- Adotar pt-BR como idioma da interface.
- Preparar uma arquitetura em que os pontos de integração próprios possam ser configurados sem chamadas ocultas à infraestrutura Warp.

## Fora de escopo nesta etapa

- Criar um backend Kikito, serviço de contas, sincronização ou modelo de IA.
- Migrar dados existentes, apagar bancos locais, alterar credenciais ou publicar instaladores.
- Traduzir documentação técnica de dependências externas ou nomes de protocolos que não aparecem na interface.

## Decisão confirmada

O primeiro resultado será local/offline. Não haverá backend Kikito neste momento. Removeremos dependências dos servidores Warp, telemetria, relatórios de falha, atualizações automáticas, conexões externas iniciadas pelo aplicativo e fluxo de autenticação Warp. O núcleo do produto será gerenciar terminais e agilizar processos locais. Recursos remotos poderão ser reconsiderados quando houver uma proposta própria que faça sentido.

## Entrega incremental

Esta é uma mudança ampla em um aplicativo grande. A primeira etapa fecha os pontos de inicialização e de rede do canal OSS e preserva os dados existentes; as etapas seguintes removem telas e recursos remotos restantes e traduzem a interface para pt-BR por áreas. A GUI desktop é o alvo inicial; a inclusão do TUI será definida após inventariar sua paridade e seus pontos de rede.
