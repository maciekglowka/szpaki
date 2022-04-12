use clap::Parser;

#[derive (Parser)]
pub struct Config {
    #[clap(short, long, default_value_t = 800)]
    pub size: usize,

    #[clap(short, long, default_value_t = 1500)]
    pub count: usize,

    #[clap(long, default_value_t = 1.0)]
    pub alignment: f32,

    #[clap(long, default_value_t = 0.2)]
    pub cohesion: f32,

    #[clap(long, default_value_t = 0.1)]
    pub roosting: f32,

    #[clap(long, default_value_t = 15.0)]
    pub separation: f32,

    #[clap(long, default_value_t = 50.0)]
    pub neighbourhood: f32,

    #[clap(long, default_value_t = 5.0)]
    pub separation_range: f32,

    #[clap(long, default_value_t = 4.0)]
    pub velocity: f32,

    #[clap(long, default_value_t = 0.2)]
    pub wind: f32
}