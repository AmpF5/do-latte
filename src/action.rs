pub enum Action {
    Render,
    Resize,
    Suspend,
    Resume,
    Quit,
    Error(String),
    Help,
}
