use clap::{Parser, Subcommand, CommandFactory};
use clap_complete::{generate, Shell};
use colored::*;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "alaz")]
#[command(author = "xjz")]
#[command(version)]
#[command(about = "Assembly Language Analyzer - AArch64 汇编语言分析工具")]
#[command(long_about = "\
Assembly Language Analyzer (ALAZ) - AArch64 汇编语言分析工具

功能特性:
  • 解析 objdump 输出文件
  • 自动生成汇编指令的语义解释
  • 支持多优化级别对比分析 (O0/O1/O2)
  • 交互式函数选择和分析
  • 生成美观的 Markdown 分析报告

使用示例:
  # 交互式分析三个优化级别的共同函数
  alaz interactive spark_matrix_naive
  
  # 单文件模式分析 (只分析一个 dump 文件)
  alaz interactive -s spark_matrix_naive_O2.dump
  
  # 直接分析指定函数
  alaz analyze Matrix_add spark_matrix_naive -o ./output
  
  # 生成 shell 补全脚本
  alaz completions bash > ~/.local/share/bash-completion/completions/alaz
")]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// 启用详细日志输出
    #[arg(short, long, global = true)]
    verbose: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// 分析指定函数并生成对比表格
    /// 
    /// 分析三个优化级别 (O0/O1/O2) 的汇编代码差异。
    /// 需要存在对应的 <PREFIX>_O0.dump, <PREFIX>_O1.dump, <PREFIX>_O2.dump 文件。
    /// 
    /// 示例:
    ///   alaz analyze Matrix_add spark_matrix_naive
    ///   alaz analyze Matrix_mul my_code -o ./reports
    #[command(verbatim_doc_comment)]
    Analyze {
        /// 要分析的函数名称
        #[arg(value_name = "FUNCTION", help = "函数名称 (如: Matrix_add, main)")]
        function: String,

        /// dump 文件前缀
        #[arg(value_name = "PREFIX", help = "文件前缀 (如: spark_matrix_naive 会查找 *_O0.dump, *_O1.dump, *_O2.dump)")]
        prefix: String,

        /// 输出目录 (默认为当前目录)
        #[arg(short, long, value_name = "DIR", help = "保存分析报告的目录")]
        output: Option<PathBuf>,
    },
    
    /// 交互式模式 - 浏览和选择函数进行分析
    /// 
    /// 提供交互式菜单，显示所有可用函数供选择分析。
    /// 支持两种模式:
    ///   • 多文件模式 (默认): 分析 O0/O1/O2 三个优化级别的共同函数
    ///   • 单文件模式 (-s): 只分析指定的一个 dump 文件
    /// 
    /// 示例:
    ///   alaz interactive spark_matrix_naive          # 多文件模式
    ///   alaz interactive -s my_code_O2.dump          # 单文件模式
    ///   alaz interactive -m spark_matrix_naive -o ./reports
    #[command(verbatim_doc_comment)]
    Interactive {
        /// dump 文件前缀或完整文件名
        #[arg(
            value_name = "PREFIX_OR_FILE",
            help = "文件前缀 (多文件模式) 或完整文件名 (单文件模式)"
        )]
        prefix: String,

        /// 单文件模式 - 只分析指定的单个 dump 文件
        #[arg(
            short = 's',
            long,
            help = "单文件模式: 只读取并分析一个 dump 文件 (如: -s my_code_O2.dump)"
        )]
        single: bool,

        /// 多文件模式 - 分析三个优化级别的共同函数 (默认)
        #[arg(
            short = 'm',
            long,
            conflicts_with = "single",
            help = "多文件模式: 分析 O0/O1/O2 三个文件的共同函数 (默认行为)"
        )]
        multi: bool,

        /// 输出目录 (默认为当前目录)
        #[arg(short, long, value_name = "DIR", help = "保存分析报告的目录")]
        output: Option<PathBuf>,
    },
    
    /// 生成 shell 补全脚本
    /// 
    /// 为指定的 shell 生成自动补全脚本。
    /// 
    /// 支持的 shell: bash, fish, zsh, powershell, elvish
    /// 
    /// 安装示例:
    ///   # Bash
    ///   alaz completions bash > ~/.local/share/bash-completion/completions/alaz
    ///   
    ///   # Fish
    ///   alaz completions fish > ~/.config/fish/completions/alaz.fish
    ///   
    ///   # Zsh
    ///   alaz completions zsh > ~/.zsh/completion/_alaz
    #[command(verbatim_doc_comment)]
    Completions {
        /// Shell 类型
        #[arg(
            value_name = "SHELL",
            help = "Shell 类型 (bash, fish, zsh, powershell, elvish)"
        )]
        shell: String,
    },
}

fn main() {
    let cli = Cli::parse();

    // 配置日志
    let log_level = if cli.verbose { "info" } else { "warn" };
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or(log_level))
        .init();

    // 执行命令
    let result = match cli.command {
        Commands::Analyze { function, prefix, output } => {
            analyze_dumps(&function, &prefix, output.as_ref())
        }
        Commands::Interactive { prefix, single, multi: _, output } => {
            interactive_mode(&prefix, single, output.as_ref())
        }
        Commands::Completions { shell } => {
            generate_completions(&shell)
        }
    };

    if let Err(e) = result {
        eprintln!("{}", format!("❌ 错误: {}", e).red().bold());
        std::process::exit(1);
    }
}

/// 分析 objdump 文件并生成对比表格
fn analyze_dumps(
    function: &str,
    prefix: &str,
    output: Option<&PathBuf>,
) -> anyhow::Result<()> {
    use alaz::table::TableGenerator;

    println!("{}", "=".repeat(60).cyan());
    println!("{}", "  ALAZ - 汇编语言分析工具".cyan().bold());
    println!("{}", "=".repeat(60).cyan());
    println!();

    println!("{} {}", "📋 分析函数:".yellow(), function.bold());
    println!("{} {}", "📁 文件前缀:".yellow(), prefix);
    if let Some(out) = output {
        println!("{} {}", "💾 输出目录:".yellow(), out.display());
    }
    println!();

    let generator = TableGenerator::new();
    generator.generate_from_dumps(function, prefix, output)?;

    println!();
    println!("{}", "✅ 分析完成！".green().bold());
    Ok(())
}

/// 交互式菜单模式
fn interactive_mode(prefix: &str, single_mode: bool, output: Option<&PathBuf>) -> anyhow::Result<()> {
    use alaz::objdump::ObjdumpParser;
    use std::io::{self, Write};

    println!("{}", "=".repeat(60).cyan());
    println!("{}", "  ALAZ - 汇编语言分析工具 (交互式模式)".cyan().bold());
    println!("{}", "=".repeat(60).cyan());
    println!();

    if single_mode {
        // 单文件模式：只读取指定的文件
        let dump_path = if prefix.ends_with(".dump") {
            prefix.to_string()
        } else {
            format!("{}.dump", prefix)
        };
        
        println!("{} {} (单文件模式)", "📂 正在读取:".yellow(), dump_path);
        
        let parser = ObjdumpParser::from_file(&dump_path)?;
        let mut functions = parser.list_functions()?;
        
        if functions.is_empty() {
            println!("{}", "❌ 未找到任何函数".red());
            return Ok(());
        }
        
        functions.sort();
        println!();
        println!("{} {} 个函数", "✓ 检测到".green(), functions.len());
        println!();
        
        // 单文件模式下的交互循环
        loop {
            println!("{}", "=".repeat(60).cyan());
            println!("{}", "可用函数列表:".yellow().bold());
            println!("{}", "-".repeat(60));
            
            for (idx, func) in functions.iter().enumerate() {
                println!("  {}. {}", format!("{:3}", idx + 1).cyan(), func);
            }
            
            println!("{}", "-".repeat(60));
            println!();
            println!("请选择:");
            println!("  {} 输入函数编号进行分析", "●".green());
            println!("  {} 输入 'q' 或 'quit' 退出", "●".red());
            println!();

            print!("{} ", "选择 >".bright_blue().bold());
            io::stdout().flush()?;

            let mut input = String::new();
            io::stdin().read_line(&mut input)?;
            let input = input.trim();

            // 处理退出
            if input == "q" || input == "quit" || input.is_empty() {
                println!();
                println!("{}", "👋 再见！".yellow());
                break;
            }

            // 处理选择
            match input.parse::<usize>() {
                Ok(num) if num > 0 && num <= functions.len() => {
                    let function = &functions[num - 1];
                    println!();
                    println!("{}", "=".repeat(60).cyan());
                    
                    use alaz::table::TableGenerator;
                    let generator = TableGenerator::new();
                    
                    if let Err(e) = generator.generate_from_single_dump(function, &dump_path, output) {
                        println!();
                        println!("{} {}", "❌ 分析失败:".red(), e);
                    }
                    
                    println!();
                    println!("按 Enter 继续...");
                    let mut _pause = String::new();
                    io::stdin().read_line(&mut _pause)?;
                    println!();
                }
                _ => {
                    println!("{}", "❌ 无效的选择，请输入正确的编号".red());
                    println!();
                }
            }
        }
        
        return Ok(());
    }

    // 多文件模式：读取三个优化级别的共同函数
    // 智能处理文件路径和提取真实前缀
    let real_prefix = if prefix.ends_with(".dump") {
        // 如果输入的是完整文件名，需要提取前缀
        // 例如: spark_matrix_naive_O2.dump -> spark_matrix_naive
        prefix
            .strip_suffix(".dump").unwrap_or(prefix)
            .trim_end_matches("_O0")
            .trim_end_matches("_O1")
            .trim_end_matches("_O2")
            .to_string()
    } else {
        prefix.to_string()
    };
    
    // 读取所有三个优化级别的文件，找出共同的函数
    let o0_path = format!("{}_O0.dump", &real_prefix);
    let o1_path = format!("{}_O1.dump", &real_prefix);
    let o2_path = format!("{}_O2.dump", &real_prefix);
    
    println!("{} 读取三个优化级别的文件以找出共同函数...", "⚙".yellow());
    
    let mut common_functions: Option<std::collections::HashSet<String>> = None;
    let mut file_count = 0;
    
    for (level, path) in [("O0", &o0_path), ("O1", &o1_path), ("O2", &o2_path)] {
        if let Ok(parser) = ObjdumpParser::from_file(path) {
            if let Ok(funcs) = parser.list_functions() {
                file_count += 1;
                let func_set: std::collections::HashSet<_> = funcs.into_iter().collect();
                common_functions = Some(match common_functions {
                    None => func_set,
                    Some(existing) => existing.intersection(&func_set).cloned().collect(),
                });
                println!("  {} {} 文件读取成功", "✓".green(), level);
            } else {
                println!("  {} {} 文件解析失败", "⚠".yellow(), level);
            }
        } else {
            println!("  {} {} 文件未找到", "⚠".yellow(), level);
        }
    }
    
    let mut functions: Vec<String> = common_functions
        .unwrap_or_default()
        .into_iter()
        .collect();
    
    if functions.is_empty() {
        println!("{}", "❌ 未找到任何共同函数".red());
        if file_count == 0 {
            println!("{}", "提示: 请确保存在 *_O0.dump, *_O1.dump, *_O2.dump 文件".yellow());
        }
        return Ok(());
    }
    
    functions.sort();
    
    println!();
    println!("{} {} 个共同函数 (在所有优化级别都存在)", "✓ 检测到".green(), functions.len());
    println!();

    loop {
        // 显示函数列表
        println!("{}", "=".repeat(60).cyan());
        println!("{}", "可用函数列表:".yellow().bold());
        println!("{}", "-".repeat(60));
        
        for (idx, func) in functions.iter().enumerate() {
            println!("  {}. {}", format!("{:3}", idx + 1).cyan(), func);
        }
        
        println!("{}", "-".repeat(60));
        println!();
        println!("请选择:");
        println!("  {} 输入函数编号进行分析", "●".green());
        println!("  {} 输入 'q' 或 'quit' 退出", "●".red());
        println!();

        print!("{} ", "选择 >".bright_blue().bold());
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let input = input.trim();

        // 处理退出
        if input == "q" || input == "quit" || input.is_empty() {
            println!();
            println!("{}", "👋 再见！".yellow());
            break;
        }

        // 处理选择
        match input.parse::<usize>() {
            Ok(num) if num > 0 && num <= functions.len() => {
                let function = &functions[num - 1];
                println!();
                println!("{}", "=".repeat(60).cyan());
                
                if let Err(e) = analyze_dumps(function, &real_prefix, output) {
                    println!();
                    println!("{} {}", "❌ 分析失败:".red(), e);
                }
                
                println!();
                println!("按 Enter 继续...");
                let mut _pause = String::new();
                io::stdin().read_line(&mut _pause)?;
                println!();
            }
            _ => {
                println!("{}", "❌ 无效的选择，请输入正确的编号".red());
                println!();
            }
        }
    }

    Ok(())
}

/// 生成 shell 补全脚本
fn generate_completions(shell_name: &str) -> anyhow::Result<()> {
    let shell = match shell_name.to_lowercase().as_str() {
        "bash" => Shell::Bash,
        "fish" => Shell::Fish,
        "zsh" => Shell::Zsh,
        "powershell" => Shell::PowerShell,
        "elvish" => Shell::Elvish,
        _ => {
            eprintln!("{}", format!("❌ 不支持的 shell: {}", shell_name).red());
            eprintln!("支持的 shell: bash, fish, zsh, powershell, elvish");
            return Ok(());
        }
    };

    let mut cmd = Cli::command();
    let bin_name = cmd.get_name().to_string();
    
    // 只输出补全脚本，不输出任何其他信息
    generate(shell, &mut cmd, bin_name, &mut std::io::stdout());
    
    Ok(())
}
