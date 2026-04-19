//! Docker-exec process spawner.
//!
//! Used by container authorities. Plugins build this via the
//! `editor.setAuthority({ spawner: { kind: "docker-exec", … } })` op
//! after they have brought a container up. Core never names "docker"
//! anywhere outside this file — the spawner is just one more
//! `dyn ProcessSpawner` implementation as far as the rest of the editor
//! is concerned.

use std::process::Stdio;

use async_trait::async_trait;
use tokio::process::Command;

use crate::services::remote::{ProcessSpawner, SpawnError, SpawnResult};

/// Spawn processes inside a long-lived Docker container via `docker exec`.
pub(crate) struct DockerExecSpawner {
    container_id: String,
    user: Option<String>,
    workspace: Option<String>,
}

impl DockerExecSpawner {
    pub(crate) fn new(
        container_id: String,
        user: Option<String>,
        workspace: Option<String>,
    ) -> Self {
        Self {
            container_id,
            user,
            workspace,
        }
    }
}

#[async_trait]
impl ProcessSpawner for DockerExecSpawner {
    async fn spawn(
        &self,
        command: String,
        args: Vec<String>,
        cwd: Option<String>,
    ) -> Result<SpawnResult, SpawnError> {
        let mut docker_args: Vec<String> = vec!["exec".into()];

        if let Some(ref user) = self.user {
            docker_args.push("-u".into());
            docker_args.push(user.clone());
        }

        // Per-call cwd wins over the authority's default workspace.
        if let Some(dir) = cwd.or_else(|| self.workspace.clone()) {
            docker_args.push("-w".into());
            docker_args.push(dir);
        }

        docker_args.push(self.container_id.clone());
        docker_args.push(command);
        docker_args.extend(args);

        let output = Command::new("docker")
            .args(&docker_args)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .await
            .map_err(|e| SpawnError::Process(e.to_string()))?;

        Ok(SpawnResult {
            stdout: String::from_utf8_lossy(&output.stdout).to_string(),
            stderr: String::from_utf8_lossy(&output.stderr).to_string(),
            exit_code: output.status.code().unwrap_or(-1),
        })
    }
}
