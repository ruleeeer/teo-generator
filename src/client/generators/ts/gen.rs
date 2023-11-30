use askama::Template;
use async_trait::async_trait;
use teo_runtime::config::client::Client;
use crate::client::ctx::Ctx;
use crate::client::generator::Generator;
use crate::client::generators::ts::package_json::{generate_package_json, update_package_json};
use crate::outline::outline::Outline;
use crate::utils::file::FileUtil;

#[derive(Template)]
#[template(path = "client/ts/readme.md.jinja", escape = "none")]
pub(self) struct TsReadMeTemplate<'a> {
    pub(self) conf: &'a Client,
}

#[derive(Template)]
#[template(path = "client/ts/index.js.jinja", escape = "none")]
pub(self) struct TsIndexJsTemplate<'a> {
    pub(self) outline: &'a Outline,
    pub(self) conf: &'a Client,
}

#[derive(Template)]
#[template(path = "client/ts/index.d.ts.jinja", escape = "none")]
pub(self) struct TsIndexDTsTemplate<'a> {
    pub(self) outline: &'a Outline,
    pub(self) conf: &'a Client,
}

pub(in crate::client) struct TSGenerator {}

impl TSGenerator {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl Generator for TSGenerator {

    fn module_directory_in_package(&self, conf: &Client) -> String {
        "src".to_owned()
    }

    async fn generate_module_files(&self, ctx: &Ctx, generator: &FileUtil) -> teo_result::Result<()> {
        generator.clear_root_directory().await
    }

    async fn generate_package_files(&self, ctx: &Ctx, generator: &FileUtil) -> teo_result::Result<()> {
        generator.ensure_root_directory().await?;
        generator.generate_file(".gitignore", include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/client/ts/gitignore"))).await?;
        generator.generate_file("README.md", TsReadMeTemplate { conf: ctx.conf }.render().unwrap()).await?;
        if generator.generate_file_if_not_exist("package.json", generate_package_json(generator.get_base_dir())).await? {
            // if exist, update package.json with a minor version
            let json_data = std::fs::read_to_string(generator.get_file_path("package.json"))
                .expect("Unable to read package.json");
            generator.generate_file("package.json", update_package_json(json_data)).await?;
        }
        Ok(())
    }

    async fn generate_main(&self, ctx: &Ctx, generator: &FileUtil) -> teo_result::Result<()> {
        generator.generate_file("index.d.ts", TsIndexDTsTemplate {
            outline: &ctx.outline,
            conf: ctx.conf,
        }.render().unwrap()).await?;
        generator.generate_file("index.js", TsIndexJsTemplate { outline: &ctx.outline, conf: ctx.conf }.render().unwrap()).await?;
        Ok(())
    }
}


