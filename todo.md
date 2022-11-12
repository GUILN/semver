# Todo

- [ ] make a `semver` rust application 
  - considering following breaking and non breaking tags:
    - feat:, fix:, refact:
    - feat!, fix!, refact!
  - consider two bins: 
    - semver: should
      - evaluate if comment is valid format, if not return error
      - return {"type": "feat", "comment": "rest of the comment"}
      - return error if not in format
    - getsemver
      - determine semantic version given current version and next type of commit 
      - should follow v<major>.<minor>.<patch>
        - major update zeroes minor and patch
        - minor update zeroes patch
- [ ] Include semver binaries into `company-log` repo as a git hook to automatically determine the version.
