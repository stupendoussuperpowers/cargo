use cargo_test_support::compare::assert_ui;
use cargo_test_support::prelude::*;
use cargo_test_support::Project;

use cargo_test_support::curr_dir;

#[cargo_test]
fn case() {
    cargo_test_support::registry::init();

    for name in ["pack1", "pack2"] {
        cargo_test_support::registry::Package::new(name, "v0.1")
            .feature(&format!("feat-{}", name)[..], &[])
            .publish();
    }

    let project = Project::from_template(curr_dir!().join("in"));
    let project_root = project.root();
    let cwd = &project_root;
    let git_dep = cargo_test_support::git::new("git-package", |project| {
        project
            .file(
                "p1/Cargo.toml",
                &cargo_test_support::basic_manifest("pack1", "v0.1"),
            )
            .file("p1/src/lib.rs", "")
            .file(
                "p2/Cargo.toml",
                &cargo_test_support::basic_manifest("pack2", "v0.1"),
            )
            .file("p2/src/lib.rs", "")
    });
    let git_url = git_dep.url().to_string();

    snapbox::cmd::Command::cargo_ui()
        .arg("add")
        .args(["pack1", "--git", &git_url, "--features=feat-pack1"])
        .current_dir(cwd)
        .assert()
        .success()
        .stdout_matches_path(curr_dir!().join("stdout.log"))
        .stderr_matches_path(curr_dir!().join("stderr.log"));

    assert_ui().subset_matches(curr_dir!().join("out"), &project_root);
}
