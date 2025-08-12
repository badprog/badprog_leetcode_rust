# badprog_leetcode_rust

Challenges from leetcode.com in Rust.  

The project files enable the management of VSCode Dev Containers, GitHub Actions, PR (GitHub Pull Requests), and the generation of the Docker image on the GHCR (GitHub Container Registry).  
Each push must be done on a different branch than main.  
Then a PR has to be created and validated (after the tests pass).  
When the PR is validated, the push on main branch is generated and the Docker image as well on the GHCR.  

Version of each tool:
- **Docker image**: rust:1.88
- **rustc** 1.88.0 (6b00bc388 2025-06-23)
- **rustfmt** 1.8.0-stable (6b00bc3880 2025-06-23)
- **clippy** 0.1.88 (6b00bc3880 2025-06-23)

Enjoy! 😎
