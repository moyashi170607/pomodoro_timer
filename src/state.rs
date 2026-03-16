pub enum PomodoroPhase {
    Work,
    ShortBreak,
    LongBreak,
    Stop,
}

pub struct PomodoroState {
    //現在のタイマーの状態
    pub phase: PomodoroPhase,
    //何回ポモドーロを行ったか
    pub counter: u16,
}
