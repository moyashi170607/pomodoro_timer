#![no_std]
#![no_main]

use crate::pomodoro::{Buzzer, Pomodoro};
use ch32_hal::gpio::Input;
use ch32_hal::peripherals::TIM1;
use ch32_hal::time::Hertz;
use ch32_hal::timer::Channel;
use ch32_hal::timer::low_level::CountingMode;
use ch32_hal::timer::simple_pwm::{PwmPin, SimplePwm};
use hal::gpio::{Level, Output};

use {ch32_hal as hal, panic_halt as _};

pub mod pomodoro;
pub mod state;

//ch32-hal用のブザー構造体
struct Ch32Buzzer<'a> {
    pwm: SimplePwm<'a, TIM1>,
    ch: Channel,
}

impl Buzzer for Ch32Buzzer<'_> {
    fn beep(&mut self, freq_hz: u32) {
        self.pwm.set_frequency(Hertz::hz(freq_hz));
        self.pwm.set_duty(self.ch, self.pwm.get_max_duty() / 2);
        self.pwm.enable(self.ch);
    }

    fn silence(&mut self) {
        self.pwm.disable(self.ch);
    }
}

#[embassy_executor::main(entry = "qingke_rt::entry")]
async fn main(_spawner: embassy_executor::Spawner) {
    hal::debug::SDIPrint::enable();
    let p = hal::init(hal::Config::default());

    // LED
    let work_led_pin = Output::new(p.PC6, Level::Low, Default::default());
    let short_break_led_pin = Output::new(p.PC2, Level::Low, Default::default());
    let long_break_led_pin = Output::new(p.PC5, Level::Low, Default::default());

    // パッシブブザー
    let buzzer_pin = PwmPin::new_ch3::<0>(p.PC3);

    //振動
    let vibration_pin = Output::new(p.PA2, Level::Low, Default::default());

    let pwm = SimplePwm::new(
        p.TIM1,
        None,
        None,
        Some(buzzer_pin),
        None,
        Hertz::hz(440),
        CountingMode::default(),
    );
    let ch = hal::timer::Channel::Ch3;
    let ch32_buzzer = Ch32Buzzer { pwm, ch };

    // ボタン
    let button1_pin = Input::new(p.PD0, ch32_hal::gpio::Pull::Up);
    let button2_pin = Input::new(p.PC7, ch32_hal::gpio::Pull::Up);

    //Pomodoroの初期化
    let mut pomodoro = Pomodoro::new(
        work_led_pin,
        short_break_led_pin,
        long_break_led_pin,
        button1_pin,
        button2_pin,
        ch32_buzzer,
        vibration_pin,
    );

    pomodoro.set_leds(false, false, false);

    //ボタンが押されるまで待つ
    pomodoro.wait_button1().await;
    pomodoro.start().await;
}
