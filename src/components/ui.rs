use leptos::prelude::*;
use leptos_router::components::A;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum ControlSize {
    Compact,
    #[default]
    Standard,
}

impl ControlSize {
    pub const fn class_name(self) -> &'static str {
        match self {
            Self::Compact => "control-frame--compact",
            Self::Standard => "control-frame--standard",
        }
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum ControlTone {
    #[default]
    Primary,
    Secondary,
    Quiet,
}

impl ControlTone {
    pub const fn class_name(self) -> &'static str {
        match self {
            Self::Primary => "control-frame--primary",
            Self::Secondary => "control-frame--secondary",
            Self::Quiet => "control-frame--quiet",
        }
    }
}

#[component]
pub fn ActionLink(
    #[prop(into)] href: String,
    #[prop(default = ControlSize::Standard)] size: ControlSize,
    #[prop(default = ControlTone::Primary)] tone: ControlTone,
    children: Children,
) -> impl IntoView {
    let class = format!("control-frame {} {}", size.class_name(), tone.class_name());

    view! {
        <A href=href attr:class=class>
            {children()}
        </A>
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EvidenceKind {
    Source,
    Browser,
    Local,
    Provider,
    Unproven,
}

impl EvidenceKind {
    const fn class_name(self) -> &'static str {
        match self {
            Self::Source => "evidence-tag--source",
            Self::Browser => "evidence-tag--browser",
            Self::Local => "evidence-tag--local",
            Self::Provider => "evidence-tag--provider",
            Self::Unproven => "evidence-tag--unproven",
        }
    }

    const fn label(self) -> &'static str {
        match self {
            Self::Source => "Source-derived",
            Self::Browser => "Browser-observed",
            Self::Local => "Locally verified",
            Self::Provider => "Provider-observed",
            Self::Unproven => "Unproven",
        }
    }
}

#[component]
pub fn EvidenceTag(kind: EvidenceKind) -> impl IntoView {
    let class = format!("evidence-tag {}", kind.class_name());
    view! { <span class=class>{kind.label()}</span> }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn control_size_classes_are_canonical() {
        assert_eq!(ControlSize::Compact.class_name(), "control-frame--compact");
        assert_eq!(
            ControlSize::Standard.class_name(),
            "control-frame--standard"
        );
    }

    #[test]
    fn control_tone_classes_are_canonical() {
        assert_eq!(ControlTone::Primary.class_name(), "control-frame--primary");
        assert_eq!(
            ControlTone::Secondary.class_name(),
            "control-frame--secondary"
        );
        assert_eq!(ControlTone::Quiet.class_name(), "control-frame--quiet");
    }
}
