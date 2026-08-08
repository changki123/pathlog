use leptos::*;
use leptos_router::*;

#[derive(Clone)]
struct Video {
    id: &'static str,
    title: &'static str,
    location: &'static str,
    date: &'static str,
    src: &'static str,
}

// TODO: 나중에 Lambda API에서 목록을 받아오도록 교체
fn videos() -> Vec<Video> {
    vec![Video {
        id: "trip6",
        title: "trip6",
        location: "unknown",
        date: "2026",
        src: "https://pub-de6b3d11021b47d7a23fcdd4a9de93c2.r2.dev/videos/trip6.mp4",
    }]
}

#[component]
fn App() -> impl IntoView {
    view! {
        <Router>
            <header class="site-header">
                <h1 class="site-title">"pathlog"</h1>
                <p class="site-tagline">"travel, logged"</p>
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
    view! {
        <div class="grid">
            {videos().into_iter().map(|v| {
                let href = format!("/video/{}", v.id);
                view! {
                    <a href=href>
                        <div class="card">
                            <div class="card-thumb"></div>
                            <div class="card-body">
                                <h3 class="card-title">{v.title}</h3>
                                <div class="card-meta">
                                    <span>{v.location}</span>
                                    <span class="stamp">{v.date}</span>
                                </div>
                            </div>
                        </div>
                    </a>
                }
            }).collect_view()}
        </div>
    }
}

#[component]
fn VideoPage() -> impl IntoView {
    let params = use_params_map();
    let id = move || params.with(|p| p.get("id").cloned().unwrap_or_default());
    let video = move || videos().into_iter().find(|v| v.id == id());

    view! {
        <div class="detail">
            <a class="back-link" href="/">"\u2190 back"</a>
            {move || match video() {
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
                        <div class="detail-meta">{v.location} " \u00b7 " {v.date}</div>
                    </div>
                }.into_view(),
                None => view! { <p>"video not found"</p> }.into_view()
            }}
        </div>
    }
}

fn main() {
    leptos::mount_to_body(App)
}
