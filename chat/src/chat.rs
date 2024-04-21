use gpui::*;
use gpui::Pixels;
use std::time::Duration;
use tungstenite::{connect, Message};
use url::Url;

pub static WIDTH: f64 = 800.0;
pub static HEIGHT: f64 = 450.0;

struct Chat {
    text: Option<SharedString>,
    _subscription: Option<Subscription>,
}

struct NewMessage {
    message: String,
}

impl EventEmitter<NewMessage> for Chat {}

impl Chat {
    pub fn build_view(model: &Model<Chat>, cx: &mut WindowContext) -> View<Self> {
        let view = cx.new_view(|cx| {
            let subscription = cx.subscribe(model, |this: &mut Chat, _emitter, event, cx| {
                println!("New message: {}", event.message);
                this.text = Some(event.message.clone().into());
                cx.notify();
            });
            Self {
                text: None,
                _subscription: Some(subscription),
            }
        });
        view
    }

    pub fn build_model(cx: &mut WindowContext) -> Model<Chat> {
        let counter: Model<Chat> = cx.new_model(|_cx| Chat {
            text: None,
            _subscription: None,
        });
        counter
    }

    pub fn options(bounds: Bounds<DevicePixels>) -> WindowOptions {
        let mut options = WindowOptions::default();
        let center = bounds.center();

        
        options.focus = true;
        let width = DevicePixels::from(800);
        let height = DevicePixels::from(450);
        let x: DevicePixels = center.x - width / 2;
        let y: DevicePixels = center.y - height / 2;
        let point = Point::<DevicePixels> { x, y}; 
        options.bounds = Some(Bounds::new(point, Size { width, height }));
        options.titlebar = None;
        options.is_movable = true;
        options.kind = WindowKind::PopUp;
        options
    }
    

}

impl Render for Chat {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        let text = self.text.get_or_insert_with(|| "".into()).clone();
        div()
            .flex()
            .bg(rgb(0x333333))
            .size_full()
            .justify_center()
            .items_center()
            .text_xl()
            .text_color(rgb(0xffffff))
            .child(format!("Message, {}!", text))
    }
}

pub fn run_app(app: gpui::App) {
    app.run(|cx: &mut AppContext| {
        let bounds = cx.displays().first().expect("No Display FOund").bounds();
        cx.open_window(Chat::options(bounds), |cx| {
            //cx.new_view(|_cx| Chat { text: None })
            let model = Chat::build_model(cx);
            let view = Chat::build_view(&model, cx);
            cx.spawn(|mut cx| async move {
                let mut count = 0;
                let (mut socket, response) = connect(Url::parse("ws://localhost:3030/chat").unwrap())
                .expect("Can't connect");
                println!("Connected to the Server!!!");
                println!("Response Http Code: {}", response.status());
                loop {
                    //count = count + 1;
                    
                    let msg = socket.read().expect("Error reading Message");
                    println!("Received: {}", msg);

                    match msg {
                        Message::Text(text) => {
                            let _ = model.update(&mut cx, |_chat, cx| {
                                cx.emit(NewMessage { message: text.clone(), });
                            });
                        }
                        Message::Close(_) => {
                            println!("Connection closed!!!");
                            break;
                        }
                        Message::Ping(_) => {
                            println!("ping");
                        }
                        Message::Pong(_) => {
                            println!("pong");
                        }
                        Message::Binary(_) => {
                            println!("Binary");
                        }
                        Message::Frame(_) => {
                            println!("Frame");
                        }
                        _ => {}
                    }
                    cx.background_executor()
                        .timer(Duration::from_millis(100))
                        .await;
                }
            })
            .detach();
            view
        });
    });
}
