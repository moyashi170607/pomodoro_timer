use embassy_time::{Duration, Timer};
use embedded_hal::digital::{InputPin, OutputPin};

use crate::state::{PomodoroPhase, PomodoroState};

const WORK_DURATION_SECS: u32 = 25 * 60;
const SHORT_BREAK_DURATION_SECS: u32 = 5 * 60;
const LONG_BREAK_DURATION_SECS: u32 = 15 * 60;

// const WORK_DURATION_SECS: u32 = 5;
// const SHORT_BREAK_DURATION_SECS: u32 = 3;
// const LONG_BREAK_DURATION_SECS: u32 = 4;

const WORK_COUNT_FOR_LONG_BREAK: u16 = 4;

const WORK_END_FREQ: u32 = 500;
const SHORT_BREAK_END_FREQ: u32 = 800;
const LONG_BREAK_END_FREQ: u32 = 1000;

pub trait Buzzer {
    fn beep(&mut self, freq_hz: u32);
    fn silence(&mut self);
}

pub struct Pomodoro<WLED, SLED, LLED, BTN1, BTN2, BUZ, VIB>
where
    WLED: OutputPin,
    SLED: OutputPin,
    LLED: OutputPin,
    BTN1: InputPin,
    BTN2: InputPin,
    BUZ: Buzzer,
    VIB: OutputPin,
{
    state: PomodoroState,
    work_led: WLED,
    short_break_led: SLED,
    long_break_led: LLED,
    button1: BTN1,
    button2: BTN2,
    buzzer: BUZ,
    vibration: VIB,
}

impl<WLED, SLED, LLED, BTN1, BTN2, BUZ, VIB> Pomodoro<WLED, SLED, LLED, BTN1, BTN2, BUZ, VIB>
where
    WLED: OutputPin,
    SLED: OutputPin,
    LLED: OutputPin,
    BTN1: InputPin,
    BTN2: InputPin,
    BUZ: Buzzer,
    VIB: OutputPin,
{
    pub fn new(
        work_led: WLED,
        short_break_led: SLED,
        long_break_led: LLED,
        button1: BTN1,
        button2: BTN2,
        mut buzzer: BUZ,
        mut vibration: VIB,
    ) -> Self {
        buzzer.silence();
        vibration.set_low().ok();
        Self {
            state: PomodoroState {
                phase: PomodoroPhase::Stop,
                counter: 0,
            },
            work_led,
            short_break_led,
            long_break_led,
            button1,
            button2,
            buzzer,
            vibration,
        }
    }

    //ボタンの状態を返す
    pub fn check_button(&mut self) -> (bool, bool) {
        let b1 = self.button1.is_high().unwrap_or(false);
        let b2 = self.button2.is_high().unwrap_or(false);
        (b1, b2)
    }

    // LEDの状態を設定する
    pub fn set_leds(&mut self, w: bool, s: bool, l: bool) {
        if w {
            self.work_led.set_high().ok();
        } else {
            self.work_led.set_low().ok();
        }
        if s {
            self.short_break_led.set_high().ok();
        } else {
            self.short_break_led.set_low().ok();
        }
        if l {
            self.long_break_led.set_high().ok();
        } else {
            self.long_break_led.set_low().ok();
        }
    }

    // ボタン1が押されるのを待つ
    pub async fn wait_button1(&mut self) {
        while self.button1.is_high().unwrap_or(true) {
            Timer::after(Duration::from_millis(10)).await;
        }
        while self.button1.is_low().unwrap_or(true) {
            Timer::after(Duration::from_millis(10)).await;
        }
    }

    async fn work(&mut self) {
        self.state.phase = PomodoroPhase::Work;
        self.set_leds(true, false, false);
        self.timer_start(WORK_DURATION_SECS).await;
    }

    async fn short_break(&mut self) {
        self.state.phase = PomodoroPhase::ShortBreak;
        self.set_leds(false, true, false);
        self.timer_start(SHORT_BREAK_DURATION_SECS).await;
    }

    async fn long_break(&mut self) {
        self.state.phase = PomodoroPhase::LongBreak;
        self.set_leds(false, false, true);
        self.timer_start(LONG_BREAK_DURATION_SECS).await;
    }

    //作業開始
    pub async fn start(&mut self) {
        loop {
            self.state.counter = 0;
            self.set_leds(false, false, false);

            loop {
                self.work().await;
                self.state.counter += 1;
                if self.state.counter >= WORK_COUNT_FOR_LONG_BREAK {
                    self.set_leds(true, false, true);
                } else {
                    self.set_leds(true, true, false);
                }
                self.buzzer.beep(WORK_END_FREQ);
                self.vibration.set_high().ok();
                self.wait_button1().await;
                self.buzzer.silence();
                self.vibration.set_low().ok();

                if self.state.counter >= WORK_COUNT_FOR_LONG_BREAK {
                    break;
                }

                self.short_break().await;
                self.set_leds(true, true, false);
                self.buzzer.beep(SHORT_BREAK_END_FREQ);
                self.vibration.set_high().ok();
                self.wait_button1().await;
                self.buzzer.silence();
                self.vibration.set_low().ok();
            }
            self.long_break().await;
            self.set_leds(true, false, true);
            self.buzzer.beep(LONG_BREAK_END_FREQ);
            self.vibration.set_high().ok();
            self.wait_button1().await;
            self.buzzer.silence();
            self.vibration.set_low().ok();
        }
    }

    // タイマー開始
    pub async fn timer_start(&mut self, length_secs: u32) {
        Timer::after(Duration::from_secs(length_secs as u64)).await;
    }
}
