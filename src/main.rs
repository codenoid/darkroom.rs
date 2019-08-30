use actix_web::HttpRequest;
use actix_web::HttpResponse;
use actix_web::{web, App, HttpServer};
use hashbrown::HashMap;
use raster::{filter, transform};
use raster::{BlurMode, Color, TransformMode};
use std::path::Path;

fn index(req: HttpRequest) -> HttpResponse {
    if req.query_string() != "" {
        let mut edit: hashbrown::HashMap<String, String> = HashMap::new();

        let url_data: Vec<String> = req
            .query_string()
            .split("&")
            .map(|s| s.to_string())
            .collect();

        for kv_line in url_data {
            let kv: Vec<String> = kv_line.split("=").map(|s| s.to_string()).collect();
            if kv.len() == 2 {
                edit.insert(kv[0].clone(), kv[1].clone());
            }
        }

        if edit.get("path") != None {
            let path = edit.get("path").unwrap();

            if Path::new(path).exists() {
                let mut image = raster::open(path).unwrap();
                for (key, value) in &edit {
                    if key != "path" {
                        match key.as_ref() {
                            "flip" => {
                                let mut flip = TransformMode::Vertical;
                                if value == "h" {
                                    flip = TransformMode::Horizontal;
                                }
                                transform::flip(&mut image, flip).unwrap();
                            }
                            "rotate" => {
                                if value.chars().all(char::is_numeric) {
                                    let rotate = value.parse::<i32>();
                                    if !rotate.is_err() {
                                        transform::rotate(
                                            &mut image,
                                            rotate.unwrap(),
                                            Color::rgb(0, 0, 0),
                                        )
                                        .unwrap();
                                    }
                                }
                            }
                            "mono" => {
                                filter::grayscale(&mut image).unwrap();
                            }
                            "blur" => {
                                let mut blur = BlurMode::Box;
                                if value == "gaussian" {
                                    blur = BlurMode::Gaussian;
                                }
                                filter::blur(&mut image, blur).unwrap();
                            }
                            "brightness" => {
                                let factor = value.parse::<f32>();
                                if !factor.is_err() {
                                    filter::brightness(&mut image, factor.unwrap()).unwrap();
                                }
                            }
                            "emboss" => {
                                filter::emboss(&mut image).unwrap();
                            }
                            "gamma" => {
                                let gamma = value.parse::<f32>();
                                if !gamma.is_err() {
                                    filter::gamma(&mut image, gamma.unwrap()).unwrap();
                                }
                            }
                            "saturation" => {
                                let sat = value.parse::<f32>();
                                if !sat.is_err() {
                                    filter::saturation(&mut image, sat.unwrap()).unwrap();
                                }
                            }
                            _ => {
                                println!("{:?}", "hehe");
                            }
                        }
                    }
                }
                // &image.dave() type check
                raster::save(&image, "./out.jpg").unwrap();
            }
        }
    }

    HttpResponse::Ok().content_type("text/plain").body("ok")
}

fn main() {
    HttpServer::new(|| App::new().route("/", web::get().to(index)))
        .bind("127.0.0.1:3000")
        .unwrap()
        .run()
        .unwrap();
}
