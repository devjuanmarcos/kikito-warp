use warp_core::channel::{Channel, ChannelState};
use warp_core::context_flag::ContextFlag;
use warp_core::features::FeatureFlag;
use warpui::ViewContext;

use super::{
    ContentItem, ContentSectionData, FeatureItem, FeatureSection, FeatureSectionData,
    ResourceCenterMainView, Section, Tip, TipAction, TipHint,
};

pub fn sections(ctx: &mut ViewContext<ResourceCenterMainView>) -> Vec<Section> {
    let is_oss = ChannelState::channel() == Channel::Oss;
    let mut sections = if is_oss {
        Vec::new()
    } else {
        vec![Section::Changelog()]
    };

    if FeatureFlag::AvatarInTabBar.is_enabled() {
        return sections;
    }

    let get_started = FeatureSectionData {
        section_name: FeatureSection::GettingStarted,
        items: vec![
            FeatureItem::new(
                "Crie seu primeiro bloco",
                "Execute um comando para ver o comando e a saída organizados em um bloco.",
                Tip::Hint(TipHint::CreateBlock),
                ctx,
            ),
            FeatureItem::new(
                "Navegue pelos blocos",
                "Clique em um bloco para selecioná-lo e navegue usando as setas.",
                Tip::Hint(TipHint::BlockSelect),
                ctx,
            ),
            FeatureItem::new(
                "Ações nos blocos",
                "Clique com o botão direito em um bloco para copiar seu conteúdo e acessar outras ações.",
                Tip::Hint(TipHint::BlockAction),
                ctx,
            ),
            FeatureItem::new(
                "Abrir a paleta de comandos",
                "Acesse todos os recursos do Kikito Terminal pelo teclado.",
                Tip::Action(TipAction::CommandPalette),
                ctx,
            ),
            FeatureItem::new(
                "Escolha seu tema",
                "Personalize o Kikito Terminal escolhendo um tema.",
                Tip::Action(TipAction::ThemePicker),
                ctx,
            ),
        ],
    };
    sections.push(Section::Feature(get_started));

    let maximize_warp = FeatureSectionData {
        section_name: FeatureSection::MaximizeWarp,
        items: maximize_warp_items(ctx),
    };
    sections.push(Section::Feature(maximize_warp));

    if !is_oss {
        let advanced_setup = ContentSectionData {
            section_name: FeatureSection::AdvancedSetup,
            items: vec![
                ContentItem {
                    title: "Use your custom prompt",
                    description: "Set up Warp to honor your PS1 setting",
                    url: "https://docs.warp.dev/terminal/appearance/prompt",
                    button_label: "View documentation",
                },
                ContentItem {
                    title: "Integrate Warp with your IDE",
                    description: "Configure Warp to launch from your most used development tools",
                    url: "https://docs.warp.dev/terminal/integrations-and-plugins",
                    button_label: "View documentation",
                },
                ContentItem {
                    title: "How Warp uses Warp",
                    description: "Learn how Warp's engineering team uses their favorite features",
                    url: "https://www.warp.dev/blog/how-warp-uses-warp",
                    button_label: "Read article",
                },
            ],
        };
        sections.push(Section::Content(advanced_setup));
    }

    sections
}

fn maximize_warp_items(ctx: &mut ViewContext<ResourceCenterMainView>) -> Vec<FeatureItem> {
    let mut maximize_warp_items = vec![];

    maximize_warp_items.push(FeatureItem::new(
        "Busca de comandos",
        "Encontre e execute novamente comandos anteriores.",
        Tip::Action(TipAction::CommandSearch),
        ctx,
    ));

    maximize_warp_items.push(FeatureItem::new(
        "Busca de comandos com IA",
        "Gere comandos de shell usando linguagem natural.",
        Tip::Action(TipAction::AiCommandSearch),
        ctx,
    ));

    if ContextFlag::CreateNewSession.is_enabled() {
        maximize_warp_items.push(FeatureItem::new(
            "Dividir painéis",
            "Divida as abas em vários painéis para montar o layout ideal.",
            Tip::Action(TipAction::SplitPane),
            ctx,
        ));
    }

    if ContextFlag::LaunchConfigurations.is_enabled() {
        maximize_warp_items.push(FeatureItem::new(
            "Configuração de inicialização",
            "Salve a configuração atual de janelas, abas e painéis.",
            Tip::Action(TipAction::SaveNewLaunchConfig),
            ctx,
        ));
    }

    maximize_warp_items
}
