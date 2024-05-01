use leptos::{component, view, IntoView};

use crate::components::{
    cards::card_title::CardTitle,
    icons::{DiscordIcon, GithubIcon, TelegramIcon, WebIcon},
};

#[component]
pub fn CommunityCard(
    #[prop(into)] name: &'static [&'static str],
    #[prop(into)] description: &'static str,
    #[prop(into)] link: &'static str,
    #[prop(into)] icon: &'static str,
    #[prop(into)] brand_src: String,
    #[prop(into, optional)] brand_alt: Option<&'static str>,
) -> impl IntoView {
    view! {
        <a
            href=link
            target="_blank"
            class="group flex flex-col gap-y-6 border border-black p-6 hover:bg-orange-500 bg-orange-50 dark:hover:bg-zinc-900/40 dark:bg-black/40 drop-shadow-[0_0_0_rgba(0,0,0)] hover:drop-shadow-[-4px_-4px_0_rgba(0,0,0)] transition justify-between"
        >
            <div>
                <img
                    src=brand_src
                    width="60"
                    height="60"
                    class="rounded-full mb-4"
                    loading="lazy"
                    alt=brand_alt
                />
                <CardTitle texts=name/>
                <p class="font-work-sans text-black dark:text-white">{description}</p>
            </div>
            <span class="ml-auto">
                {move || match icon {
                    "discord" => view! { <DiscordIcon size=30/> },
                    "github" => view! { <GithubIcon size=30/> },
                    "telegram" => view! { <TelegramIcon size=30/> },
                    "web" => view! { <WebIcon size=30/> },
                    _ => unreachable!(),
                }}

            </span>
        </a>
    }
}
