mod cli;
mod template;
mod generator;
mod replacer;
mod ui;

use crate::template::{create_directory_templates};
use crate::ui::tui;
/*
ESTE PROYECTO SERA UN CLI LLAMADO AXER ES PARA CREAR CAREPTA O BUENO PROYECTOS A PARTIR DE
TEMPLATES, POR EL MOMENTO LO INTEGRAREMOS CON RUST Y JS (EXPRESS, NESTJS, NEXTJS)
SE LEERA APARTIR DE UN TOML, QUE INTEGRARA LAS DISTINTAS COSAS PARA QUE ESTE ALL LISTO
 */

fn main() {
    create_directory_templates();
    tui()
}
