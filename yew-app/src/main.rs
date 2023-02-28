use yew::prelude::*;

#[derive(Clone, PartialEq)]
struct Video {
    id: usize,
    title: String,
    speaker: String,
    url: String,
}


#[derive(Properties, PartialEq)]
struct VideoListProps {
    videos: Vec<Video>,
}

#[function_component(VideoList)]
fn videos_list(VideoListProps { videos }: &VideoListProps) -> Html {
    videos.iter().map(|video| html! {
         <p key={video.id}>{format!("{}: {}", video.speaker, video.title)}</p>
    }).collect()
}

#[function_component(App)]
fn app() -> Html {

    let videos = vec![
        Video {
            id: 1,
            title: "launch".to_string(),
            speaker: "nagle".to_string(),
            url: "https://www.bilibili.com/video/BV1Qs4y1j7jV/?spm_id_from=333.1007.tianma.2-1-4.click".to_string()
        },
        Video {
            id: 2,
            title: "launch2".to_string(),
            speaker: "nagle".to_string(),
            url: "https://www.bilibili.com/video/BV1kU4y1p7Jb/?spm_id_from=333.788.recommend_more_video.0".to_string()
        },
    ];


    html! {
      <>
        <h1>{ "RustConf Explorer" }</h1>
        <div>
            <h3>{"Vides to watch"}</h3>
            <VideoList videos={videos} />
        </div>
        <div>
            <h3>{ "John Doe: Building and breaking things" }</h3>
            <img src="https://via.placeholder.com/640x360.png?text=Video+Player+Placeholder" alt="video thumbnail" />
        </div>
      </>
    }
}
fn main() {
    yew::Renderer::<App>::new().render();
}
