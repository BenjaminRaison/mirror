use swayipc::{Connection,Fallible,EventType,WindowChange,Event,Output};
use std::process::Command;
use sysinfo::{ProcessExt,SystemExt};


const MIRROR_APPID: &str = "at.yrlf.wl_mirror";
const MIRROR_WORKSPACE: &str = "mirror";
const PREF_SOURCE: &str = "eDP-1";
const PREF_TARGET: &str = "HDMI-1";

fn main() -> Fallible<()> {
    let mut con = Connection::new().unwrap();

    {
        if kill_active_mirrors() {
            return Ok(());
        }
    }

    let outputs = con.get_outputs();


    let source: String;
    let target: String;
    {
        let (src,tar) = target_outputs(outputs?);
        source = src.unwrap();
        target = tar.unwrap();
    }

    let workspaces = con.get_workspaces().unwrap();
    let currentWorkspace = workspaces.iter().enumerate().find(|x| x.1.focused).unwrap().1;


    // start wl-mirror, then wait until it is visible
    Command::new("wl-mirror").arg(source).spawn().expect("failed to start wl-mirror");
    for event in Connection::new()?.subscribe([EventType::Window])? {
        match event? {
            Event::Window(w) => {
                if w.change == WindowChange::New && w.container.app_id.as_deref().unwrap_or("false") == MIRROR_APPID {
                    break;
                }
            },
            _ => unreachable!(),
        }
    }

    con.run_command("[app_id=".to_owned() + MIRROR_APPID + "] move to workspace " + MIRROR_WORKSPACE + ",fullscreen enable").expect("failed to move wl-mirror to workspace");
    con.run_command("[workspace=".to_owned() + MIRROR_WORKSPACE + "] move workspace to " + target.as_str()).expect("failed to move workspace to output");
    con.run_command("[workspace=".to_owned() + currentWorkspace.name.as_str() + "] focus").expect("failed to focus workspace");

    Ok(())
}

fn kill_active_mirrors() -> bool {
    let s = sysinfo::System::new_all();
    for (_,process) in s.processes() {
        if process.name() == "wl-mirror" {
            process.kill();
            return true
        }
    }
    return false
}

fn target_outputs(outputs: Vec<Output>) -> (Option<String>, Option<String>) {
    let mut source: Option<String> = None;
    let mut target: Option<String> = None;

     if outputs.len() >= 2 {
        for output in outputs {
            if output.name == PREF_SOURCE {
                source = Some(output.name);
            } else if output.name == PREF_TARGET {
                target = Some(output.name);
            } else if output.focused {
                match source {
                    None => source = Some(output.name),
                    Some(_) => {}
                }
            } else {
                match target {
                    None => target = Some(output.name),
                    Some(_) => {}
                }
            }
        }
    }
    return (source, target);
}
