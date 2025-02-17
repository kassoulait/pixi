use console::Style;
use lazy_static::lazy_static;

pub const PROJECT_MANIFEST: &str = "pixi.toml";
pub const PROJECT_LOCK_FILE: &str = "pixi.lock";
pub const PIXI_DIR: &str = ".pixi";
pub const PREFIX_FILE_NAME: &str = "prefix";
pub const ENVIRONMENTS_DIR: &str = "envs";
pub const SOLVE_GROUP_ENVIRONMENTS_DIR: &str = "solve-group-envs";
pub const PYPI_DEPENDENCIES: &str = "pypi-dependencies";

pub const DEFAULT_ENVIRONMENT_NAME: &str = "default";

pub const DEFAULT_FEATURE_NAME: &str = DEFAULT_ENVIRONMENT_NAME;

lazy_static! {
    pub static ref TASK_STYLE: Style = Style::new().blue();
    pub static ref PLATFORM_STYLE: Style = Style::new().yellow();
    pub static ref SOLVE_GROUP_STYLE: Style = Style::new().cyan();
}
