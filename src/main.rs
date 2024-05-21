use orchestrator::LanguageShellOrchestrator;

pub mod orchestrator;

fn main() {
    let orch = orchestrator::CSharpOrchestrator::new();

    loop {
        let mut cmd = String::new();
        let b1 = std::io::stdin().read_line(&mut cmd).unwrap();
        orch.run(cmd);
    }
}
