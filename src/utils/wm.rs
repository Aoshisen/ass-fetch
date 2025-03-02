pub fn get() -> String {
    let wms = vec![
        String::from("Quartz Compositor"),
        String::from("yabai"),
        String::from("Rectangle"),
        String::from("Spectacle"),
        String::from("Amethyst"),
        String::from("Kwm"),
    ];
    //使用 ps -e | rg -o -e {wm} 来确定到底是哪个dw 然后返回
}
