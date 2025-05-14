use clap::Parser;
use rcli::{Opts,SubCommand,process_csv};
//cargo run -- csv -i /Users/alex/Desktop/r-project/rcli/assets/juventus.csv -o output.json
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
    }
    Ok(())

}