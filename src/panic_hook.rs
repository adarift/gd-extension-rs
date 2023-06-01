
pub fn init() -> Result<(), Box<dyn std::error::Error>> {
    std::panic::set_hook(Box::new(|panic_info| {
        let err = panic_info.payload().downcast_ref::<&str>();
        let location = panic_info.location();

        let mut error = "Unknown";
        let mut file = "Unknown";
        let mut line: String = "Unknown".to_string();
        let mut column: String = "Unknown".to_string();

        if err.is_some() {
            error = err.unwrap();
        }

        if location.is_some() {
            file = location.unwrap().file();
            line = location.unwrap().line().to_string();
            column = location.unwrap().column().to_string();
        }

        let error_message = format!
        ("Error: {}.\nMessage: {:?}.\nLocation: {}.\nLine: {}.\nColumn: {}.", 
        error,
        panic_info.message(),
        file, 
        line, 
        column);
        msgbox::create("Error has occured", &error_message, msgbox::IconType::Error).unwrap();
    }));
    Ok(())
}