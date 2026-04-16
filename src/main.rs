use futures_util::StreamExt;
use reqwest::{Client, header};
use std::collections::HashMap;
use std::path::PathBuf;
use tl::parse;
use tokio::fs;
use tokio::fs::File;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use xilem::masonry::properties::types::{AsUnit, UnitPoint};
use xilem::style::{Padding, Style};
use xilem::view::{
    AnyFlexChild, ChildAlignment, CrossAxisAlignment, FlexExt, flex_col, flex_row, label,
    sized_box, slider, text_button, text_input, zstack, zstack_item,
};
use xilem::{AppState, Color, EventLoop, WidgetView, WindowId, WindowView, Xilem, window};

struct Context {
    running: bool,
    id: WindowId,
    next_sub_window: String,
    sub_windows: HashMap<WindowId, SubContext>,
    client: Client,
    next_url: String,
    opened_settings: bool,
    alpha: f64,
}

struct SubContext {
    name: String,
}

impl AppState for Context {
    fn keep_running(&self) -> bool {
        self.running
    }
}

async fn parse_html()->Result<(), Box<dyn std::error::Error>>{
    let content = {
        let file = File::open("cache/sites/current_site.html").await;
        let mut content = String::new();
        file?
            .read_to_string(&mut content).await.expect("Couldn't read file!");
        content
    };
    let dom = parse(&content, tl::ParserOptions::default())?;
    let parser = dom.parser();
    for e in dom.query_selector("p").unwrap(){
        let txt = e.get(parser).unwrap().inner_html(parser);
        println!("{}", txt);
    }
    Ok(())
}

async fn cache_html(client: Client, mut url: &str) -> Result<(), Box<dyn std::error::Error>> {
    url = url.trim();
    let res = client.get(url).send().await?;
    let mut path = PathBuf::from("cache/sites");
    fs::create_dir_all(&path)
        .await
        .expect("Couldn't create cache directory.");
    path.push("current_site.html");
    let mut file = File::create(path).await?;
    let mut stream = res.bytes_stream();
    while let Some(chunk_res) = stream.next().await {
        let chunk = chunk_res?;
        file.write_all(&chunk)
            .await
            .expect("Couldn't write to HTML cache.");
    }
    Ok(())
}

fn settings_view(cx: &mut Context) -> AnyFlexChild<Context> {
    if cx.opened_settings {
        return sized_box(
            flex_col((
                label("Window Alpha"),
                slider(0., 1., cx.alpha, |cx: &mut Context, input| cx.alpha = input).step(0.1),
            ))
            .cross_axis_alignment(CrossAxisAlignment::Start)
            .padding(Padding::all(5.)),
        )
        .width(550.px())
        .height(350.px())
        .corner_radius(5.)
        .border(Color::WHITE, 2.)
        .into_any_flex();
    }
    label("").into_any_flex() // there's probably a better way to get an empty element
}

fn title_bar() -> impl WidgetView<Context> + use<> {
    flex_row(text_button("Config", |cx: &mut Context| {
        cx.opened_settings = !cx.opened_settings
    }))
}

fn logic(cx: &mut Context) -> impl Iterator<Item = WindowView<Context>> + use<> {
    let base_color = Color::new([0., 0., 0., cx.alpha as f32]);
    let main_view = flex_col((
        title_bar(),
        text_input(cx.next_sub_window.clone(), |cx: &mut Context, input| {
            cx.next_sub_window = input
        })
        .on_enter(|cx: &mut Context, _| {
            if !cx
                .sub_windows
                .values()
                .any(|sub_window| sub_window.name == cx.next_sub_window)
            {
                let name = std::mem::take(&mut cx.next_sub_window);
                cx.sub_windows.insert(WindowId::next(), SubContext { name });
            }
        })
        .placeholder("Subwindow Name"),
        text_input(cx.next_url.clone(), |cx: &mut Context, input| {
            cx.next_url = input
        })
        .on_enter(|cx: &mut Context, input| {
            let client = cx.client.clone();
            tokio::spawn(async move {
                cache_html(client, input.as_str()).await.expect("Couldn't cache HTML!");
            });
        })
        .placeholder("Search or enter an address"),
        text_button("Refresh", |cx:&mut Context| {
            tokio::spawn(async move {
                parse_html().await.expect("Couldn't parse HTML!");
            });
        })
    ));
    let root = zstack((
        zstack_item(main_view, ChildAlignment::SelfAligned(UnitPoint::TOP)),
        zstack_item(
            flex_col(settings_view(cx)),
            ChildAlignment::SelfAligned(UnitPoint::CENTER),
        ),
    ));
    std::iter::once(
        window(cx.id, "Kaleidoscope", root)
            .with_base_color(base_color)
            .with_options(|o| {
                o.on_close(|cx: &mut Context| close(cx))
                    .with_transparent(true)
            }),
    )
    .chain(cx.sub_windows.iter().map(|(id, SubContext { name })| {
        let id = *id;
        window(
            id,
            format!("Kaleidoscope Subwindow - {}", name),
            flex_col((
                label("other window"),
                label("wow"),
                text_button("button that does nothing", |cx: &mut Context| {}),
            )),
        )
        .with_base_color(base_color)
        .with_options(|o| {
            o.on_close(move |cx: &mut Context| {
                cx.sub_windows.remove(&id);
            })
            .with_transparent(true)
        })
    }))
    .collect::<Vec<_>>()
    .into_iter()
}

fn close(cx: &mut Context) {
    cx.running = false;
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    File::create("config.toml").await?;
    let client = {
        let mut headers = header::HeaderMap::new();
        headers.insert(
            header::USER_AGENT,
            header::HeaderValue::from_static("Kaleidoscope"),
        );
        Client::builder()
            .default_headers(headers)
            .tls_backend_rustls()
            .build()?
    };
    let cx = Context {
        running: true,
        id: WindowId::next(),
        next_sub_window: String::new(),
        sub_windows: HashMap::new(),
        client,
        next_url: String::new(),
        opened_settings: false,
        alpha: 0.5,
    };
    let app = Xilem::new(cx, logic);
    app.run_in(EventLoop::with_user_event())?;
    Ok(())
}