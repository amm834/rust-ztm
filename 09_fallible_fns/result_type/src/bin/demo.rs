#[derive(Debug)]
enum State {
    Quit,
    Start,
    Pause,
}

fn get_state(state: State) -> Result<State, String> {
    match state {
        State::Quit => Err("Quit".to_string()),
        State::Start => Ok(State::Start),
        State::Pause => Ok(State::Pause),
    }
}

fn main() {
    let state = State::Quit;
    let result = get_state(state);
    match result {
        Ok(state) => println!("State: {:?}", state),
    }
}