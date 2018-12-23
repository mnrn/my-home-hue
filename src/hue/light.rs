use serde_derive::Deserialize;

/// Details the state of the light, see the state table below for more details.
#[derive(Debug, Deserialize)]
pub struct LightState {
    /// On/Off state of the light. On=true, Off=false.
    on: bool,
    /// Brightness of the light. The min the light is capable of, 1, to the max of, 254.
    bri: u8,
    /// Hue of the light. This is a wrapping value between 0 and 65535. 
    hue: u16,
    /// Saturation of the light. 254 is the most saturated  and 0 is the least saturated.
    sat: u8,
    /// The dynamic effect of the light, can either be “none” or “colorloop”.
    effect: String,
    /// The x and y coordinates of a color in CIE color space.
    xy: Vec<f32>,
    /// The Mired Color temperature of the light. 2012 connected lights are capable of 153 to 500.
    ct: u16,
    /// The alert effect, which is a temporary change to the bulb’s state. 
    alert: String,
    /// Indicates the color mode in which the light is working, this is the last command type it received. 
    colormode: String,
    /// Device can be controlled by updating /state, light output is reflected in /state.
    mode: String,
    /// Indicates if a light can be reached by the bridge.
    reachable: bool,
}

/// Structure for Hue Light.
#[derive(Debug)]
pub struct Light {
    /// Hue Light Id.
    id: i32,
    /// Details the state of the light.
    state: LightState,
}

impl Light {
    /// Return a light with a state givin them.
    pub fn new(id: i32, state: LightState) -> Light {
        Light {
            id: id,
            state: state,
        }
    }

    /// On/Off state of the light. On=true, Off=false.
    pub fn is_on(self) -> bool {
        self.state.on
    }
}