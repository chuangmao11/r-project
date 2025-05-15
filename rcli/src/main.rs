use clap::Parser;
use rcli::{Opts,SubCommand,process_csv,process_genpass};
//cargo run -- csv -i /Users/alex/Desktop/r-project/rcli/assets/juventus.csv -o output.json
//cargo run -- genpass -l 16 >output.txt
//cargo run -- csv -i /Users/alex/Desktop/r-project/rcli/assets/juventus.csv --format yaml
fn main() ->anyhow::Result<()>{
    let opts =Opts::parse();
    match opts.cmd{
        SubCommand::Csv(opts) =>{
            let output = if let Some(output) =opts.output{
                output.clone()
            }else{
                format!("output.{}",opts.format)
            };
            process_csv(&opts.input,output,opts.format)?;
        }
        SubCommand::Genpass(opts) =>{
            let password = process_genpass(
                opts.length,
                opts.uppercase,
                opts.lowercase,
                opts.number,
                opts.symbol)?;
            println!("{}", password);
        }
    }
    Ok(())

}