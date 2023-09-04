use qrcode::render::svg;
use qrcode::{EcLevel, QrCode, Version};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn qrcode(data: &str, ver: Option<i8>, ec: Option<char>) -> Result<String, JsError> {
    let ec = match ec {
        Some('L') => EcLevel::L,
        Some('M') | None => EcLevel::M,
        Some('Q') => EcLevel::Q,
        Some('H') => EcLevel::H,
        _ => return Err(JsError::new("invalid error correction"))
    };

    let qr = if let Some(ver) = ver {
        QrCode::with_version(&data,
            if ver < 0 {
                Version::Micro(ver.abs() as i16)
            } else {
                Version::Normal(ver as i16)
            }, ec)?
    }else {
        QrCode::with_error_correction_level(&data, ec)?
    };

    Ok(qr.render::<svg::Color>().quiet_zone(false).build())
}
