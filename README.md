# rust-docker-sample

Sample project for rust with...

- Docker(+ VSCode) develop environment
- GitHub Actions CI/CD

## Notes

To release a crate via GitHub Actions, you need to setup GitHub secrets.

- Secrets:
  - Key: CRATES_IO_TOKEN
  - Value: API Access token for `crates.io`

After setup, create git tag which name is like `v.*`, and push that tag to remote.
