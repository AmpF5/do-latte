#[derive(Debug)]
pub enum Action {
    None,
    Render,
    RenderToDoPopup,
    Resize,
    Suspend,
    Resume,
    Quit,
    Error(String),
    Help,
}
