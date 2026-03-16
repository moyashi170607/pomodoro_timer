pub enum PomodoroPhase {
    Work,
    ShortBreak,
    LongBreak,
    Stop,
}

pub struct PomodoroState {
    pub phase: PomodoroPhase,
    pub counter: u16,
}
