use futures::future::join_all;
use leptos::{error::Result, *};
use serde::{Deserialize, Serialize};

use crate::components::ContributorCard;

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Contributor {
    login: String,
    avatar_url: String,
    html_url: String,
    bio: Option<String>,
    twitter_username: Option<String>,
    location: Option<String>,
}

async fn fetch_contributors() -> Result<Vec<Contributor>> {
    let response = reqwasm::http::Request::get(
        "https://api.github.com/repos/RustLangES/rustlanges.github.io/contributors",
    )
    .send()
    .await?
    .json::<Vec<Contributor>>()
    .await?;

    let response = join_all(
        response
            .into_iter()
            .map(|contributor| fetch_contributor_info(contributor.login.clone())),
    ).await;

    let response = response
        .into_iter()
        .filter_map(|contributor| contributor.ok())
        .collect::<Vec<Contributor>>();

    Ok(response)
}

async fn fetch_contributor_info(username: String) -> Result<Contributor> {
    let response = reqwasm::http::Request::get(&format!(
        "https://api.github.com/users/{}",
        username
    ))
    .send()
    .await?
    .json::<Contributor>()
    .await?;
    Ok(response)
}

#[component]
pub fn Contributors() -> impl IntoView {
    let contributors_results = create_local_resource(move || (), |_| fetch_contributors());
    let contributorMapper = |item: &Contributor| {
        view! {
            <ContributorCard
                name=item.login.clone()
                description=item.bio.clone()
                link=item.html_url.clone()
                brand_src=item.avatar_url.clone()
                twitter=item.twitter_username.clone()
                location=item.location.clone()
            />
        }
    };

    let contributors_view = move || {
        let contributors = contributors_results.get()?.ok()?;
        let result = contributors
            .iter()
            .map(contributorMapper)
            .collect::<Fragment>();
        Some(result.into_view())
    };

    view! {
        <section class="bg-orange-300/30 py-16">
            <div class="flex flex-col gap-y-6 container mx-auto px-4">
                <h2 class="text-3xl text-left mb-6">
                    <span class="font-work-sans font-light">"Nuestros "</span>
                    <span class="font-alfa-slab text-orange-500">"Colaboradores"</span>
                </h2>
                <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 2xl:grid-cols-5 gap-6">
                    {contributors_view}
                </div>
            </div>
        </section>
    }
}
