use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum Level
{
	Two = 2;
	Three = 3;
}
impl Default for Level
{
	fn default() -> Self
	{
		Level::Two
	}
}

//
// YamlMasconData
//
#[derive(Serialize, Deserialize)]
pub struct YamlMasconDataPatternMode
{
	pub frequency_change_rate: f64,
	pub max_control_frequency: f64,
}
impl Default for YamlMasconDataPatternMode
{
	fn default() -> Self
	{
		Self
		{
			frequency_change_rate: 60.0,
			max_control_frequency: 60.0,
		}
	}
}

#[derive(Default, Serialize, Deserialize)]
pub struct YamlMasconDataPattern
{
	pub on:  YamlMasconDataPatternMode,
	pub off: YamlMasconDataPatternMode,
}

#[derive(Default, Serialize, Deserialize)]
pub struct YamlMasconData
{
	pub braking:      YamlMasconDataPattern,
	pub accelerating: YamlMasconDataPattern,
}

//
// YamlMinSineFrequency
//
#[derive(Default, Serialize, Deserialize)]
pub struct YamlMinSineFrequency
{
	pub accelerating: f64,
	pub braking:      f64,
}

//
// YamlControlData
//
#[derive(Clone, Serialize, Deserialize)]
struct YamlControlData
{
	pub control_frequency_from: f64,
	pub rotate_frequency_from:  f64,
	pub rotate_frequency_below: f64,
	pub enable_free_run_on:     bool,
	pub stuck_free_run_on:      bool,
	pub enable_free_run_off:    bool,
	pub stuck_free_run_off:     bool,
	pub enable_normal:          bool,

	pub pulse_mode:             YamlPulseMode,
	pub amplitude:              YamlAmplitude,
	pub async_modulation_data:      YamlAsync,
}
impl Default for YamlControlData
{
	fn default() -> Self
	{
		Self
		{
			control_frequency_from: -1.0,
			rotate_frequency_from:  -1.0,
			rotate_frequency_below: -1.0,
			enable_free_run_on:      true,
			stuck_free_run_on:       false,
			enable_free_run_off:     true,
			stuck_free_run_off:      false,
			enable_normal:           true,

			pulse_mode:              default,
			amplitude:               default,
			async_modulation_data:   default,
		}
	}
}

//
// YamlMovingValue
//
#[derive(Clone, Serialize, Deserialize)]
pub struct YamlMovingValue
{
	pub mv_type:     MovingValueType,
	pub start:       f64,
	pub start_value: f64,
	pub end:         f64,
	pub end_value:   f64,
	pub degree:      f64,
	pub curve_rate:  f64,
}
impl Default for YamlMovingValue;

#[derive(Clone, Serialize, Deserialize)]
pub enum MovingValueType
{
	Proportional, Inv_Proportional, Pow2_Exponential, Sine
}
impl Default for MovingValueType
{
	fn default() -> Self
	{
		Self
		{
			MovingValueType::Proportional
		}
	}
}

//
// YamlPulseMode
//
#[derive(Clone, Serialize, Deserialize)]
pub struct YamlPulseMode
{
	//
	// Fundamental Configuration
	//
	pub pulse_type:    PulseTypeName,
	pub pulse_count:   f64,
	
	//
	// Alternative Modes
	//
	pub alternative:   PulseAlternative

	//
	// Discrete Time Configuration
	pub discrete_time: DiscreteTimeConfiguration
}

//
// YamlVvvfSoundData
//
#[derive(Default, Serialize, Deserialize)]
pub struct YamlVvvfSoundData
{
	pub level: Level,
	pub mascon_data: YamlMasconData,
	pub minimum_frequency: YamlMinSineFrequency,
	pub accelerate_pattern: std::Vec<YamlControlData>,
	pub braking_pattern: std::Vec<YamlControlData>,
}