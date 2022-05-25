use launchy::Pad;

const GRID_SIZE: i32 = 8;

pub enum ButtonType { Grid, Top, Side }

pub struct Button {
    pub btn_type: ButtonType,
    pub id: u8,
    pad: Pad
}

impl Button {
    pub fn from(pad: Pad) -> Self {
        let btype: ButtonType;
        let id: i32;

        if pad.y == 0 {
            btype = ButtonType::Top;
            id = pad.x + 1;
        } else if pad.x == 8 {
            btype = ButtonType::Side;
            id = pad.y;
        } else {
            btype = ButtonType::Grid;
            id = (pad.y - 1) * GRID_SIZE + pad.x + 1;
        }

        return Self { btn_type: btype, id: id as u8, pad: pad };
    }

    pub fn calc_pad(btn_type: ButtonType, id: u8) -> Pad {
        return match btn_type {
            ButtonType::Top => Pad { x: (id - 1) as i32, y: 0 },
            ButtonType::Side => Pad { x: 8, y: id as i32 },
            ButtonType::Grid => {
                let x: i32 = (id as i32) % GRID_SIZE - 1;
                let y: i32 = (id as i32) / GRID_SIZE + 1;
                return Pad { x: x, y: y };
            }
        };
    }

    pub fn get_pad(&self) -> Pad {
        return self.pad;
    }
}