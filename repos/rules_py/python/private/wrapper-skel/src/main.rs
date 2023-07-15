use pyembed::{
    MainPythonInterpreter, OxidizedPythonInterpreterConfig, PythonInterpreterConfig,
    PythonInterpreterProfile,
};

fn main() {
    // The following code is in a block so the MainPythonInterpreter is destroyed in an
    // orderly manner, before process exit.
    let exit_code = {
        // Load the Python configuration
        let config = OxidizedPythonInterpreterConfig {
            interpreter_config: PythonInterpreterConfig {
                // TODO: Do we want to set this to isolated? Note this is different
                //       than -I.
                //         See: https://docs.python.org/3/c-api/init_config.html#isolated-configuration
                profile: PythonInterpreterProfile::Python,
                // Equivilant to the -I flag
                isolated: Some(true),
                // We want to explicitly control our sys.path, not let Python make
                // it's own decisions about what it should be.
                module_search_paths: Some(vec![]),
                // I think we don't actually need the site module, since we've
                // already correctly setup the Python interpreter. Not importing it
                // should also speed things up.
                site_import: Some(false),
                // We don't write bytecode, we don't actually need it and it clogs
                // up our build directories if we did have it.
                // TODO: Maybe this should only be true if we're embedding our
                //       modules?
                write_bytecode: Some(false),

                ..Default::default()
            },

            // We bundle at least the standard library (if not the entire app)
            // using OxidizedImporter, so we'll need to turn it on.
            // oxidized_importer: true,
            ..Default::default()
        };

        // Construct a new Python interpreter using that config, handling any errors
        // from construction.
        match MainPythonInterpreter::new(config) {
            Ok(interp) => {
                // And run it using the default run configuration as specified by the
                // configuration.
                //
                // This will either call `interp.py_runmain()` or
                // `interp.run_multiprocessing()`. If `interp.py_runmain()` is called,
                // the interpreter is guaranteed to be finalized.
                interp.run()
            }
            Err(msg) => {
                eprintln!("error instantiating embedded Python interpreter: {}", msg);
                1
            }
        }
    };

    // And exit the process according to code execution results.
    std::process::exit(exit_code);
}
