use crate::{
    config::{self, Configuration, NotionConfig},
    model::movie::MovieQuery,
    model::notion::Data,
    network, utils,
};
use clap::{Args, Parser, Subcommand};
use std::env;
use std::io::{self, Write};

#[derive(Parser)]
#[command(name = "db2n")]
struct Cli {
    #[arg(long, hide = true)]
    uninstall: bool,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Config(ConfigArgs),
    Crawl { target: String },
    Query { keyword: String },
}

#[derive(Args)]
struct ConfigArgs {
    #[command(subcommand)]
    command: ConfigCommands,
}

#[derive(Subcommand)]
enum ConfigCommands {
    List,
    Create {
        name: String,
        database_id: String,
        token: String,
    },
    Set {
        name: String,
    },
}

fn is_cli_invocation(args: &[String]) -> bool {
    if args.len() <= 1 {
        return false;
    }else{
        return true;
    }
}

fn run_uninstall() -> Result<(), String> {
    if cfg!(debug_assertions) {
        return Err("PATH 功能在开发模式下不可用".to_string());
    }

    let exe_path = env::current_exe().map_err(|e| e.to_string())?;
    let install_dir = exe_path
        .parent()
        .ok_or_else(|| "Failed to get install directory".to_string())?
        .to_path_buf();

    let install_dir_str = install_dir.to_string_lossy().to_string();
    utils::remove_from_path(&install_dir_str)?;

    let data_dir = install_dir.join("data");
    match std::fs::remove_dir_all(&data_dir) {
        Ok(()) => {}
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {}
        Err(e) => return Err(e.to_string()),
    }

    println!("ok");
    Ok(())
}

async fn run_config(args: ConfigArgs) -> Result<(), String> {
    match args.command {
        ConfigCommands::List => {
            let (options, index) = Configuration::load_options().map_err(|e| e.to_string())?;
            if options.is_empty() {
                return Err(format!(
                        "未找到配置文件，请先创建：db2n config create <name> <database_id> <token>\n配置目录：{}",
                        config::config_dir_path().display()
                    ));
            }
            for o in options {
                if o == index {
                    println!("{o} *");
                } else {
                    println!("{o}");
                }
            }
            Ok(())
        }
        ConfigCommands::Create {
            name,
            database_id,
            token,
        } => {
            let cfg = NotionConfig { database_id, token };
            config::create_config(&name, &cfg).map_err(|e| e.to_string())?;
            Configuration::global()
                .set(&name)
                .map_err(|e| e.to_string())?;
            println!("ok");
            Ok(())
        }
        ConfigCommands::Set { name } => {
            Configuration::global()
                .set(&name)
                .map_err(|e| e.to_string())?;
            println!("ok");
            Ok(())
        }
    }
}

fn require_notion_config() -> Result<NotionConfig, String> {
    let (options, _) = Configuration::load_options().map_err(|e| e.to_string())?;
    if options.is_empty() {
        return Err(format!(
                "未找到配置文件，请先创建：db2n config create <name> <database_id> <token>\n配置目录：{}",
                config::config_dir_path().display()
            ));
    }
    let notion = Configuration::global().get_notion();
    if notion.database_id.trim().is_empty() || notion.token.trim().is_empty() {
        return Err(
            "当前配置不完整，请使用 db2n config create 或 db2n config set 选择有效配置".to_string(),
        );
    }
    Ok(notion)
}

async fn run_crawl(target: &str) -> Result<(), String> {
    let notion = require_notion_config()?;
    let movie = network::crawl::crawl_douban_movie(target).await?;
    let data = Data::from_douban_movie(&movie, &notion.database_id);
    network::notion::post_to_notion(&data)
        .await
        .map_err(|e| e.to_string())?;
    println!("{}", movie.name);
    Ok(())
}

async fn run_query(keyword: &str) -> Result<(), String> {
    let items = network::crawl::crawl_movie_query(keyword).await?;
    if items.is_empty() {
        println!("未查询到结果");
        return Ok(());
    }

    print_movie_query_table(&items);
    let Some(index) = prompt_select_index(items.len())? else {
        return Ok(());
    };
    let selected = items
        .get(index)
        .ok_or_else(|| "选择的序号超出范围".to_string())?;
    run_crawl(&selected.id).await?;
    Ok(())
}

fn print_movie_query_table(items: &[MovieQuery]) {
    let mut title_width = "标题".chars().count();
    let mut year_width = "年份".chars().count();
    let mut type_width = "类型".chars().count();
    let mut subtitle_width = "副标题".chars().count();

    for item in items {
        title_width = title_width.max(item.title.chars().count());
        year_width = year_width.max(item.year.chars().count());
        type_width = type_width.max(item.typ.chars().count());
        subtitle_width = subtitle_width.max(item.sub_title.chars().count());
    }

    title_width = title_width.min(30);
    year_width = year_width.min(6);
    type_width = type_width.min(10);
    subtitle_width = subtitle_width.min(24);

    let index_width = items.len().to_string().chars().count().max("序号".chars().count());

    let sep = format!(
        "+-{:-<index_width$}-+-{:-<title_width$}-+-{:-<year_width$}-+-{:-<type_width$}-+-{:-<subtitle_width$}-+",
        "",
        "",
        "",
        "",
        "",
        index_width = index_width,
        title_width = title_width,
        year_width = year_width,
        type_width = type_width,
        subtitle_width = subtitle_width
    );

    println!("{sep}");
    println!(
        "| {} | {} | {} | {} | {} |",
        pad_right("序号", index_width),
        pad_right("标题", title_width),
        pad_right("年份", year_width),
        pad_right("类型", type_width),
        pad_right("副标题", subtitle_width),
    );
    println!("{sep}");

    for (i, item) in items.iter().enumerate() {
        let row_index = (i + 1).to_string();
        println!(
            "| {} | {} | {} | {} | {} |",
            pad_right(&row_index, index_width),
            pad_right(&truncate(&item.title, title_width), title_width),
            pad_right(&truncate(&item.year, year_width), year_width),
            pad_right(&truncate(&item.typ, type_width), type_width),
            pad_right(&truncate(&item.sub_title, subtitle_width), subtitle_width),
        );
    }

    println!("{sep}");
}

fn prompt_select_index(len: usize) -> Result<Option<usize>, String> {
    loop {
        print!("请选择要导入的序号 (1-{len}，回车取消)：");
        io::stdout().flush().map_err(|e| e.to_string())?;

        let mut input = String::new();
        io::stdin().read_line(&mut input).map_err(|e| e.to_string())?;
        let input = input.trim();

        if input.is_empty() || input.eq_ignore_ascii_case("q") || input.eq_ignore_ascii_case("quit")
        {
            return Ok(None);
        }

        let Ok(n) = input.parse::<usize>() else {
            println!("请输入有效数字");
            continue;
        };

        if n == 0 {
            return Ok(None);
        }

        if n <= len {
            return Ok(Some(n - 1));
        }

        println!("序号超出范围，请输入 1-{len}");
    }
}

fn truncate(input: &str, max_width: usize) -> String {
    let count = input.chars().count();
    if count <= max_width {
        return input.to_string();
    }
    if max_width <= 1 {
        return "…".to_string();
    }
    let mut out: String = input.chars().take(max_width - 1).collect();
    out.push('…');
    out
}

fn pad_right(input: &str, width: usize) -> String {
    let count = input.chars().count();
    if count >= width {
        return input.to_string();
    }
    let mut out = String::with_capacity(width + 2);
    out.push_str(input);
    out.push_str(&" ".repeat(width - count));
    out
}

pub async fn run() -> bool {
    let args: Vec<String> = env::args().collect();
    if is_cli_invocation(&args) {
        let cli = Cli::parse();
        if cli.uninstall {
            if let Err(msg) = run_uninstall() {
                eprintln!("{msg}");
            }
        }

        let result = match cli.command {
            Some(Commands::Config(args)) => run_config(args).await,
            Some(Commands::Crawl { target }) => run_crawl(&target).await,
            Some(Commands::Query { keyword }) => run_query(&keyword).await,
            None => Ok(()),
        };
        if let Err(msg) = result {
            eprintln!("{msg}");
            std::process::exit(1);
        }
        return true;
    }
    false
}
