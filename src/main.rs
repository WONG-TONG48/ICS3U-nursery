/*
By: Tristan St-Gelais
Date: 2026-02-19
Program Details: <Program Description Here>
*/

mod modules;

// use crate::modules::grid::draw_grid;
use crate::modules::label::Label;
use crate::modules::preload_image::TextureManager;
use crate::modules::still_image::StillImage;
use crate::modules::text_button::TextButton;
use macroquad::prelude::*;

/// Set up window settings before the app runs
fn window_conf() -> Conf {
    Conf {
        window_title: "nursery rhymes".to_string(),
        window_width: 1400,
        window_height: 700,
        fullscreen: false,
        high_dpi: true,
        window_resizable: true,
        sample_count: 4, // MSAA: makes shapes look smoother
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut lbl_out = Label::new("Click a button below\nto view a nursery rhyme!", 100.0, 50.0, 45);
    let mut img_out = StillImage::new("", 400.0, 350.0, 700.0, 50.0, true, 1.0).await;
    let mut btn_black_sheep = TextButton::new(100.0, 450.0, 200.0, 50.0, "Black Sheep", BLACK, GRAY, 40);
    let mut btn_mary_lamb = TextButton::new(400.0, 450.0, 200.0, 50.0, "Mary's Lamb", BLACK, GRAY, 40);
    let mut btn_little_star = TextButton::new(700.0, 450.0, 200.0, 50.0, "Little Star", BLACK, GRAY, 40);
    let btn_exit = TextButton::new(1000.0, 550.0, 200.0, 50.0, "Exit", BLACK, GRAY, 40);

    let texture_manager = TextureManager::new();
    texture_manager
        .preload_with_loading_screen(&["assets/black-sheep.png", "assets/mary-lamb.png", "assets/little-star.png", "assets/arrow.png"], None)
        .await;
    img_out.set_preload(texture_manager.get_preload("assets/arrow.png").unwrap());

    loop {
        clear_background(WHITE);
        //draw_grid(50.0, GRAY);

        if btn_exit.click() {
            break;
        }
        if btn_black_sheep.click() {
            lbl_out.set_text(
                "Baa, baa, black sheep,
                \nHave you any wool?
                \nYes sir, yes sir,
                \nThree bags full.",
            );
            img_out.set_preload(texture_manager.get_preload("assets/black-sheep.png").unwrap());
            btn_black_sheep.enabled = false;
            btn_little_star.enabled = true;
            btn_mary_lamb.enabled = true;
        }
        if btn_mary_lamb.click() {
            lbl_out.set_text(
                "Mary had a little lamb,
                \nlittle lamb, little lamb. 
                \nMary had a little lamb, 
                \nits fleece was white as snow.",
            );
            img_out.set_preload(texture_manager.get_preload("assets/mary-lamb.png").unwrap());
            btn_black_sheep.enabled = true;
            btn_little_star.enabled = true;
            btn_mary_lamb.enabled = false;
        }
        if btn_little_star.click() {
            lbl_out.set_text(
                "Twinkle, twinkle, little star,
                \nHow I wonder what you are!
                \nUp above the world so high,
                \nLike a diamond in the sky.",
            );
            img_out.set_preload(texture_manager.get_preload("assets/little-star.png").unwrap());
            btn_black_sheep.enabled = true;
            btn_little_star.enabled = false;
            btn_mary_lamb.enabled = true;
        }
        lbl_out.draw();
        img_out.draw();
        next_frame().await;
    }
}
