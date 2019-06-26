use drat_lib::args;
use drat_lib::dr::dr::{DRDriver, Source};
use drat_lib::dr::header_graph::{HeaderChain, HeaderGraph, HeaderNode};
use drat_lib::dr::types::Header;
use drat_lib::error::{self, CliResult};
use drat_lib::input::input_factory::InputFactory;
use drat_lib::input::file::FileInput;
use drat_lib::input::http::Http;
use drat_lib::driver::driver::Driver;
use drat_lib::write::factory;

fn main() {
    let http: Http = Http;
    let file: FileInput = FileInput;
    let vec: Vec<Box<InputFactory>> = vec![Box::new(http),
                                           Box::new(file)];
    error::handle_drive_error(simple_example(vec));
}

pub fn simple_example(input_factories: Vec<Box<InputFactory>>) -> CliResult<()> {
    let cli_args = parse_args()?;
    let args = args::Args {cli_args, input_factories};
    setup_graph(args)?.drive();
    Ok(())
}

pub fn parse_args() -> CliResult<args::CliArgs> {
    let input = "./examples/files/hundred.dc".to_string();
    let inputs = vec![input];
    Ok(args::CliArgs::new(inputs, None, None, None)?)
}

pub fn setup_graph(mut args: args::Args) -> CliResult<Box<DRDriver>> {
    // sink writer
    let output = None;

    let source_configs = args.create_configs().unwrap();
    let mut sources: Vec<Box<Source>> = Vec::with_capacity(source_configs.len());
    let mut headers: Vec<Header> = Vec::with_capacity(source_configs.len());
    for mut s in source_configs {
        let source = s.reader().unwrap();
        headers.push(source.header().clone());
        sources.push(source);
    }

    let header_sink = factory::new_header_sink(output)?;
    let node_output = HeaderNode::HeaderSink(header_sink);
    let chain = HeaderChain::new(vec![node_output]);

    let graph = HeaderGraph::new(vec![chain]);
    Ok(Box::new(
        Driver::new(sources, graph, args.cli_args.data_range, headers)?))
}