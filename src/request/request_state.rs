#[derive(PartialEq)]
pub(super) enum RequestState {
    StateInit = 0,
    StateRequestLine = 1,
    StateHeaders = 2,
    StateBody = 3,
    StateDone = 4,
}
