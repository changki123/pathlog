use leptos::*;
use leptos_router::*;
use serde::Deserialize;
use wasm_bindgen_futures::spawn_local;

#[derive(Clone, Deserialize)]
struct Video {
    id: String,
    title: String,
    description: String,
    date: String,
    src: String,
    thumb: String,
}

async fn fetch_videos() -> Vec<Video> {
    match gloo_net::http::Request::get("/videos.json").send().await {
        Ok(resp) => resp.json::<Vec<Video>>().await.unwrap_or_default(),
        Err(_) => vec![],
    }
}

#[component]
fn App() -> impl IntoView {
    view! {
        <Router>
            <header class="site-header">
                <h1 class="site-title">"ljh-trip"</h1>
                <p class="site-tagline">"travel, logged, 네이버TV 마이그레이션(260809)"</p>
            </header>
            <main>
                <Routes>
                    <Route path="/" view=HomePage/>
                    <Route path="/video/:id" view=VideoPage/>
                </Routes>
            </main>
        </Router>
    }
}

#[component]
fn HomePage() -> impl IntoView {
    let videos = create_local_resource(|| (), |_| async move { fetch_videos().await });

    view! {
        <Suspense fallback=|| view! { <p class="loading">"불러오는 중..."</p> }>
            {move || videos.get().map(|list| view! {
                <div class="grid">
                    {list.into_iter().map(|v| {
                        let href = format!("/video/{}", v.id);
                        view! {
                            <a href=href>
                                <div class="card">
                                    <img class="card-thumb" src=v.thumb.clone() />
                                    <div class="card-body">
                                        <h3 class="card-title">{v.title}</h3>
                                        <div class="card-meta">
                                            <span class="stamp">{v.date}</span>
                                        </div>
                                    </div>
                                </div>
                            </a>
                        }
                    }).collect_view()}
                </div>
            })}
        </Suspense>
    }
}

#[component]
fn VideoPage() -> impl IntoView {
    let params = use_params_map();
    let id = move || params.with(|p| p.get("id").cloned().unwrap_or_default());
    let videos = create_local_resource(|| (), |_| async move { fetch_videos().await });

    let copied = create_rw_signal(false);

    let share = move |_| {
        if let Some(window) = web_sys::window() {
            let url = window.location().href().unwrap_or_default();
            let clipboard = window.navigator().clipboard();
            spawn_local(async move {
                let _ = wasm_bindgen_futures::JsFuture::from(clipboard.write_text(&url)).await;
            });
        }
        copied.set(true);
        spawn_local(async move {
            gloo_timers::future::TimeoutFuture::new(1500).await;
            copied.set(false);
        });
    };

    view! {
        <div class="detail">
            <a class="back-link" href="/">"\u{2190} back"</a>
            <Suspense fallback=|| view! { <p class="loading">"불러오는 중..."</p> }>
                {move || videos.get().map(|list| {
                    match list.into_iter().find(|v| v.id == id()) {
                        Some(v) => view! {
                            <div>
                                <video
                                    src=v.src
                                    controls
                                    controlslist="nodownload"
                                    disablepictureinpicture
                                    on:contextmenu=|ev| ev.prevent_default()
                                ></video>
                                <h2 class="detail-title">{v.title}</h2>
                                <button class="share-btn" on:click=share>
                                    {move || if copied.get() { "복사됨" } else { "링크 공유" }}
                                </button>
                                <div class="detail-meta">{v.date}</div>
                                <p class="detail-desc">{v.description}</p>
                            </div>
                        }.into_view(),
                        None => view! { <p>"video not found"</p> }.into_view()
                    }
                })}
            </Suspense>
        </div>
    }
}

fn main() {
    leptos::mount_to_body(App)
}