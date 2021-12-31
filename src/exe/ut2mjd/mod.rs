use super::{execcode, main_convertion, Arguments, Converter, EnvValues, Parameters};
use std::ffi::OsString;
use std::io::{BufRead, Write};
mod converter;
use converter::Ut2MjdConverter;

pub fn main_inner(
    args: impl IntoIterator<Item = impl Into<OsString> + Clone>,
    env_vars: impl IntoIterator<Item = (impl ToString, impl ToString)>,
    stdin: &mut impl BufRead,
    stdout: &mut impl Write,
    stderr: &mut impl Write,
) -> i32 {
    let args = Arguments::new("Converter from UT to MJD", args);
    let env_vars = EnvValues::new(env_vars);

    // Analyze the arguments and the environment variables.
    let params = Parameters::new(&args, &env_vars);

    let converter = Ut2MjdConverter::new(params.get_dt_fmt());

    let result = main_convertion(&converter, &params, stdin, stdout, stderr);
    return execcode::execcode(&result);
}

// TODO: test of main_inner
