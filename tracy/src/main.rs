use clap::{ArgAction, Parser};
use std::time;
use tracy_export::export;
use tracy_import::sample_scene;
use tracy_render::Renderer;
use tracy_render_view::RenderView;

// TODO: Implement a standalone type for normalized values.

fn main() {
    let start = time::Instant::now();
    let args = Args::parse();

    if args.render_view {
        let mut render_view = RenderView::new(sample_scene());
        render_view.run();
    } else {
        run();
    }

    println!("Total: {:?}", start.elapsed());
}

#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    #[arg(long, action = ArgAction::SetTrue)]
    render_view: bool,
}

fn run() {
    let scene = sample_scene();
    let renderer = Renderer { spp: 50, depth: 50 };

    let start_render = time::Instant::now();
    let buf = renderer.render(&scene);
    println!("Render: {:?}", start_render.elapsed());

    let start_export = time::Instant::now();
    export(buf).expect("export failed");
    println!("Export: {:?}", start_export.elapsed());
}
