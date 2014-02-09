//
// gash.rs
//
// Starting code for PS2
// Running on Rust 0.9
//
// University of Virginia - cs4414 Spring 2014
// Weilin Xu, David Evans
// Version 0.4
//
/*
Vikram Bhasin vb8nd and Justin Ingram jci5kb
*/

extern mod extra;

use std::{io, run, os};
use std::io::buffered::BufferedReader;
use std::io::stdin;
use std::os::getcwd;
use extra::getopts;
use std::run::Process;
use std::run::ProcessOutput;
use std::run::ProcessOptions;
use std::option::{Option, None, Some};

struct Shell {
    cmd_prompt: ~str,
    hist: ~[~str],
}

impl Shell {
    fn new(prompt_str: &str) -> Shell {
        Shell {
            cmd_prompt: prompt_str.to_owned(),
            hist: ~[],
        }
    }
    
    fn run(&mut self) {
        let mut stdin = BufferedReader::new(stdin());
        
        loop {
            print(self.cmd_prompt);
            io::stdio::flush();
            
            let line = stdin.read_line().unwrap();
            let cmd_line = line.trim().to_owned();
            let program = cmd_line.splitn(' ', 1).nth(0).expect("no program");
            self.hist.push(program.to_owned());
            match program {
                ""      =>  { continue; }
                "exit"  =>  { return; }
                "cd"	=>  { self.changeDir(cmd_line); }
                "history" =>{ self.history(); }
                _       =>  { self.run_cmdline(cmd_line); }
            }
        }
    }
    
    fn run_cmdline(&mut self, cmd_line: &str) {
        let mut argv: ~[~str] =
            cmd_line.split(' ').filter_map(|x| if x != "" { Some(x.to_owned()) } else { None }).to_owned_vec();
    
        if argv.len() > 0 {
            let program: ~str = argv.remove(0);
            //Let's process the argv
            
            let endopt = argv.pop_opt();
            let end = match endopt {
		Some(stringy) => { stringy }
		None => { ~"" }
            };
            
            if end.eq(&~"&") {
		println("Start in a new process" + end);
            } else {
		println("No new process");
		if !end.eq(&~"") {
		    argv.push(end.to_owned());
		}
            }
            
            self.run_cmd(program, argv);
        }
    }
    
    fn run_cmd(&mut self, program: &str, argv: &[~str]) {
        if self.cmd_exists(program) {
	    // Old stuff
            run::process_status(program, argv);
            
            //println("Run command hit");
            //println(program);
            /*
            let mut whichprocop = ProcessOptions::new();
	    whichprocop.dir = self.cwdopt.as_ref();
	    
	    let mut whichproc = Process::new(program, argv, whichprocop);
	    let mut process = whichproc.unwrap();
	    let mut procout = process.finish_with_output();
	    println(std::str::from_utf8(procout.output));
            */
        } else {
            println!("{:s}: command not found", program);
        }
    }
    
    fn cmd_exists(&mut self, cmd_path: &str) -> bool {
        let ret = run::process_output("which", [cmd_path.to_owned()]);
        return ret.expect("exit code error.").status.success();
        /*
        println("cmd_exists hit");
        let mut whichprocop = ProcessOptions::new();
        whichprocop.dir = self.cwdopt.as_ref();
        
        let mut whichproc = Process::new("which", [cmd_path.to_owned()], whichprocop);
        let mut process = whichproc.unwrap();
	let mut procout = process.finish();
        procout.success()
        */
    }
    
    //Justin's function for cd
    fn changeDir(&mut self, cmd_line: &str) {
	let mut argv: ~[~str] =
            cmd_line.split(' ').filter_map(|x| if x != "" { Some(x.to_owned()) } else { None }).to_owned_vec();
    
        if argv.len() > 1 {
            let pstring: ~str = argv.remove(1);
            
            //println!("You want to change cwd to {:s}", pstring);
            
            let npath = Path::new(pstring);
            
            let mut cpath = getcwd();
            cpath.push(npath);
            
            if cpath.exists() {
		let mut cpathforopt = cpath.clone();
		std::os::change_dir(&cpath);
		// self.cwd = cpath;
            } else {
		println("Path does not exist!");
            }
            
        }
        
        let x = getcwd();
        
        match x.as_str() {
	    Some(path_str) => {println!("{:s}", path_str); }
	    None	=> {println("Path not representable as string!"); }
        }
    }
    
    fn history(&mut self) {
	for stringy in self.hist.iter() {
	    let x = stringy.clone();
	    println(x);
	}
    }
}

fn get_cmdline_from_args() -> Option<~str> {
    /* Begin processing program arguments and initiate the parameters. */
    let args = os::args();
    
    let opts = ~[
        getopts::optopt("c")
    ];
    
    let matches = match getopts::getopts(args.tail(), opts) {
        Ok(m) => { m }
        Err(f) => { fail!(f.to_err_msg()) }
    };
    
    if matches.opt_present("c") {
        let cmd_str = match matches.opt_str("c") {
                                                Some(cmd_str) => {cmd_str.to_owned()}, 
                                                None => {~""}
                                              };
        return Some(cmd_str);
    } else {
        return None;
    }
}

fn main() {
    let opt_cmd_line = get_cmdline_from_args();
    
    match opt_cmd_line {
        Some(cmd_line) => Shell::new("").run_cmdline(cmd_line),
        None           => Shell::new("gash > ").run()
    }
}
